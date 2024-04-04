use std::borrow::Borrow;

use crate::restrict::{Restrict, RestrictOwned, RestrictView};

pub struct NonEmptyString(String);
impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        self.0.borrow()
    }
}
impl Restrict for NonEmptyString {
    type T = String;
    type View = str;

    fn verify(value: &Self::View) -> bool {
        NonEmptyStr::verify(value)
    }

    unsafe fn unchecked_restrict(value: Self::T) -> Self {
        Self(value)
    }

    fn unrestrict(self) -> Self::T {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonEmptyStr(str);
impl AsRef<str> for NonEmptyStr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl RestrictView for NonEmptyStr {
    type T = str;

    fn verify(value: &Self::T) -> bool {
        !value.is_empty()
    }

    unsafe fn unchecked_restrict(value: &Self::T) -> &Self {
        &*(value as *const str as *const Self)
    }
}

impl Borrow<NonEmptyStr> for NonEmptyString {
    fn borrow(&self) -> &NonEmptyStr {
        unsafe { NonEmptyStr::unchecked_restrict(self.0.borrow()) }
    }
}
impl RestrictOwned for NonEmptyString {
    type Borrowed = NonEmptyStr;
}
