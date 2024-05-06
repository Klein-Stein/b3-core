use crate::{
    macos::{MenuImpl, MenuItemImpl},
    platform::MenuItemHandler,
    Application,
};

#[derive(Debug)]
pub struct MenuItem {
    pub(crate) menu_item_impl: MenuItemImpl,
}

impl MenuItem {
    pub fn builder() -> MenuItemBuilder { MenuItemBuilder::new() }

    fn new(app: &Application) -> Self {
        Self {
            menu_item_impl: MenuItemImpl::new(app),
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

    fn title(&self) -> &String { self.menu_item_impl.title() }

    fn set_action(&mut self, action: Option<fn()>) { self.menu_item_impl.set_action(action); }

    fn set_submenu(&mut self, submenu: Option<Menu>) { self.menu_item_impl.set_submenu(submenu); }
}

#[derive(Debug)]
pub struct MenuItemBuilder {
    title:   Option<String>,
    action:  Option<fn()>,
    submenu: Option<Menu>,
}

impl MenuItemBuilder {
    fn new() -> Self {
        Self {
            title:   None,
            action:  None,
            submenu: None,
        }
    }

    pub fn with_title<S>(mut self, title: S) -> MenuItemBuilder
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    pub fn with_action(mut self, action: fn()) -> MenuItemBuilder {
        self.action = Some(action);
        self
    }

    pub fn with_submenu(mut self, submenu: Menu) -> MenuItemBuilder {
        self.submenu = Some(submenu);
        self
    }

    pub fn build(self, app: &Application) -> MenuItem {
        let mut item = MenuItem::new(app);

        if let Some(title) = self.title {
            item.set_title(title);
        }

        item.set_action(self.action);
        item.set_submenu(self.submenu);

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
