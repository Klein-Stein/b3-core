use crate::{macos::WindowImpl, platform::WindowHandler, Application};

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

/// Window ID.
pub type WindowId = usize;

/// Applcation window.
#[derive(Debug)]
pub struct Window {
    window_impl: WindowImpl,
}

impl Window {
    /// Returns a new builder instance.
    pub fn builder() -> WindowBuilder { WindowBuilder::new() }

    #[inline]
    fn new(app: &Application, size: Size) -> Self {
        Self {
            window_impl: WindowImpl::new(app, size),
        }
    }

    /// Sets a window title.
    ///
    /// # Parameters:
    /// * `title` - Window title.
    pub fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>,
    {
        self.window_impl.set_title(title.into());
    }

    /// Returns a window title.
    pub fn title(&self) -> String { self.window_impl.title() }

    /// Returns a window ID.
    pub fn id(&self) -> WindowId { self as *const Self as WindowId }

    /// Makes a window visible.
    pub fn show(&mut self) { self.window_impl.show(); }
}

/// Window builder.
#[derive(Default)]
pub struct WindowBuilder {
    title: Option<String>,
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

    /// Builds a new window instance with passed parameters.
    ///
    /// # Parameters:
    /// * `app` - Current application.
    pub fn build(self, app: &Application) -> Window {
        let mut window = Window::new(app, self.size);

        if let Some(title) = self.title {
            window.set_title(title);
        }

        window
    }
}
