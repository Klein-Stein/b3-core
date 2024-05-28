use crate::ActiveApplication;

pub trait NotificationApi {
    fn show(
        app: &ActiveApplication,
        title: Option<String>,
        message: Option<String>,
        action: Option<String>,
    );
}
