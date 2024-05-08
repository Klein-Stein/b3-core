use crate::{EventHandler, Menu};

pub(crate) trait ApplicationHandler {
    fn set_menu(&mut self, menu: Option<Menu>);
    fn run(self, handler: impl EventHandler + 'static);
}
