mod application;
mod icon;
mod menu;
#[cfg(feature = "notifications")]
mod notification;
mod window;

pub(crate) use application::*;
pub(crate) use icon::*;
pub(crate) use menu::*;
#[cfg(feature = "notifications")]
pub(crate) use notification::*;
pub(crate) use window::*;

pub(crate) trait Wrapper<T> {
    fn get_impl(&self) -> &T;
    fn get_impl_mut(&mut self) -> &mut T;
}
