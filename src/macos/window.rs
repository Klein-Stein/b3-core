use objc2::{
    rc::{autoreleasepool, Id},
    runtime::ProtocolObject,
};
use objc2_app_kit::{
    NSBackingStoreType,
    NSFullScreenWindowMask,
    NSWindow,
    NSWindowButton,
    NSWindowStyleMask,
    NSWindowTitleVisibility,
};
use objc2_foundation::{CGFloat, CGPoint, CGRect, CGSize, MainThreadMarker, NSRect, NSString};

use super::window_delegate::WindowDelegate;
use crate::{
    platform::WindowApi,
    ActiveApplication,
    Event,
    InitMode,
    Point,
    Size,
    WindowEvent,
    WindowId,
    WindowOptions,
};

#[derive(Debug)]
pub(crate) struct WindowImpl {
    pub(super) init_mode: InitMode,
    pub(super) delegate:  Id<WindowDelegate>,
    pub(super) native:    Id<NSWindow>,
}

impl WindowImpl {
    #[inline]
    pub(crate) fn new(mode: InitMode, options: Option<WindowOptions>, size: Size) -> Self {
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

        match mode {
            InitMode::Minimized => native.miniaturize(None),
            InitMode::Maximized => native.zoom(None),
            _ => (),
        }

        Self {
            init_mode: mode,
            delegate,
            native,
        }
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
    fn init(&mut self, window_id: WindowId) { self.delegate.set_window_id(window_id); }

    #[inline]
    fn set_title(&mut self, title: String) {
        let title = NSString::from_str(&title);
        self.native.setTitle(&title);
    }

    #[inline]
    fn title(&self) -> String {
        let title = self.native.title();
        title.to_string()
    }

    #[inline]
    fn set_options(&mut self, options: WindowOptions) {
        let mask = Self::to_window_style_mask(&options);
        self.native.setStyleMask(mask);
        let title_visibility = if options.borderless {
            NSWindowTitleVisibility::NSWindowTitleHidden
        } else {
            NSWindowTitleVisibility::NSWindowTitleVisible
        };
        self.native.setTitleVisibility(title_visibility);
        self.native
            .setTitlebarAppearsTransparent(options.borderless);
        if let Some(button) = self
            .native
            .standardWindowButton(NSWindowButton::NSWindowZoomButton)
        {
            button.setEnabled(options.fullscreen);
        }
    }

    #[inline]
    fn options(&self) -> WindowOptions {
        let mask = self.native.styleMask();
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
    fn show(&mut self, app: &ActiveApplication) {
        self.delegate.set_app_delegate(app.0.delegate.clone());

        self.native.makeKeyAndOrderFront(None);

        let window_id = self.delegate.window_id();
        self.delegate
            .handle_event(Event::Window(WindowEvent::Show, window_id));

        if self.init_mode == InitMode::Fullscreen {
            self.native.toggleFullScreen(None);
        }
    }

    #[inline]
    fn toggle_fullscreen(&mut self) { self.native.toggleFullScreen(None); }

    #[inline]
    fn is_fullscreen(&self) -> bool {
        (self.native.styleMask().0 & NSFullScreenWindowMask.0) == NSFullScreenWindowMask.0
    }

    #[inline]
    fn set_frame_size(&mut self, size: Size) {
        let origin = self.native.frame().origin;
        let frame = CGRect::new(origin, CGSize::new(size.width as f64, size.height as f64));
        unsafe { self.native.setFrame_display_animate(frame, true, false) };
    }

    #[inline]
    fn frame_size(&self) -> Size {
        let raw_size = self.native.frame().size;
        Size::new(raw_size.width as usize, raw_size.height as usize)
    }

    #[inline]
    fn set_position(&mut self, position: crate::Point) {
        let origin = CGPoint::new(position.x as f64, position.y as f64);
        unsafe { self.native.setFrameOrigin(origin) };
    }

    #[inline]
    fn position(&self) -> crate::Point {
        let raw_origin = self.native.frame().origin;
        Point::new(raw_origin.x as i32, raw_origin.y as i32)
    }
}
