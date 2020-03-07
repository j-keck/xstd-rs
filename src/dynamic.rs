use std::any::{Any, TypeId};

/// provides `is_instance_of::<T>()` for types which are implement `std::any::Any`.
///
/// **this uses runtime reflection.**
///
/// ```
/// use xstd::prelude::*;
/// struct S{};
/// fn check<T: 'static>(t: T) -> bool {
///   t.is_instance_of::<S>()
/// }
/// assert!(check(S{}));
/// assert!(! check(Some(1)));
/// ```
pub trait IsInstanceOf
where Self: Any,
{
    fn is_instance_of<T: ?Sized + Any>(&self) -> bool {
        TypeId::of::<Self>() == TypeId::of::<T>()
    }
}
impl <T: ?Sized + Any> IsInstanceOf for T {}
