use objc2::{rc::autoreleasepool, runtime::ProtocolObject};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
use objc2_foundation::MainThreadMarker;

use super::app_delegate::AppDelegate;
use crate::{
    platform::{ActiveApplicationApi, ApplicationApi},
    ActiveApplication,
    EventHandler,
    Menu,
};

#[derive(Debug)]
pub(crate) struct ActiveApplicationImpl {
    pub(super) mtm: MainThreadMarker,
}

impl ActiveApplicationImpl {
    pub(crate) fn new() -> Self {
        let mtm: MainThreadMarker = MainThreadMarker::new().unwrap();
        Self {
            mtm,
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
        let app = ActiveApplication::new();
        let delegate = AppDelegate::new(app, handler);

        autoreleasepool(|_| {
            let object = ProtocolObject::from_ref(&*delegate);
            ns_app.setDelegate(Some(object));
            // SAFETY: We do not run the application re-entrantly
            unsafe { ns_app.run() };
        });
    }
}
