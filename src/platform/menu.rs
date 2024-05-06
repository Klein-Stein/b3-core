use crate::{Menu, MenuItem};

pub(crate) trait MenuItemHandler {
    fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>;
    fn title(&self) -> &String;

    fn set_action(&mut self, action: Option<fn()>);

    fn set_submenu(&mut self, submenu: Option<Menu>);
}

pub(crate) trait MenuHandler {
    fn add_item(&mut self, item: MenuItem);
}
