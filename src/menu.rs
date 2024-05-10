//! This module contains a platform independent application menu implementation.

use crate::{
    macos::{MenuImpl, MenuItemImpl},
    platform::MenuItemHandler,
    Application,
};

/// Menu item action.
#[derive(Debug)]
pub enum Action {
    /// This variant will send an event with the specified action name into the
    /// event loop.
    ///
    /// Use this variant when you want to capture menu event by an event
    /// handler.
    Event(String),

    /// An action callback for a menu item.
    Callback(fn()),
}

impl Action {
    /// Creates a new action of the event type.
    ///
    /// # Parameters:
    /// * `name` - Action name.
    pub fn new_event<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self::Event(name.into())
    }

    /// Creates a new action callback.
    ///
    /// # Parameters:
    /// * `callback` - Action callback.
    pub fn new_callback(callback: fn()) -> Self { Self::Callback(callback) }
}

/// This structure represents short codes (menu hotkeys) for different
/// platforms.
#[derive(Debug, Default)]
pub struct ShortCode {
    pub(crate) macos: Option<String>,
}

impl ShortCode {
    /// Returns a short code for macOS platform or `None`.
    pub fn macos_code(&self) -> Option<&String> { self.macos.as_ref() }
}

/// Application menu item.
#[derive(Debug)]
pub struct MenuItem {
    pub(crate) menu_item_impl: MenuItemImpl,
}

impl MenuItem {
    /// Returns a new builder instance.
    pub fn builder() -> MenuItemBuilder { MenuItemBuilder::new() }

    fn new(app: &Application) -> Self {
        Self {
            menu_item_impl: MenuItemImpl::new(app, false),
        }
    }

    /// Creates a new menu separator.
    ///
    /// # Parameters:
    /// * `app` - Current application.
    pub fn separator(app: &Application) -> Self {
        Self {
            menu_item_impl: MenuItemImpl::new(app, true),
        }
    }
}

impl MenuItem {
    /// Sets a new menu item title.
    ///
    /// # Parameters:
    /// * `title` - Title.
    pub fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>,
    {
        self.menu_item_impl.set_title(title);
    }

    /// Returns a menu item title.
    pub fn title(&self) -> String { self.menu_item_impl.title() }

    /// Sets a menu item action.
    ///
    /// # Parameters:
    /// * `action` - Menu item action.
    pub fn set_action(&mut self, action: Option<Action>) { self.menu_item_impl.set_action(action); }

    /// Sets a submenu.
    ///
    /// # Parameters:
    /// * `submenu` - Submenu of the current menu item.
    pub fn set_submenu(&mut self, submenu: Option<Menu>) {
        self.menu_item_impl.set_submenu(submenu);
    }

    /// Sets short codes for different platforms.
    ///
    /// # Parameters:
    /// * `short_code` - Short codes.
    pub fn set_short_code(&mut self, short_code: ShortCode) {
        self.menu_item_impl.set_short_code(short_code);
    }

    /// Returns short codes for different platforms.
    pub fn short_code(&self) -> &ShortCode { self.menu_item_impl.short_code() }

    /// Turns on/off a menu item.
    ///
    /// # Parameters:
    /// * `enabled` - Enable flag.
    pub fn set_enabled(&mut self, enabled: bool) { self.menu_item_impl.set_enabled(enabled); }

    /// Returns if a menu item is turned on/off.
    pub fn enabled(&self) -> bool { self.menu_item_impl.enabled() }
}

/// Menu item builder.
#[derive(Debug)]
pub struct MenuItemBuilder {
    title:      Option<String>,
    action:     Option<Action>,
    submenu:    Option<Menu>,
    short_code: ShortCode,
    enabled:    Option<bool>,
}

impl MenuItemBuilder {
    fn new() -> Self {
        Self {
            title:      None,
            action:     None,
            submenu:    None,
            short_code: Default::default(),
            enabled:    None,
        }
    }

    /// Sets a title for the item under building.
    ///
    /// # Parameters:
    /// * `title` - Title.
    pub fn with_title<S>(mut self, title: S) -> MenuItemBuilder
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    /// Sets an action for the item under building.
    ///
    /// # Parameters:
    /// * `action` - Action.
    pub fn with_action(mut self, action: Action) -> MenuItemBuilder {
        self.action = Some(action);
        self
    }

    /// Sets a submenu for the item under building.
    ///
    /// # Parameters:
    /// * `submenu` - Menu item's submenu.
    pub fn with_submenu(mut self, submenu: Menu) -> MenuItemBuilder {
        self.submenu = Some(submenu);
        self
    }

    /// Sets short codes for the item under building.
    ///
    /// # Parameters:
    /// * `short_code` - Short codes.
    pub fn with_macos_short_code<S>(mut self, short_code: S) -> MenuItemBuilder
    where
        S: Into<String>,
    {
        self.short_code.macos = Some(short_code.into());
        self
    }

    /// Turns on/off the item under building.
    ///
    /// # Parameters:
    /// * `enabled` -  Enable flag.
    pub fn with_enabled(mut self, enabled: bool) -> MenuItemBuilder {
        self.enabled = Some(enabled);
        self
    }

    /// Build a new menu item with specified options.
    ///
    /// # Parameters:
    /// * `app` - Current application.
    pub fn build(self, app: &Application) -> MenuItem {
        let mut item = MenuItem::new(app);

        if let Some(title) = self.title {
            item.set_title(title);
        }

        item.set_action(self.action);
        item.set_submenu(self.submenu);
        item.set_short_code(self.short_code);

        if let Some(enabled) = self.enabled {
            item.set_enabled(enabled);
        }

        item
    }
}

/// Application menu/submenu.
#[derive(Debug)]
pub struct Menu {
    pub(crate) menu_impl: MenuImpl,
}

impl Menu {
    /// Returns a new builder instance.
    pub fn builder() -> MenuBuilder { MenuBuilder::new() }

    fn new(app: &Application, items: Vec<MenuItem>) -> Self {
        Self {
            menu_impl: MenuImpl::new(app, items),
        }
    }
}

/// Menu item builder.
#[derive(Debug)]
pub struct MenuBuilder {
    items: Vec<MenuItem>,
}

impl MenuBuilder {
    #[inline]
    fn new() -> Self {
        Self {
            items: Vec::new()
        }
    }

    /// Add a new item to the menu under building.
    ///
    /// # Parameters:
    /// * `item` - Menu item.
    pub fn with_item(mut self, item: MenuItem) -> MenuBuilder {
        self.items.push(item);
        self
    }

    /// Build a new menu with registered items.
    ///
    /// # Parameters:
    /// * `app` - Current application.
    pub fn build(self, app: &Application) -> Menu { Menu::new(app, self.items) }
}
