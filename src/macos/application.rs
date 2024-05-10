use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

use core_foundation::{
    base::CFIndex,
    runloop::{
        kCFRunLoopAfterWaiting,
        kCFRunLoopBeforeWaiting,
        kCFRunLoopExit,
        CFRunLoopObserverContext,
    },
};
use objc2::{
    rc::{autoreleasepool, Id},
    runtime::ProtocolObject,
};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSMenu};
use objc2_foundation::MainThreadMarker;

use super::{
    app_delegate::AppDelegate,
    runloop::{control_flow_begin_handler, control_flow_end_handler, PanicInfo, RunLoop},
};
use crate::{platform::ApplicationHandler, EventHandler, Menu, Window, WindowId};

pub(crate) struct ApplicationImpl {
    pub(super) mtm:      MainThreadMarker,
    pub(super) delegate: Option<Id<AppDelegate>>,
    pub(crate) menu:     Option<Menu>,
    pub(crate) windows:  HashMap<WindowId, Window>,
}

impl ApplicationImpl {
    pub(crate) fn new() -> Self {
        let mtm: MainThreadMarker = MainThreadMarker::new()
            .expect("on macOS, `Application` instance must be created on the main thread!");
        Self {
            mtm,
            delegate: None,
            menu: None,
            windows: HashMap::new(),
        }
    }

    #[inline]
    fn get_native_menu(&self) -> Option<Id<NSMenu>> {
        if let Some(menu) = &self.menu {
            Some(menu.menu_impl.native.clone())
        } else {
            None
        }
    }

    fn sync_menu(&self) {
        if let Some(delegate) = &self.delegate {
            let menu = self.get_native_menu();
            delegate.set_menu(menu);
        }
    }
}

impl ApplicationHandler for ApplicationImpl {
    #[inline]
    fn run(&mut self, handler: impl EventHandler + 'static) {
        let app = NSApplication::sharedApplication(self.mtm);
        app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

        let menu = self.get_native_menu();

        // configure the application delegate
        let delegate = AppDelegate::new(self.mtm, menu, handler);
        autoreleasepool(|_| {
            let object = ProtocolObject::from_ref(&*delegate);
            app.setDelegate(Some(object));
        });

        self.delegate = Some(delegate);

        let panic_info: Rc<PanicInfo> = Default::default();
        setup_control_flow_observers(Rc::downgrade(&panic_info));

        autoreleasepool(|_| {
            // SAFETY: We do not run the application re-entrantly
            unsafe { app.run() };
        });
    }

    #[inline]
    fn set_menu(&mut self, menu: Option<Menu>) {
        self.menu = menu;
        self.sync_menu();
    }

    #[inline]
    fn add_window(&mut self, window: Window) { self.windows.insert(window.id(), window); }

    #[inline]
    fn get_window(&self, id: &WindowId) -> Option<&Window> { self.windows.get(id) }

    #[inline]
    fn get_window_mut(&mut self, id: &WindowId) -> Option<&mut Window> { self.windows.get_mut(id) }
}

fn setup_control_flow_observers(panic_info: Weak<PanicInfo>) {
    unsafe {
        let mut context = CFRunLoopObserverContext {
            info:            Weak::into_raw(panic_info) as *mut _,
            version:         0,
            retain:          None,
            release:         None,
            copyDescription: None,
        };
        let run_loop = RunLoop::get();
        run_loop.add_observer(
            kCFRunLoopAfterWaiting,
            CFIndex::min_value(),
            control_flow_begin_handler,
            &mut context as *mut _,
        );
        run_loop.add_observer(
            kCFRunLoopExit | kCFRunLoopBeforeWaiting,
            CFIndex::max_value(),
            control_flow_end_handler,
            &mut context as *mut _,
        );
    }
}
