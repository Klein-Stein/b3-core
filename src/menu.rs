//! This module contains a platform independent application menu implementation.

use crate::{
    macos::{MenuImpl, MenuItemImpl},
    platform::{MenuApi, MenuItemApi, Wrapper},
    ContextOwner,
    Image,
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
pub struct MenuItem(MenuItemImpl);

impl MenuItem {
    /// Returns a new builder instance.
    pub fn builder() -> MenuItemBuilder { MenuItemBuilder::new() }

    fn new(ctx: &impl ContextOwner) -> Self { Self(MenuItemImpl::new(ctx, false)) }

    /// Creates a new menu separator.
    ///
    /// # Parameters:
    /// * `ctx` - ContextOnwer
    pub fn separator(ctx: &impl ContextOwner) -> Self { Self(MenuItemImpl::new(ctx, true)) }
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
        self.0.set_title(title);
    }

    /// Returns a menu item title.
    pub fn title(&self) -> String { self.0.title() }

    /// Sets a menu item action.
    ///
    /// # Parameters:
    /// * `action` - Menu item action.
    pub fn set_action(&mut self, action: Option<Action>) { self.0.set_action(action); }

    /// Sets a submenu.
    ///
    /// # Parameters:
    /// * `submenu` - Submenu of the current menu item.
    pub fn set_submenu(&mut self, submenu: Option<Menu>) { self.0.set_submenu(submenu); }

    /// Returns a submenu.
    fn submenu(&self) -> Option<&Menu> { self.0.submenu() }

    /// Returns a mutable submenu.
    fn submenu_mut(&mut self) -> Option<&mut Menu> { self.0.submenu_mut() }

    /// Checks if a menu item has a submenu.
    fn has_submenu(&self) -> bool { self.0.has_submenu() }

    /// Sets short codes for different platforms.
    ///
    /// # Parameters:
    /// * `short_code` - Short codes.
    pub fn set_short_code(&mut self, short_code: ShortCode) { self.0.set_short_code(short_code); }

    /// Returns short codes for different platforms.
    pub fn short_code(&self) -> &ShortCode { self.0.short_code() }

    /// Turns on/off a menu item.
    ///
    /// # Parameters:
    /// * `enabled` - Enable flag.
    pub fn set_enabled(&mut self, enabled: bool) { self.0.set_enabled(enabled); }

    /// Returns if a menu item is turned on/off.
    pub fn enabled(&self) -> bool { self.0.enabled() }

    /// Sets a tooltip for the menu item.
    ///
    /// # Parameters:
    /// * `tooltip` - Tooltip message.
    fn set_tooltip(&mut self, tooltip: Option<String>) { self.0.set_tooltip(tooltip); }

    /// Returns a tooltip of the menu item.
    fn tooltip(&self) -> Option<String> { self.0.tooltip() }

    /// Sets a menu item icon.
    ///
    /// # Parameters:
    /// * `icon` - Menu item icon.
    fn set_icon(&mut self, icon: Option<Image>) { self.0.set_icon(icon); }

    /// Returns a menu item icon.
    fn icon(&self) -> Option<&Image> { self.0.icon() }
}

impl Wrapper<MenuItemImpl> for MenuItem {
    #[inline]
    fn get_impl(&self) -> &MenuItemImpl { &self.0 }

    #[inline]
    fn get_impl_mut(&mut self) -> &mut MenuItemImpl { &mut self.0 }
}

/// Menu item builder.
#[derive(Debug, Default)]
pub struct MenuItemBuilder {
    title:      Option<String>,
    action:     Option<Action>,
    submenu:    Option<Menu>,
    short_code: ShortCode,
    enabled:    Option<bool>,
    icon:       Option<Image>,
}

impl MenuItemBuilder {
    fn new() -> Self {
        Self {
            ..Default::default()
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

    /// Sets an icon for the item under building.
    ///
    /// # Parameters:
    /// * `icon` - Menu item icon.
    pub fn with_icon(mut self, icon: Image) -> MenuItemBuilder {
        self.icon = Some(icon);
        self
    }

    /// Build a new menu item with specified options.
    ///
    /// # Parameters:
    /// * `ctx` - Context owner.
    pub fn build(self, ctx: &impl ContextOwner) -> MenuItem {
        let mut item = MenuItem::new(ctx);

        if let Some(title) = self.title {
            item.set_title(title);
        }

        if self.action.is_some() {
            item.set_action(self.action);
        }

        if self.submenu.is_some() {
            item.set_submenu(self.submenu);
        }

        item.set_short_code(self.short_code);

        if let Some(enabled) = self.enabled {
            item.set_enabled(enabled);
        }

        if self.icon.is_some() {
            item.set_icon(self.icon);
        }

        item
    }
}

/// Application menu/submenu.
#[derive(Debug)]
pub struct Menu(MenuImpl);

impl Menu {
    /// Returns a new builder instance.
    pub fn builder() -> MenuBuilder { MenuBuilder::new() }

    fn new(ctx: &impl ContextOwner, items: Vec<MenuItem>) -> Self {
        Self(MenuImpl::new(ctx, items))
    }
}

impl Wrapper<MenuImpl> for Menu {
    #[inline]
    fn get_impl(&self) -> &MenuImpl { &self.0 }

    #[inline]
    fn get_impl_mut(&mut self) -> &mut MenuImpl { &mut self.0 }
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
    /// * `ctx` - Context owner.
    pub fn build(self, ctx: &impl ContextOwner) -> Menu { Menu::new(ctx, self.items) }
}
