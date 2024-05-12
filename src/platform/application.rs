use crate::{EventHandler, Menu};

pub trait ActiveApplicationApi {
    fn set_menu(&mut self, menu: Option<&Menu>);

    fn stop(&mut self);
}

pub(crate) trait ApplicationApi {
    fn new() -> Self;
    fn run(&mut self, handler: impl EventHandler + 'static);
}
