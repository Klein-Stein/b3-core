mod application;
mod image;
mod menu;
mod window;

pub(crate) use application::*;
pub(crate) use image::*;
pub(crate) use menu::*;
pub(crate) use window::*;

pub(crate) trait Wrapper<T> {
    fn get_impl(&self) -> &T;
    fn get_impl_mut(&mut self) -> &mut T;
}
