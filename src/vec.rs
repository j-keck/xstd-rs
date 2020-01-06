use crate::prelude::*;

///
/// Joins a `Vec<&str>` with separating newlines in a `String`.
//
/// # Examples
///
/// ```
/// use xstd::prelude::*;
///
/// assert_eq!("", vec![].unlines());
/// assert_eq!("one", vec!["one"].unlines());
/// assert_eq!("one\ntwo", vec!["one", "two"].unlines());
/// ```
pub trait Unlines {
    fn unlines(&self) -> String;
}
impl Unlines for Vec<&str> {
    fn unlines(&self) -> String {
        self.iter()
            .intersperse(&"\n")
            .map(|str| (*str).to_string())
            .collect()
    }
}

///
/// Joins a `Vec<&str>` with separating spaces in a `String`.
///
/// # Examples
///
/// ```
/// use xstd::prelude::*;
///
/// assert_eq!("", vec![].unwords());
/// assert_eq!("one", vec!["one"].unwords());
/// assert_eq!("one two", vec!["one", "two"].unwords());
/// ```
pub trait Unwords {
    fn unwords(&self) -> String;
}
impl Unwords for Vec<&str> {
    fn unwords(&self) -> String {
        self.iter()
            .intersperse(&" ")
            .map(|str| (*str).to_string())
            .collect()
    }
}
