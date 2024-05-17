//! This module contains image type definitions.

use crate::{
    macos::ImageImpl,
    platform::{ImageApi, Wrapper},
    ContextOwner,
    Error,
};

/// Image types.
#[derive(Debug)]
pub enum ImageType {
    /// GIF.
    Gif,
    /// JPEG.
    Jpeg,
    /// PNG.
    Png,
    /// TIFF.
    Tiff,
}

/// System image.
#[derive(Debug)]
pub struct Image(ImageImpl);

impl Image {
    pub(crate) fn new(image_impl: ImageImpl) -> Self { Self(image_impl) }

    /// Creates a new image from bytes.
    ///
    /// # Parameters:
    /// * `ctx` - Context owner.
    /// * `image_data` - Image data in bytes.
    /// * `image_type` - Image type.
    pub fn from_data(
        ctx: &impl ContextOwner,
        image_data: &Vec<u8>,
        image_type: ImageType,
    ) -> Result<Self, Error> {
        Ok(Self(ImageImpl::from_data(ctx, image_data, image_type)?))
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
        Ok(Self(ImageImpl::from_str(ctx, &title.into())?))
    }
}

impl Wrapper<ImageImpl> for Image {
    #[inline]
    fn get_impl(&self) -> &ImageImpl { &self.0 }

    #[inline]
    fn get_impl_mut(&mut self) -> &mut ImageImpl { &mut self.0 }
}
