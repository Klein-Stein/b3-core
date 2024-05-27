use objc2::rc::{autoreleasepool, Id};
use objc2_app_kit::NSImage;
use objc2_foundation::{MainThreadBound, MainThreadMarker, NSData, NSString};

use crate::{
    platform::{IconApi, Wrapper},
    ContextOwner,
    Error,
    IconType,
};

#[derive(Debug)]
pub(crate) struct IconImpl {
    mtm:    MainThreadMarker,
    native: MainThreadBound<Id<NSImage>>,
}

impl IconImpl {
    #[inline]
    pub(super) fn get_native(&self) -> &Id<NSImage> { self.native.get(self.mtm) }
}

impl IconApi for IconImpl {
    #[inline]
    fn from_data(
        ctx: &impl ContextOwner,
        icon_data: &Vec<u8>,
        _icon_type: IconType,
    ) -> Result<Self, Error> {
        autoreleasepool(|_| {
            let mtm = ctx.context().get_impl().mtm();
            let allocated = mtm.alloc();

            let data = NSData::with_bytes(&icon_data);
            match NSImage::initWithData(allocated, &data) {
                Some(image) => Ok(Self {
                    mtm,
                    native: MainThreadBound::new(image, mtm),
                }),
                None => Err(Error::new("NSImage not created.")),
            }
        })
    }

    #[inline]
    fn from_str(ctx: &impl ContextOwner, title: &String) -> Result<Self, Error> {
        autoreleasepool(|_| {
            let mtm = ctx.context().get_impl().mtm();

            let name = NSString::from_str(title);
            match unsafe {
                NSImage::imageWithSystemSymbolName_accessibilityDescription(&name, None)
            } {
                Some(image) => Ok(Self {
                    mtm,
                    native: MainThreadBound::new(image, mtm),
                }),
                None => Err(Error::new("NSImage not created.")),
            }
        })
    }
}
