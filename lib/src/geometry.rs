const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 600;

/// 2D integer point
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    /// X-coordinate.
    pub x: i32,
    /// Y-coordinate.
    pub y: i32,
}

impl Point {
    /// Creates a new point.
    ///
    /// # Parameters:
    /// * `x` - X-coordinate.
    /// * `y` - Y-coordinate.
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

/// Window frame size.
///
/// By default this size defines a 800x600 px frame
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
