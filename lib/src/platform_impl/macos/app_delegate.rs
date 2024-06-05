use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    fmt::Debug,
    mem,
    rc::Weak,
};

use objc2::{
    declare_class,
    msg_send_id,
    mutability,
    rc::{autoreleasepool, Retained},
    ClassType,
    DeclaredClass,
};
use objc2_app_kit::{NSApp, NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol};

use super::panicinfo::PanicInfo;
use crate::{
    platform::Wrapper,
    ActiveApplication,
    Event,
    EventHandler,
    Icon,
    LifeCycle,
    Menu,
    WindowEvent,
    WindowId,
};

#[derive(Debug)]
pub(super) struct ActivationPolicy(NSApplicationActivationPolicy);

impl Default for ActivationPolicy {
    fn default() -> Self { Self(NSApplicationActivationPolicy::Regular) }
}

pub(super) struct State {
    app: RefCell<Option<ActiveApplication>>,
    activation_policy: ActivationPolicy,
    activate_ignoring_other_apps: bool,
    is_running: Cell<bool>,
    handler: RefCell<Option<Box<dyn EventHandler>>>,
    pending_events: RefCell<VecDeque<Event>>,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("activation_policy", &self.activation_policy)
            .field(
                "activate_ignoring_other_apps",
                &self.activate_ignoring_other_apps,
            )
            .field("is_running", &self.is_running)
            .field("pending_events", &self.pending_events)
            .finish()
    }
}

declare_class!(
    #[derive(Debug)]
    pub(super) struct AppDelegate;

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - Main thread only mutability is correct, since this is an application delegate.
    // - `AppDelegate` does not implement `Drop`.
    unsafe impl ClassType for AppDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "CocoaAppDelegate";
    }

    impl DeclaredClass for AppDelegate {
        type Ivars = State;
    }

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[method(applicationDidFinishLaunching:)]
        fn did_finish_launching(&self, _notification: &NSNotification) {
            let mtm = MainThreadMarker::from(self);
            let app = NSApp(mtm);
            // We need to delay setting the activation policy and activating the app
            // until `applicationDidFinishLaunching` has been called. Otherwise the
            // menu bar is initially unresponsive on macOS 10.15.
            app.setActivationPolicy(self.ivars().activation_policy.0);

            #[allow(deprecated)]
            app.activateIgnoringOtherApps(self.ivars().activate_ignoring_other_apps);

            self.handle_event(Event::LifeCycle(LifeCycle::Started));
            self.set_is_running(true);
        }

        #[method(applicationWillTerminate:)]
        fn will_terminate(&self, _notification: &NSNotification) {
            self.handle_event(Event::LifeCycle(LifeCycle::Finished));
            self.set_is_running(false);
        }
    }
);

impl AppDelegate {
    pub(super) fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(State {
            app: RefCell::new(None),
            activate_ignoring_other_apps: true,
            activation_policy: Default::default(),
            is_running: Cell::new(false),
            handler: RefCell::new(None),
            pending_events: RefCell::new(VecDeque::new()),
        });
        unsafe { msg_send_id![super(this), init] }
    }

    pub(super) fn get(mtm: MainThreadMarker) -> Retained<Self> {
        let app = NSApp(mtm);
        let delegate =
            unsafe { app.delegate() }.expect("a delegate was not configured on the application");
        if delegate.is_kind_of::<Self>() {
            // SAFETY: Just checked that the delegate is an instance of `ApplicationDelegate`
            unsafe { Retained::cast(delegate) }
        } else {
            panic!("tried to get a delegate that was not the one Winit has registered")
        }
    }

    #[inline]
    pub(super) fn set_active_application(&self, active_application: ActiveApplication) {
        *self.ivars().app.borrow_mut() = Some(active_application);
    }

    #[inline]
    pub(super) fn set_handler(&self, handler: impl EventHandler + 'static) {
        *self.ivars().handler.borrow_mut() = Some(Box::new(handler));
    }

    #[inline]
    pub(super) fn is_running(&self) -> bool { self.ivars().is_running.get() }

    #[inline]
    pub(super) fn set_is_running(&self, value: bool) { self.ivars().is_running.set(value) }

    pub(super) fn wakeup(&self, _panic_info: Weak<PanicInfo>) {}

    pub(super) fn cleared(&self, panic_info: Weak<PanicInfo>) {
        let panic_info = panic_info
            .upgrade()
            .expect("The panic info must exist here. This failure indicates a developer error.");

        // Return when in event handler due to https://github.com/rust-windowing/winit/issues/1779
        // XXX: how does it make sense that `event_handler.ready()` can ever return `false` here if
        // we're about to return to the `CFRunLoop` to poll for new events?
        if panic_info.is_panicking() || !self.is_running() {
            return;
        }

        let events = mem::take(&mut *self.ivars().pending_events.borrow_mut());
        for event in events.into_iter() {
            self.handle_event(event);
        }
    }

    #[inline]
    pub(super) fn queue_event(&self, event: Event) {
        self.ivars().pending_events.borrow_mut().push_back(event);
    }

    pub(super) fn handle_event(&self, event: Event) {
        let mut app = self.ivars().app.borrow_mut();

        if let Some(app) = app.as_mut() {
            let mut handler = self.ivars().handler.borrow_mut();

            if let Some(handler) = handler.as_mut() {
                handler.on_event(app, event);
            }
        }
    }

    pub(super) fn handle_redraw(&self, window_id: WindowId) {
        // Redraw request might come out of order from the OS.
        // -> Don't go back into the event handler when our callstack originates from there.
        self.handle_event(Event::Window(WindowEvent::RedrawRequested, window_id));
    }

    #[inline]
    pub(super) fn set_menu(&self, menu: Option<&Menu>) {
        let mtm = MainThreadMarker::from(self);
        let app = NSApp(mtm);
        if let Some(menu) = menu {
            app.setMainMenu(Some(&menu.get_impl().get_native(mtm)));
        } else {
            app.setMainMenu(None);
        }
    }

    #[inline]
    pub(super) fn set_icon(&self, icon: Option<&Icon>) {
        autoreleasepool(|_| {
            let mtm = MainThreadMarker::from(self);
            let app = NSApp(mtm);
            match icon {
                Some(icon) => {
                    let ns_image = icon.get_impl().get_native(mtm);
                    unsafe { app.setApplicationIconImage(Some(&ns_image)) };
                }
                None => unsafe { app.setApplicationIconImage(None) },
            }
        });
    }

    #[inline]
    pub(super) fn stop(&self) {
        autoreleasepool(|_| {
            let mtm = MainThreadMarker::from(self);
            let app = NSApplication::sharedApplication(mtm);
            app.stop(None);
        });
    }
}
