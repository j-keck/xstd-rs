

/// if, then, else
/// ```
/// use xstd::prelude::*;
/// assert_eq!(1, ifte!( true, 1, 2));
/// assert_eq!(2, ifte!(false, 1, 2));
/// ```
#[macro_export]
macro_rules! ifte {
    ($if:expr, $then:expr, $else:expr) => {
        if $if {
            $then
        } else {
            $else
        }
    };
}
