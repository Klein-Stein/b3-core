use crate::{Action, ContextOwner, Icon, Menu, MenuItem, ShortCode};

pub(crate) trait MenuItemApi {
    fn new(ctx: &impl ContextOwner, separator: bool) -> Self;

    fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>;
    fn title(&self) -> String;

    fn set_action(&mut self, action: Option<Action>);

    fn set_submenu(&mut self, submenu: Option<Menu>);
    fn submenu(&self) -> Option<&Menu>;
    fn submenu_mut(&mut self) -> Option<&mut Menu>;
    fn has_submenu(&self) -> bool;

    fn set_short_code(&mut self, short_code: ShortCode);
    fn short_code(&self) -> &ShortCode;

    fn set_enabled(&mut self, enabled: bool);
    fn enabled(&self) -> bool;

    fn set_tooltip(&mut self, tooltip: Option<String>);
    fn tooltip(&self) -> Option<String>;

    fn set_icon(&mut self, icon: Option<Icon>);
    fn icon(&self) -> Option<&Icon>;
}

pub(crate) trait MenuApi {
    fn new(ctx: &impl ContextOwner, items: Vec<MenuItem>) -> Self;

    fn add_item(&mut self, item: MenuItem);
}
