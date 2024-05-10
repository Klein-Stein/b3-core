use crate::{
    macos::{MenuImpl, MenuItemImpl},
    platform::MenuItemHandler,
    Application,
};

#[derive(Debug)]
pub enum Action {
    Event(String),
    Callback(fn()),
}

impl Action {
    pub fn new_event<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self::Event(name.into())
    }

    pub fn new_callback(callback: fn()) -> Self { Self::Callback(callback) }
}

#[derive(Debug, Default)]
pub struct ShortCode {
    pub(crate) macos: Option<String>,
}

impl ShortCode {
    pub fn macos_code(&self) -> Option<&String> { self.macos.as_ref() }
}

#[derive(Debug)]
pub struct MenuItem {
    pub(crate) menu_item_impl: MenuItemImpl,
}

impl MenuItem {
    pub fn builder() -> MenuItemBuilder { MenuItemBuilder::new() }

    fn new(app: &Application) -> Self {
        Self {
            menu_item_impl: MenuItemImpl::new(app, false),
        }
    }

    pub fn separator(app: &Application) -> Self {
        Self {
            menu_item_impl: MenuItemImpl::new(app, true),
        }
    }
}

impl MenuItem {
    fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>,
    {
        self.menu_item_impl.set_title(title);
    }

    fn title(&self) -> String { self.menu_item_impl.title() }

    fn set_action(&mut self, action: Option<Action>) { self.menu_item_impl.set_action(action); }

    fn set_submenu(&mut self, submenu: Option<Menu>) { self.menu_item_impl.set_submenu(submenu); }

    fn set_short_code(&mut self, short_code: ShortCode) {
        self.menu_item_impl.set_short_code(short_code);
    }

    fn short_code(&self) -> &ShortCode { self.menu_item_impl.short_code() }

    fn set_enabled(&mut self, enabled: bool) { self.menu_item_impl.set_enabled(enabled); }

    fn enabled(&self) -> bool { self.menu_item_impl.enabled() }
}

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

    pub fn with_title<S>(mut self, title: S) -> MenuItemBuilder
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    pub fn with_action(mut self, action: Action) -> MenuItemBuilder {
        self.action = Some(action);
        self
    }

    pub fn with_submenu(mut self, submenu: Menu) -> MenuItemBuilder {
        self.submenu = Some(submenu);
        self
    }

    pub fn with_macos_short_code<S>(mut self, short_code: S) -> MenuItemBuilder
    where
        S: Into<String>,
    {
        self.short_code.macos = Some(short_code.into());
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> MenuItemBuilder {
        self.enabled = Some(enabled);
        self
    }

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

#[derive(Debug)]
pub struct Menu {
    pub(crate) menu_impl: MenuImpl,
}

impl Menu {
    pub fn builder() -> MenuBuilder { MenuBuilder::new() }

    fn new(app: &Application, items: Vec<MenuItem>) -> Self {
        Self {
            menu_impl: MenuImpl::new(app, items),
        }
    }
}

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

    pub fn with_item(mut self, item: MenuItem) -> MenuBuilder {
        self.items.push(item);
        self
    }

    pub fn build(self, app: &Application) -> Menu { Menu::new(app, self.items) }
}
