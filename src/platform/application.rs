use crate::{EventHandler, Menu, Window, WindowId};

pub(crate) trait ApplicationHandler {
    fn set_menu(&mut self, menu: Option<Menu>);

    fn add_window(&mut self, window: Window);
    fn get_window(&self, id: &WindowId) -> Option<&Window>;
    fn get_window_mut(&mut self, id: &WindowId) -> Option<&mut Window>;

    fn run(&mut self, handler: impl EventHandler + 'static);
}
