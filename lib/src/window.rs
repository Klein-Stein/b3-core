#[cfg(feature = "dh")]
use b3_display_handler::{HasWindowHandler, WindowHandler};

use crate::{
    platform::{WindowApi, Wrapper},
    platform_impl::WindowImpl,
    ActiveApplication,
    ContextOwner,
    Point,
    Size,
};

/// Window options.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct WindowOptions {
    /// Turn on/off a window title.
    pub titled:      bool,
    /// Allow a window to be minimized.
    pub minimizable: bool,
    /// Allow a window to be closed.
    pub closable:    bool,
    /// Allow a window to be resized.
    pub resizable:   bool,
    /// Allow a window to be dragged.
    pub draggable:   bool,
    /// Allow a window to be switched to a fullscreen mode.
    pub fullscreen:  bool,
    /// Show/hide a window borders.
    pub borderless:  bool,
}

/// Initial mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InitMode {
    /// A regular window with a specified frame size.
    Default,
    /// The window will be displayed in minimized mode.
    Minimized,
    /// The window will be displayed in maximized mode.
    Maximized,
    /// The window will be displayed in fullscreen mode.
    Fullscreen,
}

impl Default for InitMode {
    fn default() -> Self { Self::Default }
}

/// Window ID.
pub type WindowId = usize;

/// Applcation window.
#[derive(Debug)]
pub struct Window(WindowImpl);

impl Window {
    /// Returns a new builder instance.
    pub fn builder() -> WindowBuilder { WindowBuilder::new() }

    fn new(
        ctx: &impl ContextOwner,
        mode: InitMode,
        options: Option<WindowOptions>,
        size: Size,
    ) -> Self {
        let mut window = Self(WindowImpl::new(ctx, mode, options, size));
        window.0.init(window.id());
        window
    }

    /// Sets a window title.
    ///
    /// # Parameters:
    /// * `title` - Window title.
    pub fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>,
    {
        self.0.set_title(title.into());
    }

    /// Returns a window title.
    pub fn title(&self) -> String { self.0.title() }

    /// Returns a window ID.
    pub fn id(&self) -> WindowId { self as *const Self as WindowId }

    /// Sets window options.
    ///
    /// # Parameters:
    /// * `options` - Window options.
    pub fn set_options(&mut self, options: WindowOptions) { self.0.set_options(options); }

    /// Retuns window options.
    pub fn options(&self) -> WindowOptions { self.0.options() }

    /// Makes a window visible.
    ///
    /// # Parameters:
    /// * `app` - Active application.
    pub fn show(&mut self, app: &ActiveApplication) { self.0.show(app); }

    /// Makes a window visible.
    ///
    /// # Parameters:
    /// * `app` - Active application.
    pub fn show_modal(&mut self, app: &ActiveApplication) { self.0.show_modal(app); }

    /// Toggles the fullscreen mode of the window.
    pub fn toggle_fullscreen(&mut self) { self.0.toggle_fullscreen(); }

    /// Returns if a window is in the fullscreen mode.
    pub fn is_fullscreen(&self) -> bool { self.0.is_fullscreen() }

    /// Sets a new frame size of the window.
    ///
    /// # Parameters:
    /// * `size` - Window frame size.
    pub fn set_frame_size(&mut self, size: Size) { self.0.set_frame_size(size); }

    /// Returns a frame size of the window.
    pub fn frame_size(&self) -> Size { self.0.frame_size() }

    /// Sets a window origin position.
    ///
    /// # Parameters:
    /// * `position` - Origin position.
    pub fn set_position(&mut self, position: Point) { self.0.set_position(position); }

    /// Returns a window origin position.
    pub fn position(&self) -> Point { self.0.position() }

    /// Sets a minimal size of the window frame.
    ///
    /// # Parameters:
    /// * `min_size` - Minimal window frame size.
    pub fn set_min_size(&mut self, min_size: Size) { self.0.set_min_size(min_size); }

    /// Returns a minimal size of the window frame.
    pub fn min_size(&self) -> Size { self.0.min_size() }

    /// Sets a maximal size of the window frame.
    ///
    /// # Parameters:
    /// * `max_size` - Maximal window frame size.
    pub fn set_max_size(&mut self, max_size: Size) { self.0.set_max_size(max_size); }

    /// Returns a maximal size of the window frame.
    pub fn max_size(&self) -> Size { self.0.max_size() }

    /// Switches a window into the maximized mode.
    pub fn maximize(&mut self) { self.0.maximize() }

    /// Checks if a window is maximized.
    pub fn is_maximized(&self) -> bool { self.0.is_maximized() }

    /// Returns a content size (inner window size).
    pub fn content_size(&self) -> Size { self.0.content_size() }

    /// Checks if a window is visible on the screen.
    pub fn is_visible(&self) -> bool { self.0.is_visible() }

    /// Closes a window.
    pub fn close(&mut self) { self.0.close(); }

    /// Switches a window into the minimized mode.
    pub fn minimize(&mut self) { self.0.minimize(); }

    /// Checks if a window is minimized.
    pub fn is_minimized(&self) -> bool { self.0.is_minimized() }

    /// De-minimizes window.
    pub fn restore(&mut self) { self.0.restore(); }
}

impl Wrapper<WindowImpl> for Window {
    #[inline]
    fn get_impl(&self) -> &WindowImpl { &self.0 }

    #[inline]
    fn get_impl_mut(&mut self) -> &mut WindowImpl { &mut self.0 }
}

#[cfg(feature = "dh")]
impl HasWindowHandler for Window {
    fn window_handler(&self) -> WindowHandler { self.0.window_handler() }
}

/// Window builder.
#[derive(Default)]
pub struct WindowBuilder {
    title: Option<String>,
    mode:  InitMode,
    flags: Option<WindowOptions>,
    size:  Size,
}

impl WindowBuilder {
    #[inline]
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Sets a title of the window under building.
    ///
    /// # Parameters:
    /// * `title` - Window title.
    pub fn with_title<S>(mut self, title: S) -> WindowBuilder
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    /// Sets an initial mode of the window under building.
    ///
    /// # Parameters:
    /// * `mode` - Initial mode.
    pub fn with_init_mode(mut self, mode: InitMode) -> WindowBuilder {
        self.mode = mode;
        self
    }

    /// Sets a frame size of the window under building.
    ///
    /// # Parameters:
    /// * `size` - Window frame size.
    pub fn with_size(mut self, size: Size) -> WindowBuilder {
        self.size = size;
        self
    }

    /// Sets options of the window under building.
    ///
    /// # Parameters:
    /// * `options` - Window options.
    pub fn with_options(mut self, options: WindowOptions) -> WindowBuilder {
        self.flags = Some(options);
        self
    }

    /// Builds a new window instance with passed parameters.
    ///
    /// # Parameters:
    /// * `ctx` - Context onwer.
    pub fn build(self, ctx: &impl ContextOwner) -> Window {
        let mut window = Window::new(ctx, self.mode, self.flags, self.size);

        if let Some(title) = self.title {
            window.set_title(title);
        }

        window
    }
}
