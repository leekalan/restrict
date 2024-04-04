use std::borrow::Borrow;

use crate::restrict::Restrict;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonZeroU8(u8);
impl AsRef<u8> for NonZeroU8 {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}
impl Restrict for NonZeroU8 {
    type T = u8;
    type View = u8;

    fn verify(value: &Self::View) -> bool {
        *value != 0
    }

    unsafe fn unchecked_restrict(value: Self::T) -> Self {
        Self(value)
    }

    fn unrestrict(self) -> Self::T {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonZeroU16(u16);
impl AsRef<u16> for NonZeroU16 {
    fn as_ref(&self) -> &u16 {
        &self.0
    }
}
impl Restrict for NonZeroU16 {
    type T = u16;
    type View = u16;

    fn verify(value: &Self::View) -> bool {
        *value != 0
    }

    unsafe fn unchecked_restrict(value: Self::T) -> Self {
        Self(value)
    }

    fn unrestrict(self) -> Self::T {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonZeroU32(u32);
impl AsRef<u32> for NonZeroU32 {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
impl Restrict for NonZeroU32 {
    type T = u32;
    type View = u32;

    fn verify(value: &Self::View) -> bool {
        *value != 0
    }

    unsafe fn unchecked_restrict(value: Self::T) -> Self {
        Self(value)
    }

    fn unrestrict(self) -> Self::T {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonZeroU64(u64);
impl AsRef<u64> for NonZeroU64 {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}
impl Restrict for NonZeroU64 {
    type T = u64;
    type View = u64;

    fn verify(value: &Self::View) -> bool {
        *value != 0
    }

    unsafe fn unchecked_restrict(value: Self::T) -> Self {
        Self(value)
    }

    fn unrestrict(self) -> Self::T {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonZeroU128(u128);
impl AsRef<u128> for NonZeroU128 {
    fn as_ref(&self) -> &u128 {
        &self.0
    }
}
impl Restrict for NonZeroU128 {
    type T = u128;
    type View = u128;

    fn verify(value: &Self::View) -> bool {
        *value != 0
    }

    unsafe fn unchecked_restrict(value: Self::T) -> Self {
        Self(value)
    }

    fn unrestrict(self) -> Self::T {
        self.0
    }
}
