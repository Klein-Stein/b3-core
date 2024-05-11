use std::{cell::RefCell, fmt::Debug};

use objc2::{declare_class, msg_send_id, mutability, rc::Id, sel, ClassType, DeclaredClass};
use objc2_app_kit::{NSEventModifierFlags, NSMenu, NSMenuItem};
use objc2_foundation::{MainThreadMarker, NSObjectProtocol, NSString};

use crate::{
    macos::app_delegate::AppDelegate,
    platform::{MenuHandler, MenuItemHandler},
    Action,
    Application,
    Event,
    Menu,
    MenuItem,
    ShortCode,
};

#[derive(Debug, Default)]
pub(super) struct Ivars {
    action: RefCell<Option<Action>>,
}

declare_class!(
    #[derive(Debug)]
    pub(super) struct CocoaMenuItem;

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - Main thread only mutability is correct, since this is an application delegate.
    // - `AppDelegate` does not implement `Drop`.
    unsafe impl ClassType for CocoaMenuItem {
        type Super = NSMenuItem;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "CocoaMenuItem";
    }

    impl DeclaredClass for CocoaMenuItem {
        type Ivars = Ivars;
    }

    unsafe impl CocoaMenuItem {
        #[method(callback)]
        fn __callback(&self) {
            let action = self.ivars().action.borrow();
            if let Some(action) = &*action {
                match action {
                    Action::Event(name) => {
                        let delegate = AppDelegate::get(MainThreadMarker::new().unwrap());
                        delegate.handle_event(Event::Menu(name.clone()));
                    },
                    Action::Callback(callback) => callback(),
                }
            }
        }
    }

    unsafe impl NSObjectProtocol for CocoaMenuItem {}
);

impl CocoaMenuItem {
    fn new(mtm: MainThreadMarker) -> Id<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(Ivars {
            action: RefCell::new(None),
        });

        unsafe { msg_send_id![super(this), init] }
    }

    fn set_action(&self, action: Option<Action>) {
        if action.is_some() {
            unsafe { self.setTarget(Some(&self)) };
            unsafe { self.setAction(Some(sel!(callback))) };
        } else {
            unsafe { self.setTarget(None) };
            unsafe { self.setAction(None) };
        }
        *self.ivars().action.borrow_mut() = action;
    }
}

#[derive(Debug)]
pub(crate) struct MenuItemImpl {
    pub(crate) short_code: ShortCode,
    pub(super) native:     Id<CocoaMenuItem>,
    pub(super) submenu:    Option<Menu>,
}

impl MenuItemImpl {
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
    fn new(app: &Application, separator: bool) -> Self {
        let mtm = app.0.mtm;
        let native = if separator {
            unsafe { msg_send_id![CocoaMenuItem::class(), separatorItem] }
        } else {
            CocoaMenuItem::new(mtm)
        };
        Self {
            native,
            submenu: None,
            short_code: Default::default(),
        }
    }

    #[inline]
    fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>,
    {
        let title = title.into();
        let title = NSString::from_str(&title);
        unsafe { self.native.setTitle(&title) };
    }

    #[inline]
    fn title(&self) -> String { unsafe { self.native.title().to_string() } }

    #[inline]
    fn set_action(&mut self, action: Option<Action>) { self.native.set_action(action); }

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

    #[inline]
    fn set_enabled(&mut self, enabled: bool) { unsafe { self.native.setEnabled(enabled) }; }

    #[inline]
    fn enabled(&self) -> bool { unsafe { self.native.isEnabled() } }
}

#[derive(Debug)]
pub(crate) struct MenuImpl {
    pub(super) native: Id<NSMenu>,
    pub(crate) items:  Vec<MenuItem>,
}

impl MenuImpl {
    pub(crate) fn new(app: &Application, items: Vec<MenuItem>) -> Self {
        let native = NSMenu::new(app.0.mtm);

        unsafe { native.setAutoenablesItems(false) };

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
    #[inline]
    fn add_item(&mut self, item: MenuItem) {
        self.native.addItem(&item.menu_item_impl.native);
        self.items.push(item);
    }
}
