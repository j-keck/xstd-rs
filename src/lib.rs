#! [deny(missing_docs)]

//! e**x**tended **st**andar**d** library
//!
pub mod iter;
pub mod vec;

pub mod prelude {
    pub use crate::iter::*;
    pub use crate::vec::*;
}
