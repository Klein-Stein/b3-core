#[cfg(target_os = "macos")]
mod macos;

pub(crate) use macos::*;
