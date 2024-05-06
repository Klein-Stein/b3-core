use objc2::{declare_class, msg_send_id, mutability, rc::Id, sel, ClassType, DeclaredClass};
use objc2_app_kit::{NSMenu, NSMenuItem};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol, NSString};

use crate::{
    platform::{MenuHandler, MenuItemHandler},
    Application,
    Menu,
    MenuItem,
};

#[derive(Debug)]
#[allow(unused)]
pub(super) struct ActionHandlerIvars {
    action: fn(),
}

declare_class!(
    #[derive(Debug)]
    pub(super) struct ActionHandler;

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - Main thread only mutability is correct, since this is an application delegate.
    // - `AppDelegate` does not implement `Drop`.
    unsafe impl ClassType for ActionHandler {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "B3ActionHandler";
    }

    impl DeclaredClass for ActionHandler {
        type Ivars = ActionHandlerIvars;
    }

    unsafe impl ActionHandler {
        #[method(callback)]
        fn __callback(&self) {
            (self.ivars().action)();
        }
    }

    unsafe impl NSObjectProtocol for ActionHandler {}
);

impl ActionHandler {
    pub(super) fn new(mtm: MainThreadMarker, action: fn()) -> Id<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(ActionHandlerIvars {
            action,
        });
        unsafe { msg_send_id![super(this), init] }
    }
}

#[derive(Debug)]
pub(crate) struct MenuItemImpl {
    pub(super) mtm:     MainThreadMarker,
    pub(crate) title:   String,
    pub(super) action:  Option<Id<ActionHandler>>,
    pub(super) native:  Id<NSMenuItem>,
    pub(super) submenu: Option<Menu>,
}

impl MenuItemImpl {
    pub(crate) fn new(app: &Application) -> Self {
        let mtm = app.application_impl.delegate.mtm();
        Self {
            mtm,
            title: "".to_owned(),
            action: None,
            native: NSMenuItem::new(mtm),
            submenu: None,
        }
    }

    pub(super) fn invalidate(&self) {
        if let Some(submenu) = &self.submenu {
            submenu.menu_impl.invalidate();
            self.native.setSubmenu(Some(&submenu.menu_impl.native));
        } else {
            self.native.setSubmenu(None);
        }
    }
}

impl MenuItemHandler for MenuItemImpl {
    #[inline]
    fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>,
    {
        self.title = title.into();
        let title = NSString::from_str(&self.title);
        unsafe { self.native.setTitle(&title) };
    }

    #[inline]
    fn title(&self) -> &String { &self.title }

    #[inline]
    fn set_action(&mut self, action: Option<fn()>) {
        if let Some(action) = action {
            let action = ActionHandler::new(self.mtm, action);
            unsafe { self.native.setTarget(Some(&action)) };
            unsafe { self.native.setAction(Some(sel!(callback))) };
            self.action = Some(action);
        } else {
            unsafe { self.native.setTarget(None) };
            unsafe { self.native.setAction(None) };
            self.action = None;
        }
    }

    #[inline]
    fn set_submenu(&mut self, submenu: Option<Menu>) { self.submenu = submenu; }
}

#[derive(Debug)]
pub(crate) struct MenuImpl {
    pub(super) native: Id<NSMenu>,
    pub(crate) items:  Vec<MenuItem>,
}

impl MenuImpl {
    pub(crate) fn new(app: &Application, items: Vec<MenuItem>) -> Self {
        let native = NSMenu::new(app.application_impl.delegate.mtm());

        Self {
            native,
            items,
        }
    }

    pub(super) fn invalidate(&self) {
        for item in self.items.iter() {
            item.menu_item_impl.invalidate();
            self.native.addItem(&item.menu_item_impl.native);
        }
    }
}

impl MenuHandler for MenuImpl {
    fn add_item(&mut self, item: MenuItem) {
        self.native.addItem(&item.menu_item_impl.native);
        self.items.push(item);
    }
}
