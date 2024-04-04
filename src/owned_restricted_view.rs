use std::borrow::Borrow;

use crate::restrict::{Restrict, RestrictOwned, RestrictView};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnedRestrictedView<R: RestrictView + ?Sized> {
    internal: Box<R::T>,
}
impl<R: RestrictView + ?Sized> AsRef<R::T> for OwnedRestrictedView<R> {
    fn as_ref(&self) -> &R::T {
        self.internal.borrow()
    }
}
impl<R: RestrictView + ?Sized> Restrict for OwnedRestrictedView<R> {
    type T = Box<R::T>;
    type View = R::T;

    fn verify(value: &Self::View) -> bool {
        R::verify(value)
    }

    unsafe fn unchecked_restrict(value: Self::T) -> Self {
        Self { internal: value }
    }

    fn unrestrict(self) -> Self::T {
        self.internal
    }
}
impl<R: RestrictView> Borrow<R> for OwnedRestrictedView<R> {
    fn borrow(&self) -> &R {
        unsafe { R::unchecked_restrict(self.internal.borrow()) }
    }
}
impl<R: RestrictView> RestrictOwned for OwnedRestrictedView<R> {
    type Borrowed = R;
}
