use crate::{ContextOwner, Error, ImageType};

pub(crate) trait ImageApi {
    fn from_data(
        ctx: &impl ContextOwner,
        image_data: &Vec<u8>,
        image_type: ImageType,
    ) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_str(ctx: &impl ContextOwner, title: &String) -> Result<Self, Error>
    where
        Self: Sized;
}
