use objc2::{
    rc::{autoreleasepool, Id},
    runtime::ProtocolObject,
};
use objc2_app_kit::{
    NSApp,
    NSBackingStoreType,
    NSFullScreenWindowMask,
    NSWindow,
    NSWindowButton,
    NSWindowStyleMask,
    NSWindowTitleVisibility,
};
use objc2_foundation::{CGFloat, CGPoint, CGRect, CGSize, MainThreadBound, NSRect, NSString};

use super::window_delegate::WindowDelegate;
use crate::{
    platform::{WindowApi, Wrapper},
    ActiveApplication,
    ContextOwner,
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
    delegate:  MainThreadBound<Id<WindowDelegate>>,
    native:    MainThreadBound<Id<NSWindow>>,
    init_mode: Option<InitMode>,
}

impl WindowImpl {
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

    fn native_on_main<F, R>(&self, f: F) -> R
    where
        F: Send + FnOnce(&Id<NSWindow>) -> R,
        R: Send,
    {
        self.native
            .get_on_main(|native| autoreleasepool(|_| f(native)))
    }

    fn delegate_on_main<F, R>(&self, f: F) -> R
    where
        F: Send + FnOnce(&Id<WindowDelegate>) -> R,
        R: Send,
    {
        self.delegate
            .get_on_main(|delegate| autoreleasepool(|_| f(delegate)))
    }
}

impl WindowApi for WindowImpl {
    #[inline]
    fn new(
        ctx: &impl ContextOwner,
        mode: InitMode,
        options: Option<WindowOptions>,
        size: Size,
    ) -> Self {
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

        let mtm = ctx.context().get_impl().mtm();
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

            if unsafe { native.isMovable() } != options.draggable {
                native.setMovable(options.draggable);
            }
        }

        match mode {
            InitMode::Minimized => native.miniaturize(None),
            InitMode::Maximized => native.zoom(None),
            _ => (),
        }

        Self {
            init_mode: Some(mode),
            delegate:  MainThreadBound::new(delegate, mtm),
            native:    MainThreadBound::new(native, mtm),
        }
    }

    #[inline]
    fn init(&mut self, window_id: WindowId) {
        self.delegate.get_on_main(|delegate| {
            delegate.set_window_id(window_id);
        });
    }

    #[inline]
    fn set_title(&mut self, title: String) {
        let title = NSString::from_str(&title);
        self.native_on_main(|native| {
            native.setTitle(&title);
        });
    }

    #[inline]
    fn title(&self) -> String {
        let title = self.native_on_main(|native| native.title());
        title.to_string()
    }

    #[inline]
    fn set_options(&mut self, options: WindowOptions) {
        let mask = Self::to_window_style_mask(&options);
        self.native_on_main(|native| {
            native.setStyleMask(mask);

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

            if unsafe { native.isMovable() } != options.draggable {
                native.setMovable(options.draggable);
            }
        });
    }

    #[inline]
    fn options(&self) -> WindowOptions {
        self.native_on_main(|native| {
            let mask = native.styleMask();
            WindowOptions {
                titled:      (mask.0 & NSWindowStyleMask::Titled.0) != 0,
                minimizable: (mask.0 & NSWindowStyleMask::Miniaturizable.0) != 0,
                closable:    (mask.0 & NSWindowStyleMask::Closable.0) != 0,
                resizable:   (mask.0 & NSWindowStyleMask::Resizable.0) != 0,
                draggable:   unsafe { native.isMovable() },
                fullscreen:  (mask.0 & NSWindowStyleMask::FullScreen.0) != 0,
                borderless:  (mask.0 & NSWindowStyleMask::Borderless.0) != 0,
            }
        })
    }

    #[inline]
    fn show(&mut self, app: &ActiveApplication) {
        self.native_on_main(|native| {
            native.makeKeyAndOrderFront(None);
        });

        self.delegate_on_main(|delegate| {
            delegate.set_app_delegate(app.get_impl().get_app_delegate().clone());
            let window_id = delegate.window_id();
            delegate.handle_event(Event::Window(WindowEvent::Show, window_id));
        });

        if self.init_mode == Some(InitMode::Fullscreen) {
            self.native_on_main(|native| {
                native.toggleFullScreen(None);
            });
            self.init_mode = None;
        }
    }

    #[inline]
    fn show_modal(&mut self, app: &ActiveApplication) {
        let mtm = app.context().get_impl().mtm();
        let ns_app = NSApp(mtm);
        let ns_window = self.native.get(mtm);
        unsafe { ns_app.runModalForWindow(ns_window) };

        self.delegate_on_main(|delegate| {
            delegate.set_app_delegate(app.get_impl().get_app_delegate().clone());
            let window_id = delegate.window_id();
            delegate.handle_event(Event::Window(WindowEvent::Show, window_id));
        });

        if self.init_mode == Some(InitMode::Fullscreen) {
            self.native_on_main(|native| {
                native.toggleFullScreen(None);
            });
            self.init_mode = None;
        }
    }

    #[inline]
    fn toggle_fullscreen(&mut self) {
        self.native_on_main(|native| {
            native.toggleFullScreen(None);
        });
    }

    #[inline]
    fn is_fullscreen(&self) -> bool {
        self.native_on_main(|native| {
            (native.styleMask().0 & NSFullScreenWindowMask.0) == NSFullScreenWindowMask.0
        })
    }

    #[inline]
    fn set_frame_size(&mut self, size: Size) {
        self.native_on_main(|native| {
            let origin = native.frame().origin;
            let frame = CGRect::new(origin, CGSize::new(size.width as f64, size.height as f64));
            unsafe { native.setFrame_display_animate(frame, true, false) };
        });
    }

    #[inline]
    fn frame_size(&self) -> Size {
        self.native_on_main(|native| {
            let raw_size = native.frame().size;
            Size::new(raw_size.width as usize, raw_size.height as usize)
        })
    }

    #[inline]
    fn set_position(&mut self, position: crate::Point) {
        self.native_on_main(|native| {
            let origin = CGPoint::new(position.x as f64, position.y as f64);
            unsafe { native.setFrameOrigin(origin) };
        });
    }

    #[inline]
    fn position(&self) -> crate::Point {
        self.native_on_main(|native| {
            let raw_origin = native.frame().origin;
            Point::new(raw_origin.x as i32, raw_origin.y as i32)
        })
    }

    #[inline]
    fn set_min_size(&mut self, min_size: Size) {
        self.native_on_main(|native| {
            let size = CGSize::new(min_size.width as f64, min_size.height as f64);
            native.setMinSize(size);
        });
    }

    #[inline]
    fn min_size(&self) -> Size {
        self.native_on_main(|native| {
            let min_size = unsafe { native.minSize() };
            Size::new(min_size.width as usize, min_size.height as usize)
        })
    }

    #[inline]
    fn set_max_size(&mut self, max_size: Size) {
        self.native_on_main(|native| {
            let size = CGSize::new(max_size.width as f64, max_size.height as f64);
            native.setMaxSize(size);
        });
    }

    #[inline]
    fn max_size(&self) -> Size {
        self.native_on_main(|native| {
            let max_size = unsafe { native.maxSize() };
            Size::new(max_size.width as usize, max_size.height as usize)
        })
    }

    #[inline]
    fn maximize(&mut self) {
        self.native_on_main(|native| {
            native.zoom(None);
        })
    }

    #[inline]
    fn is_maximized(&self) -> bool { self.native_on_main(|native| native.isZoomed()) }

    #[inline]
    fn content_size(&self) -> Size {
        self.native_on_main(|native| {
            let size = unsafe { native.contentLayoutRect().size };
            Size::new(size.width as usize, size.height as usize)
        })
    }

    #[inline]
    fn is_visible(&self) -> bool { self.native_on_main(|native| native.isVisible()) }

    #[inline]
    fn close(&mut self) {
        self.native_on_main(|native| {
            native.close();
        });
    }

    #[inline]
    fn minimize(&mut self) {
        self.native_on_main(|native| {
            native.miniaturize(None);
        });
    }

    #[inline]
    fn is_minimized(&self) -> bool { self.native_on_main(|native| native.isMiniaturized()) }

    #[inline]
    fn restore(&mut self) {
        self.native_on_main(|native| {
            if native.isMiniaturized() {
                unsafe { native.deminiaturize(None) };
            }
        });
    }
}
