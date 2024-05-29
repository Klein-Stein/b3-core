use std::{
    panic::{catch_unwind, UnwindSafe},
    rc::{Rc, Weak},
};

use objc2::{
    rc::{autoreleasepool, Retained},
    runtime::ProtocolObject,
};
use objc2_app_kit::{NSApp, NSApplication, NSApplicationActivationPolicy};
use objc2_foundation::MainThreadMarker;

use super::{
    app_delegate::AppDelegate,
    events::dummy_event,
    observers::setup_control_flow_observers,
    panicinfo::PanicInfo,
};
use crate::{
    platform::{ActiveApplicationApi, ApplicationApi, Wrapper},
    ActiveApplication,
    Context,
    ContextOwner,
    Error,
    EventHandler,
    Icon,
    Menu,
};

pub(super) fn stop_app_immediately(app: &NSApplication) {
    autoreleasepool(|_| {
        app.stop(None);
        // To stop event loop immediately, we need to post some event here.
        // See: https://stackoverflow.com/questions/48041279/stopping-the-nsapplication-main-event-loop/48064752#48064752
        app.postEvent_atStart(&dummy_event().unwrap(), true);
    });
}

/// Catches panics that happen inside `f` and when a panic
/// happens, stops the `sharedApplication`
#[inline]
pub fn stop_app_on_panic<F: FnOnce() -> R + UnwindSafe, R>(
    mtm: MainThreadMarker,
    panic_info: Weak<PanicInfo>,
    f: F,
) -> Option<R> {
    match catch_unwind(f) {
        Ok(r) => Some(r),
        Err(e) => {
            // It's important that we set the panic before requesting a `stop`
            // because some callback are still called during the `stop` message
            // and we need to know in those callbacks if the application is currently
            // panicking
            {
                let panic_info = panic_info.upgrade().unwrap();
                panic_info.set_panic(e);
            }
            let app = NSApplication::sharedApplication(mtm);
            stop_app_immediately(&app);
            None
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ContextImpl {
    mtm:      MainThreadMarker,
    delegate: Retained<AppDelegate>,
}

impl ContextImpl {
    #[inline]
    pub(super) fn mtm(&self) -> MainThreadMarker { self.mtm }

    #[inline]
    pub(super) fn app_delegate(&self) -> &Retained<AppDelegate> { &self.delegate }
}

#[derive(Debug)]
pub(crate) struct ActiveApplicationImpl(Context);

impl ActiveApplicationImpl {
    #[inline]
    fn new(context: Context) -> Self { Self(context) }

    #[inline]
    fn delegate(&self) -> &Retained<AppDelegate> { self.0.get_impl().app_delegate() }
}

impl ActiveApplicationApi for ActiveApplicationImpl {
    #[inline]
    fn set_menu(&mut self, menu: Option<&Menu>) { self.delegate().set_menu(menu); }

    #[inline]
    fn set_icon(&mut self, icon: Option<&Icon>) { self.delegate().set_icon(icon); }

    #[inline]
    fn stop(&mut self) { self.delegate().stop(); }
}

impl ContextOwner for ActiveApplicationImpl {
    #[inline]
    fn context(&self) -> &Context { &self.0 }
}

#[derive(Debug)]
pub(crate) struct ApplicationImpl {
    native:  Retained<NSApplication>,
    context: Context,
}

impl ApplicationApi for ApplicationImpl {
    #[inline]
    fn new() -> Result<Self, Error> {
        if let Some(mtm) = MainThreadMarker::new() {
            // Configure the application delegate
            let delegate = AppDelegate::new(mtm);

            // Initialize a new application.
            let app = NSApp(mtm);

            // Configure the application context
            let context = Context::new(ContextImpl {
                mtm,
                delegate: delegate.clone(),
            });

            // Configure the active application
            let active_app = ActiveApplicationImpl::new(context.clone());
            let active_app = ActiveApplication::new(active_app);
            delegate.set_active_application(active_app);

            // Set the application delegate
            autoreleasepool(|_| {
                let object = ProtocolObject::from_ref(&*delegate);
                app.setDelegate(Some(object));
            });

            Ok(Self {
                native: app,
                context,
            })
        } else {
            Err(Error::new(
                "application instance must be created on the main thread.",
            ))
        }
    }

    #[inline]
    fn run(&mut self, handler: impl EventHandler + 'static) {
        // Register an event handler
        self.context.get_impl().app_delegate().set_handler(handler);
        // Set an activation policy
        self.native
            .setActivationPolicy(NSApplicationActivationPolicy::Regular);

        let panic_info: Rc<PanicInfo> = Default::default();
        setup_control_flow_observers(Rc::downgrade(&panic_info));

        autoreleasepool(|_| {
            unsafe { self.native.run() };
        });
    }
}

impl ContextOwner for ApplicationImpl {
    #[inline]
    fn context(&self) -> &Context { &self.context }
}
