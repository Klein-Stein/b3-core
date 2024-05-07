use crate::{Menu, MenuItem, ShortCode};

pub(crate) trait MenuItemHandler {
    fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>;
    fn title(&self) -> &String;

    fn set_action(&mut self, action: Option<fn()>);

    fn set_submenu(&mut self, submenu: Option<Menu>);

    fn set_short_code(&mut self, short_code: ShortCode);
    fn short_code(&self) -> &ShortCode;

    fn set_enabled(&mut self, enabled: bool);
    fn enabled(&self) -> bool;
}

pub(crate) trait MenuHandler {
    fn add_item(&mut self, item: MenuItem);
}
