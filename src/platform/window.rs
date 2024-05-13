use crate::{ActiveApplication, WindowId, WindowOptions};

pub(crate) trait WindowApi {
    fn init(&mut self, window_id: WindowId);

    fn set_title(&mut self, title: String);
    fn title(&self) -> String;

    fn set_options(&mut self, options: WindowOptions);
    fn options(&self) -> WindowOptions;

    fn show(&mut self, app: &ActiveApplication);
}
