use crate::{macos::ApplicationImpl, platform::ApplicationHandler, Menu};

pub struct Application {
    pub(crate) application_impl: ApplicationImpl,
}

impl Application {
    pub fn new() -> Self {
        Self {
            application_impl: ApplicationImpl::new(),
        }
    }

    pub fn set_menu(&mut self, menu: Option<Menu>) { self.application_impl.set_menu(menu); }

    pub fn run(self) { self.application_impl.run(); }
}
