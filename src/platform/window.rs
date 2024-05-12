pub(crate) trait WindowApi {
    fn set_title(&mut self, title: String);
    fn title(&self) -> String;

    fn show(&mut self);
}
