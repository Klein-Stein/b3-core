use std::{cell::RefCell, fmt::Debug};

use objc2::{
    declare_class,
    msg_send_id,
    mutability,
    rc::{autoreleasepool, Id},
    sel,
    ClassType,
    DeclaredClass,
};
use objc2_app_kit::{NSEventModifierFlags, NSMenu, NSMenuItem};
use objc2_foundation::{MainThreadBound, MainThreadMarker, NSObjectProtocol, NSString};

use crate::{
    macos::app_delegate::AppDelegate,
    platform::{MenuApi, MenuItemApi, Wrapper},
    Action,
    ContextOwner,
    Event,
    Image,
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
    mtm:        MainThreadMarker,
    short_code: ShortCode,
    native:     MainThreadBound<Id<CocoaMenuItem>>,
    submenu:    Option<Menu>,
    icon:       Option<Image>,
}

impl MenuItemImpl {
    fn native_on_main<F, R>(&self, f: F) -> R
    where
        F: Send + FnOnce(&Id<CocoaMenuItem>) -> R,
        R: Send,
    {
        self.native
            .get_on_main(|native| autoreleasepool(|_| f(native)))
    }

    #[inline]
    fn get_native(&self) -> &Id<CocoaMenuItem> { self.native.get(self.mtm) }

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

        self.native_on_main(|native| {
            unsafe { native.setKeyEquivalent(&code) };

            let mut mask = if masks.is_empty() {
                NSEventModifierFlags::NSEventModifierFlagCommand.0
            } else {
                masks[0]
            };

            for m in masks.iter().skip(1) {
                mask |= m;
            }

            native.setKeyEquivalentModifierMask(NSEventModifierFlags(mask));
        });
    }
}

impl MenuItemApi for MenuItemImpl {
    #[inline]
    fn new(ctx: &impl ContextOwner, separator: bool) -> Self {
        let mtm = ctx.context().get_impl().mtm();
        let native = if separator {
            unsafe { msg_send_id![CocoaMenuItem::class(), separatorItem] }
        } else {
            CocoaMenuItem::new(mtm)
        };
        Self {
            mtm,
            native: MainThreadBound::new(native, mtm),
            short_code: Default::default(),
            submenu: None,
            icon: None,
        }
    }

    #[inline]
    fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>,
    {
        let title = title.into();
        self.native_on_main(|native| {
            let title = NSString::from_str(&title);
            unsafe { native.setTitle(&title) };
        });
    }

    #[inline]
    fn title(&self) -> String {
        self.native_on_main(|native| unsafe { native.title().to_string() })
    }

    #[inline]
    fn set_action(&mut self, action: Option<Action>) {
        self.native_on_main(|native| {
            native.set_action(action);
        });
    }

    #[inline]
    fn set_submenu(&mut self, submenu: Option<Menu>) {
        let native = self.get_native();
        if let Some(submenu) = &submenu {
            let ns_menu = submenu.get_impl().get_native();
            native.setSubmenu(Some(&ns_menu));
        } else {
            native.setSubmenu(None);
        }
        self.submenu = submenu;
    }

    #[inline]
    fn submenu(&self) -> Option<&Menu> { self.submenu.as_ref() }

    #[inline]
    fn submenu_mut(&mut self) -> Option<&mut Menu> { self.submenu.as_mut() }

    #[inline]
    fn has_submenu(&self) -> bool { self.native_on_main(|native| unsafe { native.hasSubmenu() }) }

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
    fn set_enabled(&mut self, enabled: bool) {
        self.native_on_main(|native| {
            unsafe { native.setEnabled(enabled) };
        });
    }

    #[inline]
    fn enabled(&self) -> bool { self.native_on_main(|native| unsafe { native.isEnabled() }) }

    #[inline]
    fn set_tooltip(&mut self, tooltip: Option<String>) {
        self.native_on_main(|native| match tooltip {
            Some(tooltip) => {
                let tooltip = NSString::from_str(&tooltip);
                unsafe { native.setToolTip(Some(&tooltip)) };
            }
            None => unsafe { native.setToolTip(None) },
        });
    }

    #[inline]
    fn tooltip(&self) -> Option<String> {
        self.native_on_main(|native| match unsafe { native.toolTip() } {
            Some(tooltip) => Some(tooltip.to_string()),
            None => None,
        })
    }

    #[inline]
    fn set_icon(&mut self, icon: Option<Image>) {
        let ns_menu = self.get_native();
        if let Some(icon) = &icon {
            let ns_icon = icon.get_impl().get_native();
            unsafe { ns_menu.setImage(Some(&ns_icon)) };
        } else {
            unsafe { ns_menu.setImage(None) };
        }
        self.icon = icon;
    }

    #[inline]
    fn icon(&self) -> Option<&Image> { self.icon.as_ref() }
}

#[derive(Debug)]
pub(crate) struct MenuImpl {
    mtm:    MainThreadMarker,
    native: MainThreadBound<Id<NSMenu>>,
    items:  Vec<MenuItem>,
}

impl MenuImpl {
    fn native_on_main<F, R>(&self, f: F) -> R
    where
        F: Send + FnOnce(&Id<NSMenu>) -> R,
        R: Send,
    {
        self.native
            .get_on_main(|native| autoreleasepool(|_| f(native)))
    }

    pub(super) fn get_native(&self) -> &Id<NSMenu> { self.native.get(self.mtm) }
}

impl MenuApi for MenuImpl {
    #[inline]
    fn new(ctx: &impl ContextOwner, items: Vec<MenuItem>) -> Self {
        let mtm = ctx.context().get_impl().mtm();

        let native = NSMenu::new(mtm);

        unsafe { native.setAutoenablesItems(false) };

        for item in items.iter() {
            native.addItem(&item.get_impl().native.get(mtm));
        }

        Self {
            mtm,
            native: MainThreadBound::new(native, mtm),
            items,
        }
    }

    #[inline]
    fn add_item(&mut self, item: MenuItem) {
        let ns_menu_item = item.get_impl().get_native();
        let ns_menu = self.get_native();
        ns_menu.addItem(&ns_menu_item);
        self.items.push(item);
    }
}
