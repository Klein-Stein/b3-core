use std::ptr;

use core_foundation::{
    base::{Boolean, CFIndex, CFOptionFlags},
    runloop::{
        kCFRunLoopCommonModes,
        CFRunLoopAddObserver,
        CFRunLoopGetMain,
        CFRunLoopObserverCallBack,
        CFRunLoopObserverContext,
        CFRunLoopObserverCreate,
        CFRunLoopRef,
        CFRunLoopWakeUp,
    },
};

const FALSE: Boolean = 0;
const TRUE: Boolean = 1;

pub(super) struct RunLoop(CFRunLoopRef);

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
            CFRunLoopObserverCreate(ptr::null_mut(), flags, TRUE, priority, handler, context)
        };
        unsafe { CFRunLoopAddObserver(self.0, observer, kCFRunLoopCommonModes) };
    }
}
