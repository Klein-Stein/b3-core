use std::{
    ffi::c_void,
    panic::{AssertUnwindSafe, UnwindSafe},
    rc::Weak,
};

use core_foundation::{
    base::CFIndex,
    runloop::{
        kCFRunLoopAfterWaiting,
        kCFRunLoopBeforeWaiting,
        kCFRunLoopExit,
        CFRunLoopActivity,
        CFRunLoopObserverContext,
        CFRunLoopObserverRef,
    },
};
use objc2_foundation::MainThreadMarker;

use super::{app_delegate::AppDelegate, panicinfo::PanicInfo, runloop::RunLoop, stop_app_on_panic};

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

// begin is queued with the highest priority to ensure it is processed before other observers
extern "C" fn control_flow_begin_handler(
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
extern "C" fn control_flow_end_handler(
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

pub fn setup_control_flow_observers(panic_info: Weak<PanicInfo>) {
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
