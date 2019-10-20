use std::{error, fmt};

#[macro_export]
macro_rules! err {
    ($str:expr) => {
        Err($crate::error::GenericError::new($str).into())
    };
    ($($arg:tt)*) => (Err($crate::error::GenericError::new(std::fmt::format(format_args!($($arg)*))).into()));
}

#[derive(Debug)]
pub struct GenericError {
    msg: String,
}
impl GenericError {
    pub fn new(msg: String) -> Self {
        GenericError { msg }
    }
}

impl error::Error for GenericError {}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
