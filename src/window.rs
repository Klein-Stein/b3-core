use crate::{macos::WindowImpl, platform::WindowHandler, Application};

const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 600;

#[derive(Debug)]
pub struct Size {
    pub width:  usize,
    pub height: usize,
}

impl Size {
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

pub type WindowId = usize;

#[derive(Debug)]
pub struct Window {
    window_impl: WindowImpl,
}

impl Window {
    pub fn builder() -> WindowBuilder { WindowBuilder::new() }

    #[inline]
    fn new(app: &Application, size: Size) -> Self {
        Self {
            window_impl: WindowImpl::new(app, size),
        }
    }

    pub fn set_title(&mut self, title: String) { self.window_impl.set_title(title); }

    pub fn title(&self) -> String { self.window_impl.title() }

    pub fn id(&self) -> WindowId { self as *const Self as WindowId }

    pub fn show(&mut self) { self.window_impl.show(); }
}

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

    pub fn with_title<S>(mut self, title: S) -> WindowBuilder
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    pub fn with_size(mut self, size: Size) -> WindowBuilder {
        self.size = size;
        self
    }

    pub fn build(self, app: &Application) -> Window {
        let mut window = Window::new(app, self.size);

        if let Some(title) = self.title {
            window.set_title(title);
        }

        window
    }
}
