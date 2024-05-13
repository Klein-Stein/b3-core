use crate::{ActiveApplication, Point, Size, WindowId, WindowOptions};

pub(crate) trait WindowApi {
    fn init(&mut self, window_id: WindowId);

    fn set_title(&mut self, title: String);
    fn title(&self) -> String;

    fn set_options(&mut self, options: WindowOptions);
    fn options(&self) -> WindowOptions;

    fn show(&mut self, app: &ActiveApplication);

    fn toggle_fullscreen(&mut self);
    fn is_fullscreen(&self) -> bool;

    fn set_frame_size(&mut self, size: Size);
    fn frame_size(&self) -> Size;

    fn set_position(&mut self, position: Point);
    fn position(&self) -> Point;
}
