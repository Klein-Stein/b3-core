use std::{
    panic::{catch_unwind, UnwindSafe},
    rc::{Rc, Weak},
};

use objc2::{
    rc::{autoreleasepool, Id},
    runtime::ProtocolObject,
};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
use objc2_foundation::MainThreadMarker;

use super::{
    app_delegate::AppDelegate,
    events::dummy_event,
    observers::setup_control_flow_observers,
    panicinfo::PanicInfo,
};
use crate::{
    platform::{ActiveApplicationApi, ApplicationApi},
    ActiveApplication,
    EventHandler,
    Menu,
};

pub(super) fn stop_app_immediately(app: &NSApplication) {
    autoreleasepool(|_| {
        app.stop(None);
        // To stop event loop immediately, we need to post some event here.
        // See: https://stackoverflow.com/questions/48041279/stopping-the-nsapplication-main-event-loop/48064752#48064752
        app.postEvent_atStart(&dummy_event().unwrap(), true);
    });
}

/// Catches panics that happen inside `f` and when a panic
/// happens, stops the `sharedApplication`
#[inline]
pub fn stop_app_on_panic<F: FnOnce() -> R + UnwindSafe, R>(
    mtm: MainThreadMarker,
    panic_info: Weak<PanicInfo>,
    f: F,
) -> Option<R> {
    match catch_unwind(f) {
        Ok(r) => Some(r),
        Err(e) => {
            // It's important that we set the panic before requesting a `stop`
            // because some callback are still called during the `stop` message
            // and we need to know in those callbacks if the application is currently
            // panicking
            {
                let panic_info = panic_info.upgrade().unwrap();
                panic_info.set_panic(e);
            }
            let app = NSApplication::sharedApplication(mtm);
            stop_app_immediately(&app);
            None
        }
    }
}

#[derive(Debug)]
pub(crate) struct ActiveApplicationImpl {
    pub(super) mtm:      MainThreadMarker,
    pub(super) delegate: Id<AppDelegate>,
}

impl ActiveApplicationImpl {
    fn new(mtm: MainThreadMarker, delegate: Id<AppDelegate>) -> Self {
        Self {
            mtm,
            delegate,
        }
    }
}

impl ActiveApplicationApi for ActiveApplicationImpl {
    #[inline]
    fn set_menu(&mut self, menu: Option<&Menu>) {
        let app = NSApplication::sharedApplication(self.mtm);
        if let Some(menu) = menu {
            app.setMainMenu(Some(&menu.menu_impl.native));
        } else {
            app.setMainMenu(None);
        }
    }

    #[inline]
    fn stop(&mut self) {
        autoreleasepool(|_| {
            let app = NSApplication::sharedApplication(self.mtm);
            app.stop(None);
        });
    }
}

#[derive(Debug)]
pub(crate) struct ApplicationImpl;

impl ApplicationApi for ApplicationImpl {
    fn new() -> Self { Self {} }

    fn run(&mut self, handler: impl EventHandler + 'static) {
        let mtm: MainThreadMarker = MainThreadMarker::new()
            .expect("on macOS, `Application` instance must be created on the main thread!");

        let ns_app = NSApplication::sharedApplication(mtm);
        ns_app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

        // configure the application delegate
        let delegate = AppDelegate::new(mtm, handler);

        let app = ActiveApplicationImpl::new(mtm, delegate.clone());
        let app = ActiveApplication::new(app);
        delegate.set_active_application(app);

        autoreleasepool(|_| {
            let object = ProtocolObject::from_ref(&*delegate);
            ns_app.setDelegate(Some(object));
        });

        let panic_info: Rc<PanicInfo> = Default::default();
        setup_control_flow_observers(Rc::downgrade(&panic_info));

        autoreleasepool(|_| {
            // SAFETY: We do not run the application re-entrantly
            unsafe { ns_app.run() };
        });
    }
}
