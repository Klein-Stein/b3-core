use std::cell::{Cell, RefCell};

use objc2::{
    declare_class,
    msg_send_id,
    mutability,
    rc::Id,
    runtime::NSObjectProtocol,
    ClassType,
    DeclaredClass,
};
use objc2_app_kit::NSWindowDelegate;
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject};

use super::app_delegate::AppDelegate;
use crate::{Event, WindowEvent, WindowId};

#[derive(Debug, Default)]
pub(super) struct Ivars {
    window_id:    Cell<Option<WindowId>>,
    app_delegate: RefCell<Option<Id<AppDelegate>>>,
}

declare_class!(
    #[derive(Debug)]
    pub(super) struct WindowDelegate;

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - Main thread only mutability is correct, since this is an application delegate.
    // - `AppDelegate` does not implement `Drop`.
    unsafe impl ClassType for WindowDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "B3WindowDelegate";
    }

    impl DeclaredClass for WindowDelegate {
        type Ivars = Ivars;
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
    pub(super) fn new(mtm: MainThreadMarker) -> Id<WindowDelegate> {
        let this = mtm.alloc();
        let this = this.set_ivars(Ivars {
            ..Default::default()
        });
        unsafe { msg_send_id![super(this), init] }
    }

    pub(super) fn set_app_delegate(&self, app_delegate: Id<AppDelegate>) {
        let mut delegate = self.ivars().app_delegate.borrow_mut();
        *delegate = Some(app_delegate);
    }

    #[inline]
    pub(super) fn window_id(&self) -> WindowId {
        self.ivars()
            .window_id
            .clone()
            .get()
            .expect("window ID was not set.")
    }

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
