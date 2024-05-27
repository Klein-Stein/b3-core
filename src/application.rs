//! This module contains a platform independent Application implementation.
use crate::{
    platform::{ActiveApplicationApi, ApplicationApi, Wrapper},
    platform_impl::{ActiveApplicationImpl, ApplicationImpl, ContextImpl},
    Error,
    EventHandler,
    Icon,
    Menu,
};

/// Native context.
#[derive(Debug)]
pub struct Context(ContextImpl);

impl Context {
    pub(crate) fn new(ctx: ContextImpl) -> Self { Self(ctx) }
}

impl Wrapper<ContextImpl> for Context {
    #[inline]
    fn get_impl(&self) -> &ContextImpl { &self.0 }

    #[inline]
    fn get_impl_mut(&mut self) -> &mut ContextImpl { &mut self.0 }
}

/// Context owner.
pub trait ContextOwner {
    /// Returns a system context.
    fn context(&self) -> &Context;
}

/// This structure represents a platform independent running application.
#[derive(Debug)]
pub struct ActiveApplication(ActiveApplicationImpl);

impl ActiveApplication {
    pub(crate) fn new(app_impl: ActiveApplicationImpl) -> Self { Self(app_impl) }

    /// Sets an application icon.
    ///
    /// # Parameters:
    /// * `icon` - Icon.
    pub fn set_icon(&mut self, icon: Option<&Icon>) { self.0.set_icon(icon); }

    /// Sets an application menu.
    ///
    /// # Parameters:
    /// * `menu` - Application menu.
    pub fn set_menu(&mut self, menu: Option<&Menu>) { self.0.set_menu(menu); }

    /// Stops a running applicaiton.
    pub fn stop(&mut self) { self.0.stop(); }
}

impl Wrapper<ActiveApplicationImpl> for ActiveApplication {
    #[inline]
    fn get_impl(&self) -> &ActiveApplicationImpl { &self.0 }

    #[inline]
    fn get_impl_mut(&mut self) -> &mut ActiveApplicationImpl { &mut self.0 }
}

impl ContextOwner for ActiveApplication {
    fn context(&self) -> &Context { self.0.context() }
}

unsafe impl Sync for ActiveApplication {}

/// The main entity that provides entrypoints to the event loop and other API.
///
/// Any program that uses the **b3-core** crate must create an instance of
/// this structure before using any other crate's entities.
#[derive(Debug)]
pub struct Application(ApplicationImpl);

impl Application {
    /// Creates a new [Application] instance.
    ///
    /// # Examples:
    ///
    /// ```rust
    /// use b3_core::Application;
    ///
    /// let app = Application::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, Error> { Ok(Self(ApplicationImpl::new()?)) }

    /// Runs an application (event loop).
    ///
    /// # Parameters:
    /// * `handler` - Event handler.
    pub fn run(mut self, handler: impl EventHandler + 'static) { self.0.run(handler); }
}

impl Wrapper<ApplicationImpl> for Application {
    #[inline]
    fn get_impl(&self) -> &ApplicationImpl { &self.0 }

    #[inline]
    fn get_impl_mut(&mut self) -> &mut ApplicationImpl { &mut self.0 }
}

impl ContextOwner for Application {
    fn context(&self) -> &Context { self.0.context() }
}
