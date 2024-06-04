use objc2_app_kit::NSWindowStyleMask;
use objc2_foundation::CGSize;

use crate::{LogicalSize, Pixel, WindowOptions};

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

impl<U: Pixel> Into<CGSize> for LogicalSize<U> {
    fn into(self) -> CGSize { CGSize::new(self.width.into(), self.height.into()) }
}
