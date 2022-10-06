//! This module shows an example of code generated by the macro. **IT MUST NOT BE USED OUTSIDE THIS
//! CRATE**.

bitflags! {
    /// This is the same `Flags` struct defined in the [crate level example](../index.html#example).
    /// Note that this struct is just for documentation purposes only, it must not be used outside
    /// this crate.
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
    }
}

/// This is the same internal field available as `self.0` on bitflags types.
/// These types aren't reachable by callers of `bitflags!`, they don't appear in the API of your
/// crate, but you can still interact with them through `self.0` in the module that defines the
/// bitflags type.
///
/// You can use this example as a reference for what methods are available to all internal bitflags
/// fields if you want to add custom functionality to your bitflags types.
///
/// Note that this struct is just for documentation purposes only, it must not be used outside
/// this crate.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FlagsField {
    bits: u32,
}

pub struct FlagsIter(FlagsIterRaw);

impl FlagsIter {
    const fn new(flags: &FlagsField) -> Self {
        FlagsIter(flags.iter_raw())
    }
}

pub struct FlagsIterRaw {
    idx: usize,
    source: FlagsField,
    state: FlagsField,
}

impl FlagsIterRaw {
    const fn new(flags: &FlagsField) -> Self {
        FlagsIterRaw {
            idx: 0,
            source: *flags,
            state: *flags,
        }
    }
}

__impl_internal_bitflags! {
    FlagsField: u32, FlagsIter, FlagsIterRaw {
        A;
        B;
        C;
        ABC;
    }
}

impl crate::__private::InternalFlags for FlagsField {
    type PublicFlags = Flags;
}

impl crate::__private::InternalIter for FlagsField {
    type Iter = FlagsIter;
    type IterRaw = FlagsIterRaw;
}
