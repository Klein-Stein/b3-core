use std::{cell::RefCell, fmt::Debug};

use objc2::{declare_class, msg_send_id, mutability, rc::Id, ClassType, DeclaredClass};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol};

use crate::{ActiveApplication, Event, EventHandler, LifeCycle};

#[derive(Debug)]
pub(super) struct ActivationPolicy(NSApplicationActivationPolicy);

impl Default for ActivationPolicy {
    fn default() -> Self { Self(NSApplicationActivationPolicy::Regular) }
}

pub(super) struct Ivars {
    app: RefCell<ActiveApplication>,
    activation_policy: ActivationPolicy,
    activate_ignoring_other_apps: bool,
    handler: RefCell<Box<dyn EventHandler>>,
}

impl Debug for Ivars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ivars")
            .field("activation_policy", &self.activation_policy)
            .field(
                "activate_ignoring_other_apps",
                &self.activate_ignoring_other_apps,
            )
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

            #[allow(deprecated)]
            app.activateIgnoringOtherApps(self.ivars().activate_ignoring_other_apps);

            self.handle_event(Event::LifeCycle(LifeCycle::Start));
        }

        #[method(applicationWillTerminate:)]
        fn will_terminate(&self, _notification: &NSNotification) {
            self.handle_event(Event::LifeCycle(LifeCycle::Finish));
        }
    }
);

impl AppDelegate {
    pub(super) fn new(app: ActiveApplication, handler: impl EventHandler + 'static) -> Id<Self> {
        let this = app.0.mtm.alloc();
        let this = this.set_ivars(Ivars {
            app: RefCell::new(app),
            activate_ignoring_other_apps: true,
            activation_policy: Default::default(),
            handler: RefCell::new(Box::new(handler)),
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

    pub(super) fn handle_event(&self, event: Event) {
        let mut app = self.ivars().app.borrow_mut();
        self.ivars().handler.borrow_mut().on_event(&mut app, event);
    }
}
