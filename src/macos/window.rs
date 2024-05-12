use objc2::{
    rc::{autoreleasepool, Id},
    runtime::ProtocolObject,
};
use objc2_app_kit::{
    NSBackingStoreType,
    NSWindow,
    NSWindowButton,
    NSWindowStyleMask,
    NSWindowTitleVisibility,
};
use objc2_foundation::{CGFloat, CGPoint, CGSize, MainThreadMarker, NSRect, NSString};

use super::window_delegate::WindowDelegate;
use crate::{platform::WindowApi, Size, WindowOptions};

#[derive(Debug)]
pub(crate) struct WindowImpl(pub(super) Id<NSWindow>);

impl WindowImpl {
    #[inline]
    pub(crate) fn new(options: Option<WindowOptions>, size: Size) -> Self {
        let style = if let Some(options) = &options {
            Self::to_window_style_mask(&options)
        } else {
            NSWindowStyleMask(
                NSWindowStyleMask::Titled.0
                    | NSWindowStyleMask::Resizable.0
                    | NSWindowStyleMask::Closable.0
                    | NSWindowStyleMask::Miniaturizable.0,
            )
        };

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

        if let Some(options) = &options {
            let title_visibility = if options.borderless {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            };
            native.setTitleVisibility(title_visibility);
            native.setTitlebarAppearsTransparent(options.borderless);

            if let Some(button) = native.standardWindowButton(NSWindowButton::NSWindowZoomButton) {
                button.setEnabled(options.fullscreen);
            }
        }

        Self(native)
    }

    fn to_window_style_mask(options: &WindowOptions) -> NSWindowStyleMask {
        let mut mask: usize = 0;
        if options.titled {
            mask |= NSWindowStyleMask::Titled.0;
        }
        if options.closable {
            mask |= NSWindowStyleMask::Closable.0;
        }
        if options.minimizable {
            mask |= NSWindowStyleMask::Miniaturizable.0;
        }
        if options.resizable {
            mask |= NSWindowStyleMask::Resizable.0;
        }
        if options.borderless {
            mask |= NSWindowStyleMask::Borderless.0 | NSWindowStyleMask::FullSizeContentView.0;
        }
        NSWindowStyleMask(mask)
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
    fn set_options(&mut self, options: WindowOptions) {
        let mask = Self::to_window_style_mask(&options);
        self.0.setStyleMask(mask);
        let title_visibility = if options.borderless {
            NSWindowTitleVisibility::NSWindowTitleHidden
        } else {
            NSWindowTitleVisibility::NSWindowTitleVisible
        };
        self.0.setTitleVisibility(title_visibility);
        self.0.setTitlebarAppearsTransparent(options.borderless);
        if let Some(button) = self
            .0
            .standardWindowButton(NSWindowButton::NSWindowZoomButton)
        {
            button.setEnabled(options.fullscreen);
        }
    }

    #[inline]
    fn options(&self) -> WindowOptions {
        let mask = self.0.styleMask();
        WindowOptions {
            titled:      (mask.0 & NSWindowStyleMask::Titled.0) != 0,
            minimizable: (mask.0 & NSWindowStyleMask::Miniaturizable.0) != 0,
            closable:    (mask.0 & NSWindowStyleMask::Closable.0) != 0,
            resizable:   (mask.0 & NSWindowStyleMask::Resizable.0) != 0,
            fullscreen:  (mask.0 & NSWindowStyleMask::FullScreen.0) != 0,
            borderless:  (mask.0 & NSWindowStyleMask::Borderless.0) != 0,
        }
    }

    #[inline]
    fn show(&mut self) { self.0.makeKeyAndOrderFront(None); }
}
