use core_graphics::display::CGDisplay;
use dpi::{LogicalPosition, LogicalSize, Pixel};
use objc2_app_kit::{NSWindow, NSWindowStyleMask};
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
pub(super) fn to_b3_position(window: &NSWindow) -> CGPoint {
    let frame = window.frame();
    let screen = window.screen();
    let screen_height = match screen {
        Some(screen) => screen.frame().size.height,
        None => CGDisplay::main().bounds().size.height,
    };
    CGPoint::new(
        frame.origin.x,
        screen_height - frame.origin.y - frame.size.height,
    )
}

#[inline]
pub(super) fn to_macos_coords(position: LogicalPosition<f64>, window: &NSWindow) -> CGPoint {
    let size = window.frame().size;
    let screen = window.screen();
    let screen_height = match screen {
        Some(screen) => screen.frame().size.height,
        None => CGDisplay::main().bounds().size.height,
    };
    CGPoint::new(position.x, screen_height + position.y - size.height)
}
