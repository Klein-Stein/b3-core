use std::cell::{Cell, RefCell};

use objc2::{
    declare_class,
    msg_send_id,
    mutability,
    rc::Retained,
    runtime::NSObjectProtocol,
    ClassType,
    DeclaredClass,
};
use objc2_app_kit::NSWindowDelegate;
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject};

use super::app_delegate::AppDelegate;
use crate::{Event, WindowEvent, WindowId};

#[derive(Debug, Default)]
pub(super) struct State {
    window_id:    Cell<Option<WindowId>>,
    app_delegate: RefCell<Option<Retained<AppDelegate>>>,
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
        #[method(windowWillClose:)]
        unsafe fn will_close(&self, _notification: &NSNotification) {
            let window_id = self.window_id();
            self.handle_event(Event::Window(WindowEvent::Close, window_id));
        }
    }
);

impl WindowDelegate {
    pub(super) fn new(mtm: MainThreadMarker) -> Retained<WindowDelegate> {
        let this = mtm.alloc();
        let this = this.set_ivars(State {
            ..Default::default()
        });
        unsafe { msg_send_id![super(this), init] }
    }

    pub(super) fn set_app_delegate(&self, app_delegate: Retained<AppDelegate>) {
        *self.ivars().app_delegate.borrow_mut() = Some(app_delegate);
    }

    #[inline]
    pub(super) fn window_id(&self) -> WindowId {
        self.ivars()
            .window_id
            .clone()
            .get()
            .expect("window ID was not set.")
    }

    #[inline]
    pub(super) fn set_window_id(&self, window_id: WindowId) {
        self.ivars().window_id.set(Some(window_id));
    }

    pub(super) fn handle_event(&self, event: Event) {
        let mut delegate = self.ivars().app_delegate.borrow_mut();

        if let Some(delegate) = delegate.as_mut() {
            delegate.queue_event(event);
        }
    }
}
