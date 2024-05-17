use crate::{Error, EventHandler, Menu};

pub trait ActiveApplicationApi {
    fn set_menu(&mut self, menu: Option<&Menu>);

    fn stop(&mut self);
}

pub(crate) trait ApplicationApi {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;
    fn run(&mut self, handler: impl EventHandler + 'static);
}
