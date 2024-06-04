use std::ptr::NonNull;

#[cfg(feature = "dh")]
use b3_display_handler::{appkit::AppKitWindowHandler, HasWindowHandler, WindowHandler};
use dpi::{PhysicalPosition, PhysicalSize, Position, Size};
use objc2::{
    rc::{autoreleasepool, Retained},
    runtime::ProtocolObject,
};
use objc2_app_kit::{
    NSBackingStoreType,
    NSScreen,
    NSWindow,
    NSWindowButton,
    NSWindowStyleMask,
    NSWindowTitleVisibility,
};
use objc2_foundation::{CGPoint, CGSize, MainThreadBound, MainThreadMarker, NSRect};

use super::{view::View, window_delegate::WindowDelegate, window_utils::to_cgsize};
use crate::{
    platform::{WindowApi, Wrapper},
    ActiveApplication,
    ContextOwner,
    InitMode,
    WindowId,
    WindowOptions,
};

#[derive(Debug)]
pub(crate) struct WindowImpl {
    delegate: MainThreadBound<Retained<WindowDelegate>>,
    native:   MainThreadBound<Retained<NSWindow>>,
}

impl WindowImpl {
    #[inline]
    fn delegate_on_main<F, R>(&self, f: F) -> R
    where
        F: Send + FnOnce(&Retained<WindowDelegate>) -> R,
        R: Send,
    {
        self.delegate.get_on_main(f)
    }

    #[inline]
    fn get_native(&self, mtm: MainThreadMarker) -> &Retained<NSWindow> { self.native.get(mtm) }
}

impl WindowApi for WindowImpl {
    #[inline]
    fn new(
        ctx: &impl ContextOwner,
        mode: InitMode,
        options: Option<WindowOptions>,
        size: Option<Size>,
    ) -> Self {
        // Extract the application context
        let mtm = ctx.context().get_impl().mtm();
        let app_delegate = ctx.context().get_impl().app_delegate().clone();

        // Create NSWindow
        let style = if let Some(options) = &options {
            (*options).into()
        } else {
            NSWindowStyleMask(
                NSWindowStyleMask::Titled.0
                    | NSWindowStyleMask::Resizable.0
                    | NSWindowStyleMask::Closable.0
                    | NSWindowStyleMask::Miniaturizable.0,
            )
        };

        let scale_factor = NSScreen::mainScreen(mtm)
            .map(|screen| screen.backingScaleFactor() as f64)
            .unwrap_or(1.0);

        let cgsize = match size {
            Some(Size::Logical(size)) => to_cgsize(size),
            Some(Size::Physical(size)) => to_cgsize(size.to_logical::<f64>(scale_factor)),
            None => CGSize::new(800.0, 600.0),
        };

        let content_rect = NSRect::new(CGPoint::new(200.0, 200.0), cgsize);

        let this = mtm.alloc();
        let window = unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                this,
                content_rect,
                style,
                NSBackingStoreType::NSBackingStoreBuffered,
                false,
            )
        };

        // Create a window delegate
        let window_delegate = WindowDelegate::new(mtm, app_delegate, window.clone(), mode);
        autoreleasepool(|_| {
            let object = ProtocolObject::from_ref(&*window_delegate);
            window.setDelegate(Some(object));
        });

        // Create a root view
        let view = View::new(&window);
        window.setContentView(Some(&view));

        // Set post-creation window options
        if let Some(options) = &options {
            let title_visibility = if options.borderless {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            };
            window.setTitleVisibility(title_visibility);
            window.setTitlebarAppearsTransparent(options.borderless);

            if let Some(button) = window.standardWindowButton(NSWindowButton::NSWindowZoomButton) {
                button.setEnabled(options.fullscreen);
            }

            if unsafe { window.isMovable() } != options.draggable {
                window.setMovable(options.draggable);
            }
        }

        match mode {
            InitMode::Minimized => window.miniaturize(None),
            InitMode::Maximized => window.zoom(None),
            _ => (),
        }

        Self {
            delegate: MainThreadBound::new(window_delegate, mtm),
            native:   MainThreadBound::new(window, mtm),
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
        self.delegate_on_main(|delegate| {
            delegate.set_title(title);
        });
    }

    #[inline]
    fn title(&self) -> String { self.delegate_on_main(|delegate| delegate.title()) }

    #[inline]
    fn set_options(&mut self, options: WindowOptions) {
        self.delegate_on_main(|delegate| {
            delegate.set_options(options);
        });
    }

    #[inline]
    fn options(&self) -> WindowOptions { self.delegate_on_main(|delegate| delegate.options()) }

    #[inline]
    fn show(&mut self, _app: &ActiveApplication) {
        self.delegate_on_main(|delegate| delegate.show());
    }

    #[inline]
    fn show_modal(&mut self, _app: &ActiveApplication) {
        self.delegate_on_main(|delegate| {
            delegate.show_modal();
        });
    }

    #[inline]
    fn toggle_fullscreen(&mut self) {
        self.delegate_on_main(|delegate| {
            delegate.toggle_fullscreen();
        });
    }

    #[inline]
    fn is_fullscreen(&self) -> bool { self.delegate_on_main(|delegate| delegate.is_fullscreen()) }

    #[inline]
    fn set_frame_size(&mut self, size: Size) {
        self.delegate_on_main(|delegate| {
            delegate.set_frame_size(size);
        });
    }

    #[inline]
    fn frame_size(&self) -> PhysicalSize<u32> {
        self.delegate_on_main(|delegate| delegate.frame_size())
    }

    #[inline]
    fn set_position(&mut self, position: Position) {
        self.delegate_on_main(|delegate| {
            delegate.set_position(position);
        });
    }

    #[inline]
    fn position(&self) -> PhysicalPosition<i32> {
        self.delegate_on_main(|delegate| delegate.position())
    }

    #[inline]
    fn set_min_size(&mut self, min_size: Size) {
        self.delegate_on_main(|delegate| {
            delegate.set_min_size(min_size);
        });
    }

    #[inline]
    fn min_size(&self) -> PhysicalSize<u32> {
        self.delegate_on_main(|delegate| delegate.min_size())
    }

    #[inline]
    fn set_max_size(&mut self, max_size: Size) {
        self.delegate_on_main(|delegate| {
            delegate.set_max_size(max_size);
        });
    }

    #[inline]
    fn max_size(&self) -> PhysicalSize<u32> {
        self.delegate_on_main(|delegate| delegate.max_size())
    }

    #[inline]
    fn maximize(&mut self) {
        self.delegate_on_main(|delegate| {
            delegate.maximize();
        });
    }

    #[inline]
    fn is_maximized(&self) -> bool { self.delegate_on_main(|delegate| delegate.is_maximized()) }

    #[inline]
    fn content_size(&self) -> PhysicalSize<u32> {
        self.delegate_on_main(|delegate| delegate.content_size())
    }

    #[inline]
    fn is_visible(&self) -> bool { self.delegate_on_main(|delegate| delegate.is_visible()) }

    #[inline]
    fn close(&mut self) {
        self.delegate_on_main(|delegate| {
            delegate.close();
        });
    }

    #[inline]
    fn minimize(&mut self) {
        self.delegate_on_main(|delegate| {
            delegate.minimize();
        });
    }

    #[inline]
    fn is_minimized(&self) -> bool { self.delegate_on_main(|delegate| delegate.is_minimized()) }

    #[inline]
    fn restore(&mut self) {
        self.delegate_on_main(|delegate| {
            delegate.restore();
        });
    }

    #[inline]
    fn scale_factor(&self) -> f64 { self.delegate_on_main(|delegate| delegate.scale_factor()) }
}

#[cfg(feature = "dh")]
impl HasWindowHandler for WindowImpl {
    fn window_handler(&self) -> WindowHandler {
        let mtm =
            MainThreadMarker::new().expect("window_handler() must be called on the main thread");
        let native = self.get_native(mtm);
        let view = native.contentView().unwrap();
        let ptr = Retained::as_ptr(&view) as *mut _;
        let ptr = NonNull::new(ptr).expect("Retained<T> should never be null");
        WindowHandler::AppKit(AppKitWindowHandler::new(ptr))
    }
}
