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
use objc2_foundation::{MainThreadMarker, NSObject};

#[derive(Debug)]
pub(super) struct Ivars {}

declare_class!(
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
    }
);

impl WindowDelegate {
    pub(super) fn new(mtm: MainThreadMarker) -> Id<WindowDelegate> {
        let this = mtm.alloc();
        let this = this.set_ivars(Ivars {});
        unsafe { msg_send_id![super(this), init] }
    }
}
