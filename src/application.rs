use crate::{
    macos::ApplicationImpl,
    platform::ApplicationHandler,
    EventHandler,
    Menu,
    Window,
    WindowId,
};

pub struct Application(pub(crate) ApplicationImpl);

impl Application {
    pub fn new() -> Self { Self(ApplicationImpl::new()) }

    pub fn set_menu(&mut self, menu: Option<Menu>) { self.0.set_menu(menu); }

    pub fn add_window(&mut self, window: Window) { self.0.add_window(window); }

    pub fn get_window(&self, id: &WindowId) -> Option<&Window> { self.0.get_window(id) }

    pub fn get_window_mut(&mut self, id: &WindowId) -> Option<&mut Window> {
        self.0.get_window_mut(id)
    }

    pub fn run(mut self, handler: impl EventHandler + 'static) { self.0.run(handler); }
}
