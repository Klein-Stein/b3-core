use objc2::{
    declare_class,
    msg_send_id,
    mutability,
    rc::Retained,
    runtime::NSObjectProtocol,
    ClassType,
    DeclaredClass,
};
use objc2_app_kit::{NSResponder, NSView, NSWindow};
use objc2_foundation::{MainThreadMarker, NSObject};

#[derive(Debug)]
pub(super) struct ViewState;

declare_class!(
    #[derive(Debug)]
    pub(super) struct View;

    unsafe impl ClassType for View {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "B3View";
    }

    impl DeclaredClass for View {
        type Ivars = ViewState;
    }

    unsafe impl NSObjectProtocol for View {}

    unsafe impl View {
        // Put overrides here...
    }
);

impl View {
    pub(super) fn new(window: &NSWindow) -> Retained<Self> {
        let mtm = MainThreadMarker::from(window);
        let this = mtm.alloc().set_ivars(ViewState);
        unsafe { msg_send_id![super(this), init] }
    }
}
