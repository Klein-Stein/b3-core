use crate::{ContextOwner, Error, IconType};

pub(crate) trait IconApi {
    fn from_data(
        ctx: &impl ContextOwner,
        icon_data: &Vec<u8>,
        icon_type: IconType,
    ) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_str(ctx: &impl ContextOwner, title: &String) -> Result<Self, Error>
    where
        Self: Sized;
}
