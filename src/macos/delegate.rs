use std::cell::RefCell;

use objc2::{declare_class, msg_send_id, mutability, rc::Id, ClassType, DeclaredClass};
use objc2_app_kit::{NSApplication, NSApplicationDelegate, NSMenu};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol};

#[derive(Debug)]
#[allow(unused)]
pub(super) struct AppDelegateIvars {
    mtm:  MainThreadMarker,
    app:  Id<NSApplication>,
    menu: RefCell<Option<Id<NSMenu>>>,
}

declare_class!(
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
        type Ivars = AppDelegateIvars;
    }

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[method(applicationDidFinishLaunching:)]
        fn did_finish_launching(&self, _notification: &NSNotification) {
            self.invalidate_menu();
        }

        #[method(applicationWillTerminate:)]
        fn will_terminate(&self, _notification: &NSNotification) {
        }
    }
);

impl AppDelegate {
    pub(super) fn new(mtm: MainThreadMarker, app: Id<NSApplication>) -> Id<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(AppDelegateIvars {
            mtm,
            app,
            menu: RefCell::new(None),
        });
        unsafe { msg_send_id![super(this), init] }
    }

    #[inline]
    pub(super) fn mtm(&self) -> MainThreadMarker { self.ivars().mtm }

    #[inline]
    pub(super) fn app(&self) -> &Id<NSApplication> { &self.ivars().app }

    #[inline]
    pub(super) fn set_menu(&self, menu: Option<Id<NSMenu>>) {
        let mut ivar_menu = self.ivars().menu.borrow_mut();
        *ivar_menu = menu;
    }

    fn invalidate_menu(&self) {
        if let Some(menu) = self.ivars().menu.borrow_mut().as_mut() {
            self.ivars().app.setMainMenu(Some(&menu));
        }
    }
}
