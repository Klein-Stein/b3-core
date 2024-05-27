use objc2_foundation::{NSString, NSUUID};
use objc2_user_notifications::{
    UNMutableNotificationContent,
    UNNotificationRequest,
    UNUserNotificationCenter,
};

use crate::{
    platform::{NotificationApi, Wrapper},
    ActiveApplication,
    ContextOwner,
};

#[derive(Debug)]
pub struct NotificationImpl;

impl NotificationApi for NotificationImpl {
    fn show(
        app: &ActiveApplication,
        title: Option<String>,
        message: Option<String>,
        _action: Option<String>,
    ) {
        let mtm = app.context().get_impl().mtm();

        let content_alloc = mtm.alloc();
        let content = unsafe { UNMutableNotificationContent::init(content_alloc) };

        if let Some(title) = title {
            let title = NSString::from_str(&title);
            unsafe { content.setTitle(&title) };
        }

        if let Some(message) = message {
            let message = NSString::from_str(&message);
            unsafe { content.setBody(&message) };
        }

        let uuid_alloc = mtm.alloc();
        let uuid = NSUUID::init(uuid_alloc);
        let identifier = uuid.UUIDString();

        let request = unsafe {
            UNNotificationRequest::requestWithIdentifier_content_trigger(
                &identifier,
                &content,
                None,
            )
        };

        unsafe {
            let center = UNUserNotificationCenter::currentNotificationCenter();
            center.addNotificationRequest_withCompletionHandler(&request, None);
        }
    }
}
