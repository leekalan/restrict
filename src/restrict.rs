use std::borrow::Borrow;

pub trait Restrict: Sized + AsRef<Self::View>
where
    Self::T: Borrow<Self::View>,
{
    type T;
    type View: ?Sized;

    fn verify(value: &Self::View) -> bool;
    unsafe fn unchecked_restrict(value: Self::T) -> Self;
    fn view(&self) -> &Self::View {
        self.as_ref()
    }
    fn restrict(value: Self::T) -> Option<Self> {
        if Self::verify(value.borrow()) {
            Some(unsafe { Self::unchecked_restrict(value) })
        } else {
            None
        }
    }
    fn unrestrict(self) -> Self::T;
}

pub trait RestrictView: AsRef<Self::T> {
    type T: ?Sized;

    fn verify(value: &Self::T) -> bool;
    unsafe fn unchecked_restrict(value: &Self::T) -> &Self;
    fn view(&self) -> &Self::T {
        self.as_ref()
    }
    fn restrict(value: &Self::T) -> Option<&Self> {
        if Self::verify(value) {
            Some(unsafe { Self::unchecked_restrict(value) })
        } else {
            None
        }
    }
}

pub trait RestrictOwned: Restrict + Borrow<Self::Borrowed> {
    type Borrowed: ?Sized + RestrictView<T = Self::View>;
}
