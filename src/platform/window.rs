use crate::{ActiveApplication, ContextOwner, InitMode, Point, Size, WindowId, WindowOptions};

pub(crate) trait WindowApi {
    fn new(
        ctx: &impl ContextOwner,
        mode: InitMode,
        options: Option<WindowOptions>,
        size: Size,
    ) -> Self;

    fn init(&mut self, window_id: WindowId);

    fn set_title(&mut self, title: String);
    fn title(&self) -> String;

    fn set_options(&mut self, options: WindowOptions);
    fn options(&self) -> WindowOptions;

    fn show(&mut self, app: &ActiveApplication);
    fn show_modal(&mut self, app: &ActiveApplication);

    fn toggle_fullscreen(&mut self);
    fn is_fullscreen(&self) -> bool;

    fn set_frame_size(&mut self, size: Size);
    fn frame_size(&self) -> Size;

    fn set_position(&mut self, position: Point);
    fn position(&self) -> Point;

    fn set_min_size(&mut self, min_size: Size);
    fn min_size(&self) -> Size;

    fn set_max_size(&mut self, max_size: Size);
    fn max_size(&self) -> Size;

    fn maximize(&mut self);
    fn is_maximized(&self) -> bool;

    fn content_size(&self) -> Size;

    fn is_visible(&self) -> bool;

    fn close(&mut self);

    fn minimize(&mut self);
    fn is_minimized(&self) -> bool;

    fn restore(&mut self);
}
