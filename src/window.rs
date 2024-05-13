use crate::{macos::WindowImpl, platform::WindowApi, ActiveApplication, Point, Size};

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
    /// Allow a window to be switched to a fullscreen mode.
    pub fullscreen:  bool,
    /// Show/hide a window borders.
    pub borderless:  bool,
}

/// Initial mode
#[derive(Debug, PartialEq, Eq, Hash)]
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

    fn new(mode: InitMode, options: Option<WindowOptions>, size: Size) -> Self {
        let mut window = Self(WindowImpl::new(mode, options, size));
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

    /// Toggles the fullscreen mode of the window.
    fn toggle_fullscreen(&mut self) { self.0.toggle_fullscreen(); }

    /// Returns if a window is in the fullscreen mode.
    fn is_fullscreen(&self) -> bool { self.0.is_fullscreen() }

    /// Sets a new frame size of the window.
    ///
    /// # Parameters:
    /// * `size` - Window frame size.
    fn set_frame_size(&mut self, size: Size) { self.0.set_frame_size(size); }

    /// Returns a frame size of the window.
    fn frame_size(&self) -> Size { self.0.frame_size() }

    /// Sets a window origin position.
    ///
    /// # Parameters:
    /// * `position` - Origin position.
    fn set_position(&mut self, position: Point) { self.0.set_position(position); }

    /// Returns a window origin position.
    fn position(&self) -> Point { self.0.position() }
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
    pub fn build(self) -> Window {
        let mut window = Window::new(self.mode, self.flags, self.size);

        if let Some(title) = self.title {
            window.set_title(title);
        }

        window
    }
}
