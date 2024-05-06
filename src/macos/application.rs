use objc2::{rc::Id, runtime::ProtocolObject};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSMenuItem};
use objc2_foundation::MainThreadMarker;

use super::AppDelegate;
use crate::{platform::ApplicationHandler, Menu};

pub(crate) struct ApplicationImpl {
    pub(super) delegate: Id<AppDelegate>,
    pub(crate) menu:     Option<Menu>,
}

impl ApplicationImpl {
    pub(crate) fn new() -> Self {
        let mtm: MainThreadMarker = MainThreadMarker::new().unwrap();

        let app = NSApplication::sharedApplication(mtm);
        app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

        // configure the application delegate
        let delegate = AppDelegate::new(mtm, app.clone());
        let object = ProtocolObject::from_ref(&*delegate);
        app.setDelegate(Some(object));

        Self {
            delegate,
            menu: None,
        }
    }
}

impl ApplicationHandler for ApplicationImpl {
    #[inline]
    fn run(self) { unsafe { self.delegate.app().run() }; }

    #[inline]
    fn set_menu(&mut self, menu: Option<Menu>) {
        self.menu = menu;

        if let Some(menu) = &self.menu {
            let root_menu = Some(menu.menu_impl.native.clone());

            let mut items: Vec<Id<NSMenuItem>> = Vec::new();
            for item in menu.menu_impl.items.iter() {
                item.menu_item_impl.invalidate();
                items.push(item.menu_item_impl.native.clone());
            }

            self.delegate.set_menu(root_menu, items);
        } else {
            self.delegate.set_menu(None, Vec::new());
        }
    }
}
