use std::borrow::Borrow;

use crate::restrict::{Restrict, RestrictOwned, RestrictView};

pub struct NonEmptyString {
    internal: String,
}
impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        self.internal.borrow()
    }
}
impl Restrict for NonEmptyString {
    type T = String;
    type View = str;

    fn verify(value: &Self::View) -> bool {
        NonEmptyStr::verify(value)
    }

    unsafe fn unchecked_restrict(value: Self::T) -> Self {
        Self { internal: value }
    }

    fn unrestrict(self) -> Self::T {
        self.internal
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonEmptyStr {
    internal: str,
}
impl AsRef<str> for NonEmptyStr {
    fn as_ref(&self) -> &str {
        &self.internal
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
        unsafe { NonEmptyStr::unchecked_restrict(self.internal.borrow()) }
    }
}
impl RestrictOwned for NonEmptyString {
    type Borrowed = NonEmptyStr;
}
