use crate::{EventHandler, Menu};

pub(crate) trait ApplicationHandler {
    fn set_menu(&mut self, menu: Option<Menu>);
    fn run(&mut self, handler: impl EventHandler + 'static);
}
