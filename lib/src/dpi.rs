//! This module contains platform-independent implementations of size and point
//! structures.
//!
//! Some implementations are based on [1]
//! [1]:

/// This trait represents a physical or logical pixel.
pub trait Pixel: Copy + Into<f64> {
    /// Creates a new pixel from a [f64] value.
    ///
    /// # Parameters:
    /// * `value` - Source value.
    fn from_f64(value: f64) -> Self;

    /// Casts the pixel from source type to destination type.
    fn cast<P: Pixel>(self) -> P { P::from_f64(self.into()) }
}

impl Pixel for u8 {
    fn from_f64(value: f64) -> Self { value.round() as u8 }
}

impl Pixel for u16 {
    fn from_f64(value: f64) -> Self { value.round() as u16 }
}

impl Pixel for u32 {
    fn from_f64(value: f64) -> Self { value.round() as u32 }
}

impl Pixel for i8 {
    fn from_f64(value: f64) -> Self { value.round() as i8 }
}

impl Pixel for i16 {
    fn from_f64(value: f64) -> Self { value.round() as i16 }
}

impl Pixel for i32 {
    fn from_f64(value: f64) -> Self { value.round() as i32 }
}

impl Pixel for f32 {
    fn from_f64(value: f64) -> Self { value as f32 }
}

impl Pixel for f64 {
    fn from_f64(value: f64) -> Self { value }
}

/// Checks that the scale factor is a normal positive `f64`.
///
/// All functions that take a scale factor assert that this will return `true`. If you're sourcing
/// scale factors from anywhere other than winit, it's recommended to validate them using this
/// function before passing them to winit; otherwise, you risk panics.
#[inline]
pub fn validate_scale_factor(scale_factor: f64) -> bool {
    scale_factor.is_sign_positive() && scale_factor.is_normal()
}

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

/// Logical window frame size.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicalSize<P> {
    /// Logical window frame width.
    pub width:  P,
    /// Logical window frame height.
    pub height: P,
}

impl<P> LogicalSize<P> {
    /// Creates a new instance of the logical window frame size.
    ///
    /// # Parameters:
    /// * `width` - Logical width.
    /// * `height` - Logical height.
    #[inline]
    pub fn new(width: P, height: P) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl<P: Pixel> LogicalSize<P> {
    /// Creates a new logical size from physical one.
    ///
    /// # Parameters:
    /// * `size` - Physical size.
    /// * `scale_factor` - Backing scale factor.
    #[inline]
    pub fn from_physical<S, X>(size: S, scale_factor: f64) -> Self
    where
        S: Into<PhysicalSize<X>>,
        X: Pixel,
    {
        size.into().to_logical(scale_factor)
    }

    /// Converts the logical size into physical one.
    ///
    /// # Parameters:
    /// * `scale_factor` - Backing scale factor.
    #[inline]
    pub fn to_physical<X: Pixel>(&self, scale_factor: f64) -> PhysicalSize<X> {
        assert!(validate_scale_factor(scale_factor));
        let width = self.width.into() * scale_factor;
        let height = self.height.into() * scale_factor;
        PhysicalSize::new(width, height).cast()
    }

    /// Casts the logical size from source type to destination type.
    #[inline]
    pub fn cast<X: Pixel>(&self) -> LogicalSize<X> {
        LogicalSize {
            width:  self.width.cast(),
            height: self.height.cast(),
        }
    }
}

impl<P: Pixel, X: Pixel> From<(X, X)> for LogicalSize<P> {
    fn from((x, y): (X, X)) -> LogicalSize<P> { LogicalSize::new(x.cast(), y.cast()) }
}

impl<P: Pixel, X: Pixel> From<LogicalSize<P>> for (X, X) {
    fn from(s: LogicalSize<P>) -> (X, X) { (s.width.cast(), s.height.cast()) }
}

impl<P: Pixel, X: Pixel> From<[X; 2]> for LogicalSize<P> {
    fn from([x, y]: [X; 2]) -> LogicalSize<P> { LogicalSize::new(x.cast(), y.cast()) }
}

impl<P: Pixel, X: Pixel> From<LogicalSize<P>> for [X; 2] {
    fn from(s: LogicalSize<P>) -> [X; 2] { [s.width.cast(), s.height.cast()] }
}

/// Physical window frame size.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalSize<P> {
    /// Physical window frame width.
    pub width:  P,
    /// Physical window frame height.
    pub height: P,
}

impl<P> PhysicalSize<P> {
    /// Creates a new instance of the physical window frame size.
    ///
    /// # Parameters:
    /// * `width` - Physical width.
    /// * `height` - Physical height.
    #[inline]
    pub fn new(width: P, height: P) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl<P: Pixel> PhysicalSize<P> {
    /// Creates a new physical size from logical one.
    ///
    /// # Parameters:
    /// * `size` - Physical size.
    /// * `scale_factor` - Backing scale factor.
    #[inline]
    pub fn from_logical<S, X>(size: S, scale_factor: f64) -> Self
    where
        S: Into<LogicalSize<X>>,
        X: Pixel,
    {
        size.into().to_physical(scale_factor)
    }

    /// Converts the physical size into logical one.
    ///
    /// # Parameters:
    /// * `scale_factor` - Backing scale factor.
    #[inline]
    pub fn to_logical<X: Pixel>(&self, scale_factor: f64) -> LogicalSize<X> {
        assert!(validate_scale_factor(scale_factor));
        let width = self.width.into() / scale_factor;
        let height = self.height.into() / scale_factor;
        LogicalSize::new(width, height).cast()
    }

    /// Casts the physical size from source type to destination type.
    #[inline]
    pub fn cast<X: Pixel>(&self) -> PhysicalSize<X> {
        PhysicalSize {
            width:  self.width.cast(),
            height: self.height.cast(),
        }
    }
}

impl<P: Pixel, X: Pixel> From<(X, X)> for PhysicalSize<P> {
    fn from((x, y): (X, X)) -> PhysicalSize<P> { PhysicalSize::new(x.cast(), y.cast()) }
}

impl<P: Pixel, X: Pixel> From<PhysicalSize<P>> for (X, X) {
    fn from(s: PhysicalSize<P>) -> (X, X) { (s.width.cast(), s.height.cast()) }
}

impl<P: Pixel, X: Pixel> From<[X; 2]> for PhysicalSize<P> {
    fn from([x, y]: [X; 2]) -> PhysicalSize<P> { PhysicalSize::new(x.cast(), y.cast()) }
}

impl<P: Pixel, X: Pixel> From<PhysicalSize<P>> for [X; 2] {
    fn from(s: PhysicalSize<P>) -> [X; 2] { [s.width.cast(), s.height.cast()] }
}

/// Window frame size.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Size {
    /// Logical size.
    Logical(LogicalSize<f64>),
    /// Physical size.
    Physical(PhysicalSize<u32>),
}

impl Size {
    /// Creates a new instance of the logical window frame size.
    ///
    /// # Parameters:
    /// * `width` - Width.
    /// * `height` - Height.
    pub fn new_logical<T>(width: T, height: T) -> Self
    where
        T: Into<f64>,
    {
        Self::Logical(LogicalSize::new(width.into(), height.into()))
    }

    /// Creates a new instance of the physical window frame size.
    ///
    /// # Parameters:
    /// * `width` - Width.
    /// * `height` - Height.
    pub fn new_physical<T>(width: T, height: T) -> Self
    where
        T: Into<u32>,
    {
        Self::Physical(PhysicalSize::new(width.into(), height.into()))
    }
}
