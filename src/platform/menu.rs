use crate::{Action, Menu, MenuItem, ShortCode};

pub(crate) trait MenuItemApi {
    fn new(separator: bool) -> Self;

    fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>;
    fn title(&self) -> String;

    fn set_action(&mut self, action: Option<Action>);

    fn set_submenu(&mut self, submenu: Option<Menu>);

    fn set_short_code(&mut self, short_code: ShortCode);
    fn short_code(&self) -> &ShortCode;

    fn set_enabled(&mut self, enabled: bool);
    fn enabled(&self) -> bool;
}

pub(crate) trait MenuApi {
    fn add_item(&mut self, item: MenuItem);
}
