use crate::{macos::WindowImpl, platform::WindowApi};

const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 600;

/// Window frame size.
///
/// By default this size defines a 800x600 px frame
#[derive(Debug)]
pub struct Size {
    /// Window frame width.
    pub width:  usize,
    /// Window frame height.
    pub height: usize,
}

impl Size {
    /// Creates a new instance of the window frame size.
    ///
    /// # Parameters:
    /// * `width` - Width.
    /// * `height` - Height.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width:  DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }
}

/// Window options.
#[derive(Debug, Default, PartialEq, Eq, Hash)]
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

/// Window ID.
pub type WindowId = usize;

/// Applcation window.
#[derive(Debug)]
pub struct Window(WindowImpl);

impl Window {
    /// Returns a new builder instance.
    pub fn builder() -> WindowBuilder { WindowBuilder::new() }

    fn new(options: Option<WindowOptions>, size: Size) -> Self {
        Self(WindowImpl::new(options, size))
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
    pub fn show(&mut self) { self.0.show(); }
}

/// Window builder.
#[derive(Default)]
pub struct WindowBuilder {
    title: Option<String>,
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
        let mut window = Window::new(self.flags, self.size);

        if let Some(title) = self.title {
            window.set_title(title);
        }

        window
    }
}
