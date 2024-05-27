//! This module contains image type definitions.

use crate::{
    macos::IconImpl,
    platform::{IconApi, Wrapper},
    ContextOwner,
    Error,
};

/// Icon types.
#[derive(Debug)]
pub enum IconType {
    /// GIF.
    Gif,
    /// JPEG.
    Jpeg,
    /// PNG.
    Png,
    /// TIFF.
    Tiff,
}

/// System icon.
#[derive(Debug)]
pub struct Icon(IconImpl);

impl Icon {
    /// Creates a new icon from bytes.
    ///
    /// # Parameters:
    /// * `ctx` - Context owner.
    /// * `icon_data` - Icon data in bytes.
    /// * `icon_type` - Icon type.
    pub fn from_data(
        ctx: &impl ContextOwner,
        icon_data: &Vec<u8>,
        icon_type: IconType,
    ) -> Result<Self, Error> {
        Ok(Self(IconImpl::from_data(ctx, icon_data, icon_type)?))
    }

    /// Creates a new image from built-in system icons.
    ///
    /// # Parameters:
    /// * `ctx` - Context owner.
    /// * `title` - Built-in system icon title.
    pub fn from_str<S>(ctx: &impl ContextOwner, title: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        Ok(Self(IconImpl::from_str(ctx, &title.into())?))
    }
}

impl Wrapper<IconImpl> for Icon {
    #[inline]
    fn get_impl(&self) -> &IconImpl { &self.0 }

    #[inline]
    fn get_impl_mut(&mut self) -> &mut IconImpl { &mut self.0 }
}
