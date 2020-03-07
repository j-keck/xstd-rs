#! [deny(missing_docs)]

//! e**x**tended **st**andar**d** library
//!

/// iterator
pub mod iter;

/// vector
pub mod vec;

/// streams
#[cfg(feature = "stream")]
pub mod stream;

/// macros
pub mod macros;

/// reflection
pub mod dynamic;

/// use the `prelude` to import all extensions
/// ```
/// use xstd::prelude::*;
/// ```
pub mod prelude {
    pub use crate::iter::*;
    pub use crate::vec::*;
    #[cfg(feature = "stream")]
    pub use crate::stream::*;
    pub use crate::dynamic::*;

    /// macros
    pub use crate::ifte;
}
