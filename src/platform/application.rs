use crate::{Error, EventHandler, Icon, Menu};

pub trait ActiveApplicationApi {
    fn set_menu(&mut self, menu: Option<&Menu>);

    fn set_icon(&mut self, icon: Option<&Icon>);

    fn stop(&mut self);
}

pub(crate) trait ApplicationApi {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;
    fn run(&mut self, handler: impl EventHandler + 'static);
}
