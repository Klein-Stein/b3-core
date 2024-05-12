use crate::WindowOptions;

pub(crate) trait WindowApi {
    fn set_title(&mut self, title: String);
    fn title(&self) -> String;

    fn set_options(&mut self, options: WindowOptions);
    fn options(&self) -> WindowOptions;

    fn show(&mut self);
}
