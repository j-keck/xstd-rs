#! [deny(missing_docs)]

//! e**x**tended **st**andar**d** library
//!
pub mod iter;
pub mod vec;
#[cfg(feature = "stream")]
pub mod stream;

pub mod prelude {
    pub use crate::iter::*;
    pub use crate::vec::*;
    #[cfg(feature = "stream")]
    pub use crate::stream::*;
}
