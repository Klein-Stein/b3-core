use dpi::{LogicalPosition, LogicalSize, Pixel};
use objc2_app_kit::NSWindowStyleMask;
use objc2_foundation::{CGPoint, CGSize};

use crate::WindowOptions;

impl Into<NSWindowStyleMask> for WindowOptions {
    fn into(self) -> NSWindowStyleMask {
        let mut mask: usize = 0;
        if self.titled {
            mask |= NSWindowStyleMask::Titled.0;
        }
        if self.closable {
            mask |= NSWindowStyleMask::Closable.0;
        }
        if self.minimizable {
            mask |= NSWindowStyleMask::Miniaturizable.0;
        }
        if self.resizable {
            mask |= NSWindowStyleMask::Resizable.0;
        }
        if self.borderless {
            mask |= NSWindowStyleMask::Borderless.0 | NSWindowStyleMask::FullSizeContentView.0;
        }
        NSWindowStyleMask(mask)
    }
}

#[inline]
pub(super) fn to_cgsize<P: Pixel>(size: LogicalSize<P>) -> CGSize {
    CGSize::new(size.width.into(), size.height.into())
}

#[inline]
pub(super) fn to_cgpoint<P: Pixel>(position: LogicalPosition<P>) -> CGPoint {
    CGPoint::new(position.x.into(), position.y.into())
}
