//! This module contains a platform independent Application implementation.

use crate::{
    macos::ApplicationImpl,
    platform::ApplicationHandler,
    EventHandler,
    Menu,
    Window,
    WindowId,
};

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

    /// Sets an application menu.
    ///
    /// The applicaiton menu will only be displayed if the target system
    /// supports this kind of menu.
    ///
    /// # Parameters:
    /// * `menu` - Application menu (optional).
    pub fn set_menu(&mut self, menu: Option<Menu>) { self.0.set_menu(menu); }

    /// Registers a new window in the applicaiton.
    ///
    /// # Parameters:
    /// * `window` - [Window].
    pub fn add_window(&mut self, window: Window) { self.0.add_window(window); }

    /// Returns a registered window by [WindowId] or `None`.
    ///
    /// # Parameters:
    /// * `id` - [WindowId].
    pub fn get_window(&self, id: &WindowId) -> Option<&Window> { self.0.get_window(id) }

    /// Returns a mutable registered window by [WindowId] or `None`.
    ///
    /// # Parameters:
    /// * `id` - [WindowId].
    pub fn get_window_mut(&mut self, id: &WindowId) -> Option<&mut Window> {
        self.0.get_window_mut(id)
    }

    /// Runs an application (event loop).
    ///
    /// # Parameters:
    /// * `handler` - Event handler.
    pub fn run(mut self, handler: impl EventHandler + 'static) { self.0.run(handler); }
}
