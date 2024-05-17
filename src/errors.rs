use std::fmt;

/// This structure represents an internal error.
///
/// You can intercept this error and override the behavior in case of a failure.
#[derive(Debug, Clone)]
pub struct Error(String);

impl Error {
    pub(crate) fn new<S>(msg: S) -> Self
    where
        S: Into<String>,
    {
        Self(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self.0) }
}
