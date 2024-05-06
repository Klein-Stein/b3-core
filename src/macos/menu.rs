use objc2::{declare_class, msg_send_id, mutability, rc::Id, sel, ClassType, DeclaredClass};
use objc2_app_kit::{NSEventModifierFlags, NSMenu, NSMenuItem};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol, NSString};

use crate::{
    platform::{MenuHandler, MenuItemHandler},
    Application,
    Menu,
    MenuItem,
    ShortCode,
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
    pub(super) mtm:        MainThreadMarker,
    pub(crate) title:      String,
    pub(super) action:     Option<Id<ActionHandler>>,
    pub(crate) short_code: ShortCode,
    pub(super) native:     Id<NSMenuItem>,
    pub(super) submenu:    Option<Menu>,
}

impl MenuItemImpl {
    pub(crate) fn new(app: &Application, separator: bool) -> Self {
        let mtm = app.application_impl.delegate.mtm();
        Self {
            mtm,
            title: "".to_owned(),
            action: None,
            native: if separator {
                NSMenuItem::separatorItem(mtm)
            } else {
                NSMenuItem::new(mtm)
            },
            submenu: None,
            short_code: Default::default(),
        }
    }

    fn parse_short_code(&self, code: &String) {
        let parts = code.split("+").collect::<Vec<&str>>();
        let mut masks = Vec::new();
        let mut code = String::from("");

        for part in parts.iter() {
            match *part {
                "Control" => {
                    masks.push(NSEventModifierFlags::NSEventModifierFlagControl.0);
                }
                "Command" => {
                    masks.push(NSEventModifierFlags::NSEventModifierFlagCommand.0);
                }
                "Help" => {
                    masks.push(NSEventModifierFlags::NSEventModifierFlagHelp.0);
                }
                "Function" => {
                    masks.push(NSEventModifierFlags::NSEventModifierFlagFunction.0);
                }
                "Option" => {
                    masks.push(NSEventModifierFlags::NSEventModifierFlagOption.0);
                }
                "Shift" => {
                    masks.push(NSEventModifierFlags::NSEventModifierFlagShift.0);
                }
                "CapsLock" => {
                    masks.push(NSEventModifierFlags::NSEventModifierFlagCapsLock.0);
                }
                "NumPad" => {
                    masks.push(NSEventModifierFlags::NSEventModifierFlagNumericPad.0);
                }
                c => {
                    code = c.to_owned();
                }
            }
        }

        let code = NSString::from_str(&code);
        unsafe { self.native.setKeyEquivalent(&code) };

        let mut mask = if masks.is_empty() {
            NSEventModifierFlags::NSEventModifierFlagCommand.0
        } else {
            masks[0]
        };

        for m in masks.iter().skip(1) {
            mask |= m;
        }

        self.native
            .setKeyEquivalentModifierMask(NSEventModifierFlags(mask));
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
    fn set_submenu(&mut self, submenu: Option<Menu>) {
        self.submenu = submenu;
        if let Some(submenu) = &self.submenu {
            self.native.setSubmenu(Some(&submenu.menu_impl.native));
        } else {
            self.native.setSubmenu(None);
        }
    }

    #[inline]
    fn set_short_code(&mut self, short_code: ShortCode) {
        self.short_code = short_code;

        if let Some(code) = &self.short_code.macos {
            self.parse_short_code(code);
        }
    }

    #[inline]
    fn short_code(&self) -> &ShortCode { &self.short_code }
}

#[derive(Debug)]
pub(crate) struct MenuImpl {
    pub(super) native: Id<NSMenu>,
    pub(crate) items:  Vec<MenuItem>,
}

impl MenuImpl {
    pub(crate) fn new(app: &Application, items: Vec<MenuItem>) -> Self {
        let native = NSMenu::new(app.application_impl.delegate.mtm());

        for item in items.iter() {
            native.addItem(&item.menu_item_impl.native);
        }

        Self {
            native,
            items,
        }
    }
}

impl MenuHandler for MenuImpl {
    fn add_item(&mut self, item: MenuItem) {
        self.native.addItem(&item.menu_item_impl.native);
        self.items.push(item);
    }
}
