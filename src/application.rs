//! This module contains a platform independent Application implementation.

use crate::{
    macos::{ActiveApplicationImpl, ApplicationImpl},
    platform::{ActiveApplicationApi, ApplicationApi},
    EventHandler,
    Menu,
};

/// This structure represents a platform independent running application.
#[derive(Debug)]
pub struct ActiveApplication(pub(crate) ActiveApplicationImpl);

impl ActiveApplication {
    pub(crate) fn new(app_impl: ActiveApplicationImpl) -> Self { Self(app_impl) }

    /// Sets an application menu.
    ///
    /// # Parameters:
    /// * `menu` - Application menu.
    pub fn set_menu(&mut self, menu: Option<&Menu>) { self.0.set_menu(menu); }

    /// Stops a running applicaiton.
    pub fn stop(&mut self) { self.0.stop(); }
}

/// The main entity that provides entrypoints to the event loop and other API.
///
/// Any program that uses the **b3-platform** crate must create an instance of
/// this structure before using any other crate's entities.
#[derive(Debug)]
pub struct Application(pub(crate) ApplicationImpl);

impl Application {
    /// Creates a new [Application] instance.
    ///
    /// # Examples:
    ///
    /// ```rust
    /// use b3_platform::Application;
    ///
    /// let app = Application::new();
    /// ```
    pub fn new() -> Self { Self(ApplicationImpl::new()) }

    /// Runs an application (event loop).
    ///
    /// # Parameters:
    /// * `handler` - Event handler.
    pub fn run(mut self, handler: impl EventHandler + 'static) { self.0.run(handler); }
}
