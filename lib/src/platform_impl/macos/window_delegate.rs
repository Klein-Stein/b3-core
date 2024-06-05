use std::cell::Cell;

use dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position, Size};
use objc2::{
    declare_class,
    msg_send_id,
    mutability,
    rc::{autoreleasepool, Retained},
    runtime::NSObjectProtocol,
    ClassType,
    DeclaredClass,
};
use objc2_app_kit::{
    NSApp,
    NSFullScreenWindowMask,
    NSWindow,
    NSWindowButton,
    NSWindowDelegate,
    NSWindowStyleMask,
    NSWindowTitleVisibility,
};
use objc2_foundation::{CGRect, MainThreadMarker, NSNotification, NSObject, NSString};

use super::{
    app_delegate::AppDelegate,
    window_utils::{to_b3_position, to_cgsize, to_macos_coords},
    CocoaWindow,
};
use crate::{Event, InitMode, WindowEvent, WindowId, WindowOptions};

#[derive(Debug)]
pub(super) struct State {
    app_delegate:  Retained<AppDelegate>,
    window:        Retained<CocoaWindow>,
    init_mode:     Cell<Option<InitMode>>,
    prev_position: Cell<PhysicalPosition<i32>>,
}

declare_class!(
    #[derive(Debug)]
    pub(super) struct WindowDelegate;

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - Main thread only mutability is correct, since this is a window delegate.
    // - `WindowDelegate` does not implement `Drop`.
    unsafe impl ClassType for WindowDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "CocoaWindowDelegate";
    }

    impl DeclaredClass for WindowDelegate {
        type Ivars = State;
    }

    unsafe impl NSObjectProtocol for WindowDelegate {}

    unsafe impl NSWindowDelegate for WindowDelegate {
        #[method(windowDidResize:)]
        unsafe fn window_did_resize(&self, _notification: &NSNotification) { self.emit_move_event(); }

        #[method(windowDidMove:)]
        unsafe fn window_did_move(&self, _notification: &NSNotification) { self.emit_move_event(); }

        #[method(windowDidBecomeKey:)]
        unsafe fn window_did_become_key(&self, _notification: &NSNotification) {
            self.queue_event(WindowEvent::Focused(true));
        }

        #[method(windowDidResignKey:)]
        unsafe fn window_did_resign_key(&self, _notification: &NSNotification) {
            self.queue_event(WindowEvent::Focused(false));
        }

        #[method(windowShouldClose:)]
        unsafe fn window_should_close(&self, _sender: &NSWindow) -> bool {
            self.queue_event(WindowEvent::CloseRequested);
            false
        }

        #[method(windowWillClose:)]
        unsafe fn will_close(&self, _notification: &NSNotification) {
            autoreleasepool(|_| {
                self.window().setDelegate(None);
            });
            self.queue_event(WindowEvent::Destroyed);
        }
    }
);

impl WindowDelegate {
    #[inline]
    pub(super) fn new(
        mtm: MainThreadMarker,
        app_delegate: Retained<AppDelegate>,
        window: Retained<CocoaWindow>,
        init_mode: InitMode,
    ) -> Retained<WindowDelegate> {
        let this = mtm.alloc();
        let scale_factor = window.backingScaleFactor();
        let origin = to_b3_position(&window);
        let this = this.set_ivars(State {
            app_delegate,
            window,
            init_mode: Cell::new(Some(init_mode)),
            prev_position: Cell::new(
                LogicalPosition::new(origin.x, origin.y).to_physical(scale_factor),
            ),
        });
        unsafe { msg_send_id![super(this), init] }
    }

    #[inline]
    fn queue_event(&self, event: WindowEvent) {
        self.app_delegate()
            .queue_event(Event::Window(event, self.window_id()));
    }

    fn emit_move_event(&self) {
        let position = self.position();
        if position != self.ivars().prev_position.get() {
            self.ivars().prev_position.set(position);
            self.queue_event(WindowEvent::Moved(position));
        }
    }

    #[inline]
    fn app_delegate(&self) -> &AppDelegate { &self.ivars().app_delegate }

    #[inline]
    fn window(&self) -> &CocoaWindow { &self.ivars().window }

    #[inline]
    pub(super) fn window_id(&self) -> WindowId { self.window().id() }

    #[inline]
    pub(super) fn set_title(&self, title: String) {
        let title = NSString::from_str(&title);
        self.window().setTitle(&title);
    }

    #[inline]
    pub(super) fn title(&self) -> String {
        let title = self.window().title();
        title.to_string()
    }

    #[inline]
    pub(super) fn set_options(&self, options: WindowOptions) {
        let mask: NSWindowStyleMask = options.into();
        let window = self.window();
        window.setStyleMask(mask);

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

    #[inline]
    pub(super) fn options(&self) -> WindowOptions {
        let window = self.window();
        let mask = window.styleMask();
        WindowOptions {
            titled:      (mask.0 & NSWindowStyleMask::Titled.0) != 0,
            minimizable: (mask.0 & NSWindowStyleMask::Miniaturizable.0) != 0,
            closable:    (mask.0 & NSWindowStyleMask::Closable.0) != 0,
            resizable:   (mask.0 & NSWindowStyleMask::Resizable.0) != 0,
            draggable:   unsafe { window.isMovable() },
            fullscreen:  (mask.0 & NSWindowStyleMask::FullScreen.0) != 0,
            borderless:  (mask.0 & NSWindowStyleMask::Borderless.0) != 0,
        }
    }

    fn sync_with_init_mode(&self) {
        let init_mode = self.ivars().init_mode.get();
        if init_mode == Some(InitMode::Fullscreen) {
            self.window().toggleFullScreen(None);
        }
        self.ivars().init_mode.set(None);
    }

    #[inline]
    pub(super) fn show(&self) {
        let window = self.window();
        window.makeKeyAndOrderFront(None);

        self.queue_event(WindowEvent::Showed);

        self.sync_with_init_mode();
    }

    #[inline]
    pub(super) fn show_modal(&self) {
        let mtm = MainThreadMarker::from(self);
        let ns_app = NSApp(mtm);
        let window = self.window();
        unsafe { ns_app.runModalForWindow(window) };

        self.queue_event(WindowEvent::Showed);

        self.sync_with_init_mode();
    }

    #[inline]
    pub(super) fn toggle_fullscreen(&self) { self.window().toggleFullScreen(None); }

    #[inline]
    pub(super) fn is_fullscreen(&self) -> bool {
        (self.window().styleMask().0 & NSFullScreenWindowMask.0) == NSFullScreenWindowMask.0
    }

    #[inline]
    pub(super) fn set_frame_size(&self, size: Size) {
        let window = self.window();
        let origin = window.frame().origin;
        let logical_size = match size {
            Size::Logical(size) => size,
            Size::Physical(size) => {
                let scale_factor = self.scale_factor();
                size.to_logical::<f64>(scale_factor)
            }
        };
        let size = to_cgsize(logical_size);
        let frame = CGRect::new(origin, size);
        unsafe { window.setFrame_display_animate(frame, true, false) };
    }

    #[inline]
    pub(super) fn frame_size(&self) -> PhysicalSize<u32> {
        let size = self.window().frame().size;
        let scale_factor = self.scale_factor();
        LogicalSize::new(size.width, size.height).to_physical(scale_factor)
    }

    #[inline]
    pub(super) fn set_position(&self, position: Position) {
        let scale_factor = self.scale_factor();
        let logical_position = match position {
            Position::Physical(position) => position.to_logical(scale_factor),
            Position::Logical(position) => position,
        };
        let origin = to_macos_coords(logical_position, self.window());
        unsafe { self.window().setFrameOrigin(origin) };
    }

    #[inline]
    pub(super) fn position(&self) -> PhysicalPosition<i32> {
        let window = self.window();
        let scale_factor = self.scale_factor();
        let origin = to_b3_position(window);
        LogicalPosition::new(origin.x, origin.y).to_physical(scale_factor)
    }

    #[inline]
    pub(super) fn set_min_size(&self, min_size: Size) {
        let logical_size = match min_size {
            Size::Logical(size) => size,
            Size::Physical(size) => {
                let scale_factor = self.scale_factor();
                size.to_logical::<f64>(scale_factor)
            }
        };
        let size = to_cgsize(logical_size);
        self.window().setMinSize(size);
    }

    #[inline]
    pub(super) fn min_size(&self) -> PhysicalSize<u32> {
        let min_size = unsafe { self.window().minSize() };
        let scale_factor = self.scale_factor();
        LogicalSize::new(min_size.width, min_size.height).to_physical(scale_factor)
    }

    #[inline]
    pub(super) fn set_max_size(&self, max_size: Size) {
        let logical_size = match max_size {
            Size::Logical(size) => size,
            Size::Physical(size) => {
                let scale_factor = self.scale_factor();
                size.to_logical::<f64>(scale_factor)
            }
        };
        let size = to_cgsize(logical_size);
        self.window().setMaxSize(size);
    }

    #[inline]
    pub(super) fn max_size(&self) -> PhysicalSize<u32> {
        let max_size = unsafe { self.window().maxSize() };
        let scale_factor = self.scale_factor();
        LogicalSize::new(max_size.width, max_size.height).to_physical(scale_factor)
    }

    #[inline]
    pub(super) fn maximize(&self) { self.window().zoom(None); }

    #[inline]
    pub(super) fn is_maximized(&self) -> bool { self.window().isZoomed() }

    #[inline]
    pub(super) fn content_size(&self) -> PhysicalSize<u32> {
        let size = unsafe { self.window().contentLayoutRect().size };
        let scale_factor = self.scale_factor();
        LogicalSize::new(size.width, size.height).to_physical(scale_factor)
    }

    #[inline]
    pub(super) fn is_visible(&self) -> bool { self.window().isVisible() }

    #[inline]
    pub(super) fn close(&self) { self.window().close(); }

    #[inline]
    pub(super) fn minimize(&self) { self.window().miniaturize(None); }

    #[inline]
    pub(super) fn is_minimized(&self) -> bool { self.window().isMiniaturized() }

    #[inline]
    pub(super) fn restore(&self) {
        if self.window().isMiniaturized() {
            unsafe { self.window().deminiaturize(None) };
        }
    }

    #[inline]
    pub(super) fn scale_factor(&self) -> f64 { self.window().backingScaleFactor() }
}
