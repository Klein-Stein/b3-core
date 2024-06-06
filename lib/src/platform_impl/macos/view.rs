use std::{cell::Cell, ptr};

use dpi::{LogicalPosition, LogicalSize};
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
use objc2_app_kit::{
    NSEvent,
    NSEventPhase,
    NSResponder,
    NSTrackingRectTag,
    NSView,
    NSViewFrameDidChangeNotification,
};
use objc2_foundation::{CGRect, MainThreadMarker, NSNotificationCenter, NSObject, NSRect};

use super::{app_delegate::AppDelegate, CocoaWindow};
use crate::{
    Event,
    MouseButton,
    MouseButtonState,
    MouseEvent,
    ScrollingDelta,
    ScrollingPhase,
    WindowEvent,
};

const LEFT_MOUSE_BUTTON: u16 = 0;
const RIGHT_MOUSE_BUTTON: u16 = 1;
const MIDDLE_MOUSE_BUTTON: u16 = 2;
const BACK_MOUSE_BUTTON: u16 = 3;
const FORWARD_MOUSE_BUTTON: u16 = 4;

#[derive(Debug)]
pub(super) struct ViewState {
    app_delegate:      Retained<AppDelegate>,
    ns_window:         Weak<CocoaWindow>,
    tracking_rect_tag: Cell<Option<NSTrackingRectTag>>,
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
        #[method(viewDidMoveToWindow)]
        fn view_did_move_to_window(&self) {
            let rect = self.frame();
            self.update_tracking_area(rect);
        }

        #[method(frameDidChange:)]
        fn frame_did_change(&self, _event: &NSEvent) {
            let rect = self.frame();
            self.update_tracking_area(rect);
            let logical_size = LogicalSize::new(rect.size.width as f64, rect.size.height as f64);
            let size = logical_size.to_physical::<u32>(self.scale_factor());
            self.queue_window_event(WindowEvent::Resized(size));
        }

        #[method(drawRect:)]
        fn draw_rect(&self, _rect: NSRect) {
            if let Some(window) = self.ivars().ns_window.load() {
                self.ivars().app_delegate.handle_redraw(window.id());
            }

            // This is a direct subclass of NSView, no need to call superclass' drawRect:
        }

        #[method(mouseDown:)]
        fn mouse_down(&self, _event: &NSEvent) {
            self.queue_mouse_click(LEFT_MOUSE_BUTTON, MouseButtonState::Pressed);
        }

        #[method(rightMouseDown:)]
        fn right_mouse_down(&self, _event: &NSEvent) {
            self.queue_mouse_click(RIGHT_MOUSE_BUTTON, MouseButtonState::Pressed);
        }

        #[method(otherMouseDown:)]
        unsafe fn other_mouse_down(&self, event: &NSEvent) {
            self.queue_mouse_click(event.buttonNumber() as u16, MouseButtonState::Pressed);
        }

        #[method(mouseUp:)]
        fn mouse_up(&self, _event: &NSEvent) {
            self.queue_mouse_click(LEFT_MOUSE_BUTTON, MouseButtonState::Released);
        }

        #[method(rightMouseUp:)]
        fn right_mouse_up(&self, _event: &NSEvent) {
            self.queue_mouse_click(RIGHT_MOUSE_BUTTON, MouseButtonState::Released);
        }

        #[method(otherMouseUp:)]
        unsafe fn other_mouse_up(&self, event: &NSEvent) {
            self.queue_mouse_click(event.buttonNumber() as u16, MouseButtonState::Released);
        }

        #[method(scrollWheel:)]
        unsafe fn scroll_wheel(&self, event: &NSEvent) {
            let raw_delta = (event.scrollingDeltaX(), event.scrollingDeltaY());
            let delta = if event.hasPreciseScrollingDeltas() {
                ScrollingDelta::Pixel(raw_delta.0, raw_delta.1)
            } else {
                ScrollingDelta::Line(raw_delta.0 as f32, raw_delta.1 as f32)
            };

            let phase = match event.momentumPhase() {
                NSEventPhase::Began | NSEventPhase::MayBegin => ScrollingPhase::Started,
                NSEventPhase::Stationary => ScrollingPhase::Stationary,
                NSEventPhase::Changed => ScrollingPhase::Changed,
                NSEventPhase::Ended | NSEventPhase::Cancelled => ScrollingPhase::Ended,
                _ => match event.phase() {
                    NSEventPhase::Began | NSEventPhase::MayBegin => ScrollingPhase::Started,
                    NSEventPhase::Stationary => ScrollingPhase::Stationary,
                    NSEventPhase::Changed => ScrollingPhase::Changed,
                    _ => ScrollingPhase::Ended,
                },
            };

            self.emit_mouse_event(MouseEvent::Scroll {
                delta,
                phase,
            });
        }

        #[method(mouseEntered:)]
        fn mouse_entered(&self, _event: &NSEvent) {
            self.emit_mouse_event(MouseEvent::Entered);
        }

        #[method(mouseExited:)]
        fn mouse_exited(&self, _event: &NSEvent) {
            self.emit_mouse_event(MouseEvent::Exited);
        }

        #[method(mouseMoved:)]
        fn mouse_moved(&self, event: &NSEvent) {
            self.emit_mouse_motion(event);
        }

        #[method(mouseDragged:)]
        fn mouse_dragged(&self, event: &NSEvent) {
            self.emit_mouse_motion(event);
        }

        #[method(rightMouseDragged:)]
        fn right_mouse_dragged(&self, event: &NSEvent) {
            self.emit_mouse_motion(event);
        }

        #[method(otherMouseDragged:)]
        fn other_mouse_dragged(&self, event: &NSEvent) {
            self.emit_mouse_motion(event);
        }

        #[method(acceptsFirstResponder)]
        fn accepts_first_responder(&self) -> bool {
            true
        }
    }
);

impl View {
    pub(super) fn new(
        app_delegate: Retained<AppDelegate>,
        ns_window: &Retained<CocoaWindow>,
    ) -> Retained<Self> {
        let mtm = MainThreadMarker::from(ns_window.as_ref());
        let this = mtm.alloc().set_ivars(ViewState {
            app_delegate,
            ns_window: Weak::from_retained(ns_window),
            tracking_rect_tag: Default::default(),
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
    fn window(&self) -> Retained<CocoaWindow> {
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

    fn queue_window_event(&self, event: WindowEvent) {
        if let Some(window) = self.ivars().ns_window.load() {
            self.ivars()
                .app_delegate
                .queue_event(Event::Window(event, window.id()));
        }
    }

    #[inline]
    fn emit_mouse_event(&self, event: MouseEvent) {
        self.queue_window_event(WindowEvent::Mouse(event));
    }

    fn queue_mouse_click(&self, button_number: u16, state: MouseButtonState) {
        let button = match button_number {
            LEFT_MOUSE_BUTTON => MouseButton::Left,
            RIGHT_MOUSE_BUTTON => MouseButton::Right,
            MIDDLE_MOUSE_BUTTON => MouseButton::Middle,
            BACK_MOUSE_BUTTON => MouseButton::Back,
            FORWARD_MOUSE_BUTTON => MouseButton::Forward,
            button_number => MouseButton::Other {
                id: button_number
            },
        };
        self.emit_mouse_event(MouseEvent::Input {
            button,
            state,
        });
    }

    fn update_tracking_area(&self, rect: CGRect) {
        if let Some(tracking_rect_tag) = self.ivars().tracking_rect_tag.take() {
            self.removeTrackingRect(tracking_rect_tag);
        }

        let tracking_rect_tag = unsafe {
            self.addTrackingRect_owner_userData_assumeInside(rect, self, ptr::null_mut(), false)
        };
        self.ivars().tracking_rect_tag.set(Some(tracking_rect_tag));
    }

    fn emit_mouse_motion(&self, event: &NSEvent) {
        let window_location = unsafe { event.locationInWindow() };
        let position = self.convertPoint_fromView(window_location, None);
        let scale_factor = self.scale_factor();
        let position = LogicalPosition::new(position.x, position.y).to_physical(scale_factor);
        self.emit_mouse_event(MouseEvent::Moved {
            position,
        });
    }
}
