use std::{
    any::Any,
    cell::Cell,
    ffi::c_void,
    panic::{catch_unwind, AssertUnwindSafe, RefUnwindSafe, UnwindSafe},
    ptr,
    rc::Weak,
};

use core_foundation::{
    base::{Boolean, CFIndex, CFOptionFlags},
    runloop::{
        kCFRunLoopAfterWaiting,
        kCFRunLoopBeforeWaiting,
        kCFRunLoopCommonModes,
        kCFRunLoopExit,
        CFRunLoopActivity,
        CFRunLoopAddObserver,
        CFRunLoopGetMain,
        CFRunLoopObserverCallBack,
        CFRunLoopObserverContext,
        CFRunLoopObserverCreate,
        CFRunLoopObserverRef,
        CFRunLoopRef,
        CFRunLoopWakeUp,
    },
};
use objc2::rc::{autoreleasepool, Id};
use objc2_app_kit::{NSApplication, NSEvent, NSEventModifierFlags, NSEventSubtype, NSEventType};
use objc2_foundation::{MainThreadMarker, NSPoint};

use super::app_delegate::AppDelegate;

const TRUE: Boolean = 1;

#[derive(Default)]
pub struct PanicInfo {
    inner: Cell<Option<Box<dyn Any + Send + 'static>>>,
}

// WARNING:
// As long as this struct is used through its `impl`, it is UnwindSafe.
// (If `get_mut` is called on `inner`, unwind safety may get broken.)
impl UnwindSafe for PanicInfo {}
impl RefUnwindSafe for PanicInfo {}
impl PanicInfo {
    pub fn is_panicking(&self) -> bool {
        let inner = self.inner.take();
        let result = inner.is_some();
        self.inner.set(inner);
        result
    }

    /// Overwrites the current state if the current state is not panicking
    pub fn set_panic(&self, p: Box<dyn Any + Send + 'static>) {
        if !self.is_panicking() {
            self.inner.set(Some(p));
        }
    }

    pub fn take(&self) -> Option<Box<dyn Any + Send + 'static>> { self.inner.take() }
}

fn post_empty_event() -> Option<Id<NSEvent>> {
    unsafe {
        NSEvent::otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2(
            NSEventType::ApplicationDefined,
            NSPoint::new(0.0, 0.0),
            NSEventModifierFlags(0),
            0.0,
            0,
            None,
            NSEventSubtype::WindowExposed.0,
            0,
            0,
        )
    }
}

pub(super) fn stop_app_immediately(app: &NSApplication) {
    autoreleasepool(|_| {
        app.stop(None);
        // To stop event loop immediately, we need to post some event here.
        // See: https://stackoverflow.com/questions/48041279/stopping-the-nsapplication-main-event-loop/48064752#48064752
        app.postEvent_atStart(&post_empty_event().unwrap(), true);
    });
}

unsafe fn control_flow_handler<F>(panic_info: *mut c_void, f: F)
where
    F: FnOnce(Weak<PanicInfo>) + UnwindSafe,
{
    let info_from_raw = unsafe { Weak::from_raw(panic_info as *mut PanicInfo) };
    // Asserting unwind safety on this type should be fine because `PanicInfo` is
    // `RefUnwindSafe` and `Rc<T>` is `UnwindSafe` if `T` is `RefUnwindSafe`.
    let panic_info = AssertUnwindSafe(Weak::clone(&info_from_raw));
    // `from_raw` takes ownership of the data behind the pointer.
    // But if this scope takes ownership of the weak pointer, then
    // the weak pointer will get free'd at the end of the scope.
    // However we want to keep that weak reference around after the function.
    std::mem::forget(info_from_raw);

    let mtm = MainThreadMarker::new().unwrap();
    stop_app_on_panic(mtm, Weak::clone(&panic_info), move || {
        let _ = &panic_info;
        f(panic_info.0)
    });
}

/// Catches panics that happen inside `f` and when a panic
/// happens, stops the `sharedApplication`
#[inline]
pub(super) fn stop_app_on_panic<F: FnOnce() -> R + UnwindSafe, R>(
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

// begin is queued with the highest priority to ensure it is processed before other observers
pub(super) extern "C" fn control_flow_begin_handler(
    _: CFRunLoopObserverRef,
    activity: CFRunLoopActivity,
    panic_info: *mut c_void,
) {
    unsafe {
        control_flow_handler(panic_info, |panic_info| {
            #[allow(non_upper_case_globals)]
            match activity {
                kCFRunLoopAfterWaiting => {
                    AppDelegate::get(MainThreadMarker::new().unwrap()).wakeup(panic_info);
                }
                _ => unreachable!(),
            }
        });
    }
}

// end is queued with the lowest priority to ensure it is processed after other observers
// without that, LoopExiting would  get sent after AboutToWait
pub(super) extern "C" fn control_flow_end_handler(
    _: CFRunLoopObserverRef,
    activity: CFRunLoopActivity,
    panic_info: *mut c_void,
) {
    unsafe {
        control_flow_handler(panic_info, |panic_info| {
            #[allow(non_upper_case_globals)]
            match activity {
                kCFRunLoopBeforeWaiting => {
                    AppDelegate::get(MainThreadMarker::new().unwrap()).cleared(panic_info);
                }
                kCFRunLoopExit => (), // unimplemented!(), // not expected to ever happen
                _ => unreachable!(),
            }
        });
    }
}

pub struct RunLoop(CFRunLoopRef);

impl RunLoop {
    pub(super) unsafe fn get() -> Self { RunLoop(unsafe { CFRunLoopGetMain() }) }

    pub(super) fn wakeup(&self) { unsafe { CFRunLoopWakeUp(self.0) } }

    pub(super) unsafe fn add_observer(
        &self,
        flags: CFOptionFlags,
        priority: CFIndex,
        handler: CFRunLoopObserverCallBack,
        context: *mut CFRunLoopObserverContext,
    ) {
        let observer = unsafe {
            CFRunLoopObserverCreate(
                ptr::null_mut(),
                flags,
                TRUE,     // Indicates we want this to run repeatedly
                priority, // The lower the value, the sooner this will run
                handler,
                context,
            )
        };
        unsafe { CFRunLoopAddObserver(self.0, observer, kCFRunLoopCommonModes) };
    }
}
