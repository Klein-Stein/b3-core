use crate::{platform::NotificationApi, platform_impl::NotificationImpl, ActiveApplication};

/// Notification builder.
#[derive(Debug, Default)]
pub struct NotificationBuilder {
    title:   Option<String>,
    message: Option<String>,
    action:  Option<String>,
}

impl NotificationBuilder {
    /// Creates a new instance of the notification builder.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Sets a title of the notification under building.
    ///
    /// # Parameters:
    /// * `title` - Notification title.
    pub fn with_title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    /// Sets a body message of the notification under building.
    ///
    /// # Parameters:
    /// * `message` - Notification body message.
    pub fn with_message<S>(mut self, message: S) -> Self
    where
        S: Into<String>,
    {
        self.message = Some(message.into());
        self
    }

    /// Sets an action for the notification under building.
    ///
    /// # Parameters:
    /// * `action` - Action name.
    pub fn with_action<S>(mut self, action: S) -> Self
    where
        S: Into<String>,
    {
        self.action = Some(action.into());
        self
    }

    /// Builds and displays a new notification.
    ///
    /// # Parameters:
    /// * `app` - Active application.
    pub fn build(self, app: &ActiveApplication) {
        NotificationImpl::show(app, self.title, self.message, self.action);
    }
}
