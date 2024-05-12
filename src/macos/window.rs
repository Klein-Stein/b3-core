use objc2::{
    rc::{autoreleasepool, Id},
    runtime::ProtocolObject,
};
use objc2_app_kit::{NSBackingStoreType, NSWindow, NSWindowStyleMask};
use objc2_foundation::{CGFloat, CGPoint, CGSize, MainThreadMarker, NSRect, NSString};

use super::window_delegate::WindowDelegate;
use crate::{platform::WindowApi, Size};

#[derive(Debug)]
pub(crate) struct WindowImpl(pub(super) Id<NSWindow>);

impl WindowImpl {
    #[inline]
    pub(crate) fn new(size: Size) -> Self {
        let style = NSWindowStyleMask(
            NSWindowStyleMask::Titled.0
                | NSWindowStyleMask::Resizable.0
                | NSWindowStyleMask::Closable.0
                | NSWindowStyleMask::Miniaturizable.0,
        );

        let content_rect = NSRect::new(
            CGPoint::new(200.0, 200.0),
            CGSize::new(size.width as CGFloat, size.height as CGFloat),
        );

        let mtm: MainThreadMarker = MainThreadMarker::new()
            .expect("on macOS, `WindowImpl` instance must be created on the main thread!");
        let this = mtm.alloc();
        let native = unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                this,
                content_rect,
                style,
                NSBackingStoreType::NSBackingStoreBuffered,
                false,
            )
        };

        let delegate = WindowDelegate::new(mtm);
        autoreleasepool(|_| {
            let object = ProtocolObject::from_ref(&*delegate);
            native.setDelegate(Some(object));
        });

        Self(native)
    }
}

impl WindowApi for WindowImpl {
    #[inline]
    fn set_title(&mut self, title: String) {
        let title = NSString::from_str(&title);
        self.0.setTitle(&title);
    }

    #[inline]
    fn title(&self) -> String {
        let title = self.0.title();
        title.to_string()
    }

    #[inline]
    fn show(&mut self) { self.0.makeKeyAndOrderFront(None); }
}
