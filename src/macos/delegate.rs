use std::cell::RefCell;

use objc2::{declare_class, msg_send_id, mutability, rc::Id, ClassType, DeclaredClass};
use objc2_app_kit::{NSApplication, NSApplicationDelegate, NSMenu, NSMenuItem};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol};

#[derive(Debug)]
#[allow(unused)]
pub(super) struct AppDelegateIvars {
    mtm:        MainThreadMarker,
    app:        Id<NSApplication>,
    root_menu:  RefCell<Option<Id<NSMenu>>>,
    root_items: RefCell<Vec<Id<NSMenuItem>>>,
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
            root_menu: RefCell::new(None),
            root_items: RefCell::new(Vec::new()),
        });
        unsafe { msg_send_id![super(this), init] }
    }

    #[inline]
    pub(super) fn mtm(&self) -> MainThreadMarker { self.ivars().mtm }

    #[inline]
    pub(super) fn app(&self) -> &Id<NSApplication> { &self.ivars().app }

    #[inline]
    pub(super) fn set_menu(&self, root_menu: Option<Id<NSMenu>>, root_items: Vec<Id<NSMenuItem>>) {
        let mut ivar_menu = self.ivars().root_menu.borrow_mut();
        *ivar_menu = root_menu;

        let mut ivar_menu_items = self.ivars().root_items.borrow_mut();
        *ivar_menu_items = root_items;
    }

    fn invalidate_menu(&self) {
        if let Some(menu) = self.ivars().root_menu.borrow_mut().as_mut() {
            self.ivars().app.setMainMenu(Some(&menu));

            for item in self.ivars().root_items.borrow().iter() {
                menu.addItem(item);
            }
        }
    }
}
