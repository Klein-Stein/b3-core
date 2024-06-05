use std::cell::Cell;

use dpi::LogicalSize;
use objc2::{
    declare_class,
    msg_send_id,
    mutability,
    rc::{Retained, Weak},
    runtime::NSObjectProtocol,
    sel,
    ClassType,
    DeclaredClass,
};
use objc2_app_kit::{NSEvent, NSResponder, NSView, NSViewFrameDidChangeNotification, NSWindow};
use objc2_foundation::{MainThreadMarker, NSNotificationCenter, NSObject, NSRect};

use super::app_delegate::AppDelegate;
use crate::{Event, WindowEvent, WindowId};

#[derive(Debug)]
pub(super) struct ViewState {
    app_delegate: Retained<AppDelegate>,
    window_id:    Cell<Option<WindowId>>,
    ns_window:    Weak<NSWindow>,
}

declare_class!(
    #[derive(Debug)]
    pub(super) struct View;

    unsafe impl ClassType for View {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "CocoaView";
    }

    impl DeclaredClass for View {
        type Ivars = ViewState;
    }

    unsafe impl NSObjectProtocol for View {}

    unsafe impl View {
        #[method(frameDidChange:)]
        fn frame_did_change(&self, _event: &NSEvent) {
            let rect = self.frame();
            let logical_size = LogicalSize::new(rect.size.width as f64, rect.size.height as f64);
            let size = logical_size.to_physical::<u32>(self.scale_factor());
            self.queue_event(WindowEvent::Resized(size));
        }

        #[method(drawRect:)]
        fn draw_rect(&self, _rect: NSRect) {
            if let Some(window_id) = self.ivars().window_id.get() {
                self.ivars().app_delegate.handle_redraw(window_id);
            }

            // This is a direct subclass of NSView, no need to call superclass' drawRect:
        }
    }
);

impl View {
    pub(super) fn new(
        app_delegate: Retained<AppDelegate>,
        ns_window: &Retained<NSWindow>,
    ) -> Retained<Self> {
        let mtm = MainThreadMarker::from(ns_window.as_ref());
        let this = mtm.alloc().set_ivars(ViewState {
            app_delegate,
            window_id: Cell::new(None),
            ns_window: Weak::from_retained(ns_window),
        });
        let view: Retained<Self> = unsafe { msg_send_id![super(this), init] };

        view.setPostsFrameChangedNotifications(true);
        let notification_center = unsafe { NSNotificationCenter::defaultCenter() };
        unsafe {
            notification_center.addObserver_selector_name_object(
                &view,
                sel!(frameDidChange:),
                Some(NSViewFrameDidChangeNotification),
                Some(&view),
            )
        }

        view
    }

    #[inline]
    pub(super) fn set_window_id(&self, window_id: WindowId) {
        self.ivars().window_id.set(Some(window_id));
    }

    #[inline]
    fn window(&self) -> Retained<NSWindow> {
        // TODO: Simply use `window` property on `NSView`.
        // That only returns a window _after_ the view has been attached though!
        // (which is incompatible with `frameDidChange:`)
        self.ivars()
            .ns_window
            .load()
            .expect("view has no a linked window")
    }

    #[inline]
    fn scale_factor(&self) -> f64 { self.window().backingScaleFactor() }

    fn queue_event(&self, event: WindowEvent) {
        if let Some(window_id) = self.ivars().window_id.get() {
            self.ivars()
                .app_delegate
                .queue_event(Event::Window(event, window_id));
        }
    }
}
