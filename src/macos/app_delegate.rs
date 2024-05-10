use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    rc::Weak,
};

use objc2::{declare_class, msg_send_id, mutability, rc::Id, ClassType, DeclaredClass};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSMenu};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol};

use super::runloop::PanicInfo;
use crate::{macos::runloop::stop_app_immediately, Event, EventHandler, LifeCycle};

#[derive(Debug)]
pub(super) struct ActivationPolicy(NSApplicationActivationPolicy);

impl Default for ActivationPolicy {
    fn default() -> Self { Self(NSApplicationActivationPolicy::Regular) }
}

pub(super) struct Ivars {
    activation_policy: ActivationPolicy,
    activate_ignoring_other_apps: bool,
    menu: RefCell<Option<Id<NSMenu>>>,
    stop_on_launch: Cell<bool>,
    handler: Box<dyn EventHandler>,
}

impl Debug for Ivars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ivars")
            .field("activation_policy", &self.activation_policy)
            .field(
                "activate_ignoring_other_apps",
                &self.activate_ignoring_other_apps,
            )
            .field("menu", &self.menu)
            .field("stop_on_launch", &self.stop_on_launch)
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
        const NAME: &'static str = "B3AppDelegate";
    }

    impl DeclaredClass for AppDelegate {
        type Ivars = Ivars;
    }

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[method(applicationDidFinishLaunching:)]
        fn did_finish_launching(&self, _notification: &NSNotification) {
            let mtm = MainThreadMarker::from(self);
            let app = NSApplication::sharedApplication(mtm);
            // We need to delay setting the activation policy and activating the app
            // until `applicationDidFinishLaunching` has been called. Otherwise the
            // menu bar is initially unresponsive on macOS 10.15.
            app.setActivationPolicy(self.ivars().activation_policy.0);

            Self::window_activation_hack(&app);
            #[allow(deprecated)]
            app.activateIgnoringOtherApps(self.ivars().activate_ignoring_other_apps);

            self.invalidate_menu(&app);

            self.handle_event(Event::LifeCycle(LifeCycle::Start));

            // If the application is being launched via `EventLoop::pump_app_events()` then we'll
            // want to stop the app once it is launched (and return to the external loop)
            //
            // In this case we still want to consider Winit's `EventLoop` to be "running",
            // so we call `start_running()` above.
            if self.ivars().stop_on_launch.get() {
                // NOTE: the original idea had been to only stop the underlying `RunLoop`
                // for the app but that didn't work as expected (`-[NSApplication run]`
                // effectively ignored the attempt to stop the RunLoop and re-started it).
                //
                // So we return from `pump_events` by stopping the application.
                let app = NSApplication::sharedApplication(mtm);
                stop_app_immediately(&app);
            }
        }

        #[method(applicationWillTerminate:)]
        fn will_terminate(&self, _notification: &NSNotification) {
            self.handle_event(Event::LifeCycle(LifeCycle::Finish));
        }
    }
);

impl AppDelegate {
    pub(super) fn new(
        mtm: MainThreadMarker,
        menu: Option<Id<NSMenu>>,
        handler: impl EventHandler + 'static,
    ) -> Id<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(Ivars {
            activate_ignoring_other_apps: true,
            menu: RefCell::new(menu),
            activation_policy: Default::default(),
            stop_on_launch: Cell::new(false),
            handler: Box::new(handler),
        });
        unsafe { msg_send_id![super(this), init] }
    }

    pub fn get(mtm: MainThreadMarker) -> Id<Self> {
        let app = NSApplication::sharedApplication(mtm);
        let delegate =
            unsafe { app.delegate() }.expect("a delegate was not configured on the application");
        if delegate.is_kind_of::<Self>() {
            // SAFETY: Just checked that the delegate is an instance of `ApplicationDelegate`
            unsafe { Id::cast(delegate) }
        } else {
            panic!("tried to get a delegate that was not the one Winit has registered")
        }
    }

    pub(super) fn handle_event(&self, event: Event) { self.ivars().handler.on_event(event); }

    // Called by RunLoopObserver after finishing waiting for new events
    pub fn wakeup(&self, _panic_info: Weak<PanicInfo>) {}

    // Called by RunLoopObserver before waiting for new events
    pub fn cleared(&self, _panic_info: Weak<PanicInfo>) {}

    #[inline]
    pub(super) fn set_menu(&self, menu: Option<Id<NSMenu>>) {
        let mut ivar_menu = self.ivars().menu.borrow_mut();
        *ivar_menu = menu;
    }

    fn invalidate_menu(&self, app: &Id<NSApplication>) {
        if let Some(menu) = self.ivars().menu.borrow_mut().as_mut() {
            app.setMainMenu(Some(&menu));
        }
    }

    /// A hack to make activation of multiple windows work when creating them before
    /// `applicationDidFinishLaunching:` / `Event::Event::NewEvents(StartCause::Init)`.
    ///
    /// Alternative to this would be the user calling `window.set_visible(true)` in
    /// `StartCause::Init`.
    ///
    /// If this becomes too bothersome to maintain, it can probably be removed
    /// without too much damage.
    fn window_activation_hack(app: &NSApplication) {
        // TODO: Proper ordering of the windows
        app.windows().into_iter().for_each(|window| {
            // Call `makeKeyAndOrderFront` if it was called on the window in `WinitWindow::new`
            // This way we preserve the user's desired initial visibility status
            // TODO: Also filter on the type/"level" of the window, and maybe other things?
            if window.isVisible() {
                window.makeKeyAndOrderFront(None);
            }
        })
    }
}
