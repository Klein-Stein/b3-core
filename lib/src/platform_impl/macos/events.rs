use objc2::rc::Retained;
use objc2_app_kit::{NSEvent, NSEventModifierFlags, NSEventSubtype, NSEventType};
use objc2_foundation::NSPoint;

pub(super) fn dummy_event() -> Option<Retained<NSEvent>> {
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
