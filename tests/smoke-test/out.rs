#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use bitflags::bitflags;
pub struct Flags(<Flags as ::bitflags::__private::PublicFlags>::InternalFlags);
#[automatically_derived]
impl ::core::clone::Clone for Flags {
    #[inline]
    fn clone(&self) -> Flags {
        let _: ::core::clone::AssertParamIsClone<
            <Flags as ::bitflags::__private::PublicFlags>::InternalFlags,
        >;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Flags {}
#[automatically_derived]
impl ::core::fmt::Debug for Flags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Flags", &&self.0)
    }
}
impl ::bitflags::__private::core::fmt::Binary for Flags {
    fn fmt(
        &self,
        f: &mut ::bitflags::__private::core::fmt::Formatter,
    ) -> ::bitflags::__private::core::fmt::Result {
        ::bitflags::__private::core::fmt::Binary::fmt(&self.0, f)
    }
}
impl ::bitflags::__private::core::fmt::Octal for Flags {
    fn fmt(
        &self,
        f: &mut ::bitflags::__private::core::fmt::Formatter,
    ) -> ::bitflags::__private::core::fmt::Result {
        ::bitflags::__private::core::fmt::Octal::fmt(&self.0, f)
    }
}
impl ::bitflags::__private::core::fmt::LowerHex for Flags {
    fn fmt(
        &self,
        f: &mut ::bitflags::__private::core::fmt::Formatter,
    ) -> ::bitflags::__private::core::fmt::Result {
        ::bitflags::__private::core::fmt::LowerHex::fmt(&self.0, f)
    }
}
impl ::bitflags::__private::core::fmt::UpperHex for Flags {
    fn fmt(
        &self,
        f: &mut ::bitflags::__private::core::fmt::Formatter,
    ) -> ::bitflags::__private::core::fmt::Result {
        ::bitflags::__private::core::fmt::UpperHex::fmt(&self.0, f)
    }
}
#[allow(
    dead_code,
    deprecated,
    unused_doc_comments,
    unused_attributes,
    unused_mut,
    non_upper_case_globals
)]
impl Flags {
    pub const A: Self = Self::from_bits_retain(0b00000001);
    pub const B: Self = Self::from_bits_retain(0b00000010);
    pub const C: Self = Self::from_bits_retain(0b00000100);
    pub const ABC: Self =
        Self::from_bits_retain(Flags::A.bits() | Flags::B.bits() | Flags::C.bits());
    /// Returns an empty set of flags.
    #[inline]
    pub const fn empty() -> Self {
        Self(<Flags as ::bitflags::__private::PublicFlags>::InternalFlags::empty())
    }
    /// Returns the set containing all flags.
    #[inline]
    pub const fn all() -> Self {
        Self(<Flags as ::bitflags::__private::PublicFlags>::InternalFlags::all())
    }
    /// Returns the raw value of the flags currently stored.
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0.bits()
    }
    /// Convert from underlying bit representation, unless that
    /// representation contains bits that do not correspond to a flag.
    #[inline]
    pub const fn from_bits(bits: u32) -> ::bitflags::__private::core::option::Option<Self> {
        match <Flags as ::bitflags::__private::PublicFlags>::InternalFlags::from_bits(bits) {
            ::bitflags::__private::core::option::Option::Some(bits) => {
                ::bitflags::__private::core::option::Option::Some(Self(bits))
            }
            ::bitflags::__private::core::option::Option::None => {
                ::bitflags::__private::core::option::Option::None
            }
        }
    }
    /// Convert from underlying bit representation, dropping any bits
    /// that do not correspond to flags.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(<Flags as ::bitflags::__private::PublicFlags>::InternalFlags::from_bits_truncate(bits))
    }
    /// Convert from underlying bit representation, preserving all
    /// bits (even those not corresponding to a defined flag).
    ///
    /// # Safety
    ///
    /// The caller of the `bitflags!` macro can choose to allow or
    /// disallow extra bits for their bitflags type.
    ///
    /// The caller of `from_bits_retain()` has to ensure that
    /// all bits correspond to a defined flag or that extra bits
    /// are valid for this bitflags type.
    #[inline]
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(<Flags as ::bitflags::__private::PublicFlags>::InternalFlags::from_bits_retain(bits))
    }
    /// Returns `true` if no flags are currently stored.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Returns `true` if all flags are currently set.
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.0.is_all()
    }
    /// Returns `true` if there are flags common to both `self` and `other`.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        self.0.intersects(other.0)
    }
    /// Returns `true` if all of the flags in `other` are contained within `self`.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        self.0.contains(other.0)
    }
    /// Inserts the specified flags in-place.
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0.insert(other.0)
    }
    /// Removes the specified flags in-place.
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0.remove(other.0)
    }
    /// Toggles the specified flags in-place.
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0.toggle(other.0)
    }
    /// Inserts or removes the specified flags depending on the passed value.
    #[inline]
    pub fn set(&mut self, other: Self, value: bool) {
        self.0.set(other.0, value)
    }
    /// Returns the intersection between the flags in `self` and
    /// `other`.
    ///
    /// Specifically, the returned set contains only the flags which are
    /// present in *both* `self` *and* `other`.
    ///
    /// This is equivalent to using the `&` operator (e.g.
    /// [`ops::BitAnd`]), as in `flags & other`.
    ///
    /// [`ops::BitAnd`]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0.intersection(other.0))
    }
    /// Returns the union of between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags which are
    /// present in *either* `self` *or* `other`, including any which are
    /// present in both (see [`Self::symmetric_difference`] if that
    /// is undesirable).
    ///
    /// This is equivalent to using the `|` operator (e.g.
    /// [`ops::BitOr`]), as in `flags | other`.
    ///
    /// [`ops::BitOr`]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0.union(other.0))
    }
    /// Returns the difference between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags present in
    /// `self`, except for the ones present in `other`.
    ///
    /// It is also conceptually equivalent to the "bit-clear" operation:
    /// `flags & !other` (and this syntax is also supported).
    ///
    /// This is equivalent to using the `-` operator (e.g.
    /// [`ops::Sub`]), as in `flags - other`.
    ///
    /// [`ops::Sub`]: https://doc.rust-lang.org/std/ops/trait.Sub.html
    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0.difference(other.0))
    }
    /// Returns the [symmetric difference][sym-diff] between the flags
    /// in `self` and `other`.
    ///
    /// Specifically, the returned set contains the flags present which
    /// are present in `self` or `other`, but that are not present in
    /// both. Equivalently, it contains the flags present in *exactly
    /// one* of the sets `self` and `other`.
    ///
    /// This is equivalent to using the `^` operator (e.g.
    /// [`ops::BitXor`]), as in `flags ^ other`.
    ///
    /// [sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    /// [`ops::BitXor`]: https://doc.rust-lang.org/std/ops/trait.BitXor.html
    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0.symmetric_difference(other.0))
    }
    /// Returns the complement of this set of flags.
    ///
    /// Specifically, the returned set contains all the flags which are
    /// not set in `self`, but which are allowed for this type.
    ///
    /// Alternatively, it can be thought of as the set difference
    /// between [`Self::all()`] and `self` (e.g. `Self::all() - self`)
    ///
    /// This is equivalent to using the `!` operator (e.g.
    /// [`ops::Not`]), as in `!flags`.
    ///
    /// [`Self::all()`]: Self::all
    /// [`ops::Not`]: https://doc.rust-lang.org/std/ops/trait.Not.html
    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(self.0.complement())
    }
    /// Returns an iterator over set flags and their names.
    pub fn iter(
        self,
    ) -> impl ::bitflags::__private::core::iter::Iterator<Item = (&'static str, Self)> {
        use ::bitflags::__private::core::iter::Iterator as _;
        self.0
            .iter()
            .map(|(name, bits)| (name, Self::from_bits_retain(bits)))
    }
}
impl ::bitflags::__private::core::ops::BitOr for Flags {
    type Output = Self;
    /// Returns the union of the two sets of flags.
    #[inline]
    fn bitor(self, other: Flags) -> Self {
        self.union(other)
    }
}
impl ::bitflags::__private::core::ops::BitOrAssign for Flags {
    /// Adds the set of flags.
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 = self.0.union(other.0);
    }
}
impl ::bitflags::__private::core::ops::BitXor for Flags {
    type Output = Self;
    /// Returns the left flags, but with all the right flags toggled.
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl ::bitflags::__private::core::ops::BitXorAssign for Flags {
    /// Toggles the set of flags.
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.0 = self.0.symmetric_difference(other.0);
    }
}
impl ::bitflags::__private::core::ops::BitAnd for Flags {
    type Output = Self;
    /// Returns the intersection between the two sets of flags.
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl ::bitflags::__private::core::ops::BitAndAssign for Flags {
    /// Disables all flags disabled in the set.
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 = self.0.intersection(other.0);
    }
}
impl ::bitflags::__private::core::ops::Sub for Flags {
    type Output = Self;
    /// Returns the set difference of the two sets of flags.
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl ::bitflags::__private::core::ops::SubAssign for Flags {
    /// Disables all flags enabled in the set.
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.difference(other.0);
    }
}
impl ::bitflags::__private::core::ops::Not for Flags {
    type Output = Self;
    /// Returns the complement of this set of flags.
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl ::bitflags::__private::core::iter::Extend<Flags> for Flags {
    fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
        &mut self,
        iterator: T,
    ) {
        for item in iterator {
            self.insert(item)
        }
    }
}
impl ::bitflags::__private::core::iter::FromIterator<Flags> for Flags {
    fn from_iter<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
        iterator: T,
    ) -> Self {
        use ::bitflags::__private::core::iter::Extend;
        let mut result = Self::empty();
        result.extend(iterator);
        result
    }
}
impl ::bitflags::BitFlags for Flags {
    type Bits = u32;
    fn empty() -> Self {
        Flags::empty()
    }
    fn all() -> Self {
        Flags::all()
    }
    fn bits(&self) -> u32 {
        Flags::bits(self)
    }
    fn from_bits(bits: u32) -> ::bitflags::__private::core::option::Option<Flags> {
        Flags::from_bits(bits)
    }
    fn from_bits_truncate(bits: u32) -> Flags {
        Flags::from_bits_truncate(bits)
    }
    fn from_bits_retain(bits: u32) -> Flags {
        Flags::from_bits_retain(bits)
    }
    fn is_empty(&self) -> bool {
        Flags::is_empty(self)
    }
    fn is_all(&self) -> bool {
        Flags::is_all(self)
    }
    fn intersects(&self, other: Flags) -> bool {
        Flags::intersects(self, other)
    }
    fn contains(&self, other: Flags) -> bool {
        Flags::contains(self, other)
    }
    fn insert(&mut self, other: Flags) {
        Flags::insert(self, other)
    }
    fn remove(&mut self, other: Flags) {
        Flags::remove(self, other)
    }
    fn toggle(&mut self, other: Flags) {
        Flags::toggle(self, other)
    }
    fn set(&mut self, other: Flags, value: bool) {
        Flags::set(self, other, value)
    }
}
impl ::bitflags::__private::ImplementedByBitFlagsMacro for Flags {}
const _: () = {
    mod __internal_bitflags {
        #[repr(transparent)]
        pub struct InternalFlags {
            bits: u32,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for InternalFlags {
            #[inline]
            fn clone(&self) -> InternalFlags {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for InternalFlags {}
        impl ::core::marker::StructuralPartialEq for InternalFlags {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for InternalFlags {
            #[inline]
            fn eq(&self, other: &InternalFlags) -> bool {
                self.bits == other.bits
            }
            #[inline]
            fn ne(&self, other: &InternalFlags) -> bool {
                self.bits != other.bits
            }
        }
        impl ::core::marker::StructuralEq for InternalFlags {}
        #[automatically_derived]
        impl ::core::cmp::Eq for InternalFlags {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for InternalFlags {
            #[inline]
            fn partial_cmp(
                &self,
                other: &InternalFlags,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.bits, &other.bits)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for InternalFlags {
            #[inline]
            fn cmp(&self, other: &InternalFlags) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.bits, &other.bits)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for InternalFlags {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.bits, state)
            }
        }
        impl ::bitflags::__private::core::default::Default for InternalFlags {
            #[inline]
            fn default() -> Self {
                InternalFlags::empty()
            }
        }
        impl ::bitflags::__private::core::fmt::Debug for InternalFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let mut first = true;
                for (name, _) in self.iter() {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    f.write_str(name)?;
                }
                let extra_bits = self.bits & !Self::all().bits;
                if extra_bits != <u32 as ::bitflags::__private::Bits>::EMPTY {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    f.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_lower_hex(&extra_bits)],
                        &[::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 4u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        }],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ))?;
                }
                if first {
                    f.write_str("empty")?;
                }
                ::bitflags::__private::core::fmt::Result::Ok(())
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for InternalFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::Binary::fmt(&self.bits(), f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for InternalFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::Octal::fmt(&self.bits(), f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for InternalFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::LowerHex::fmt(&self.bits(), f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for InternalFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::UpperHex::fmt(&self.bits(), f)
            }
        }
        #[allow(
            dead_code,
            deprecated,
            unused_doc_comments,
            unused_attributes,
            unused_mut,
            non_upper_case_globals
        )]
        impl InternalFlags {
            #[inline]
            pub const fn empty() -> Self {
                Self {
                    bits: <u32 as ::bitflags::__private::Bits>::EMPTY,
                }
            }
            #[inline]
            pub const fn all() -> Self {
                Self::from_bits_truncate(<u32 as ::bitflags::__private::Bits>::ALL)
            }
            #[inline]
            pub const fn bits(&self) -> u32 {
                self.bits
            }
            #[inline]
            pub const fn from_bits(bits: u32) -> ::bitflags::__private::core::option::Option<Self> {
                let truncated = Self::from_bits_truncate(bits).bits;
                if truncated == bits {
                    ::bitflags::__private::core::option::Option::Some(Self { bits })
                } else {
                    ::bitflags::__private::core::option::Option::None
                }
            }
            #[inline]
            pub const fn from_bits_truncate(bits: u32) -> Self {
                if bits == <u32 as ::bitflags::__private::Bits>::EMPTY {
                    return Self { bits };
                }
                let mut truncated = <u32 as ::bitflags::__private::Bits>::EMPTY;
                if bits
                    & <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::A.bits()
                    == <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::A
                        .bits()
                {
                    truncated |=
                        <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::A
                            .bits()
                }
                if bits
                    & <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::B.bits()
                    == <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::B
                        .bits()
                {
                    truncated |=
                        <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::B
                            .bits()
                }
                if bits
                    & <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::C.bits()
                    == <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::C
                        .bits()
                {
                    truncated |=
                        <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::C
                            .bits()
                }
                if bits
                    & <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::ABC
                        .bits()
                    == <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::ABC
                        .bits()
                {
                    truncated |=
                        <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::ABC
                            .bits()
                }
                Self { bits: truncated }
            }
            #[inline]
            pub const fn from_bits_retain(bits: u32) -> Self {
                Self { bits }
            }
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.bits == Self::empty().bits
            }
            #[inline]
            pub const fn is_all(&self) -> bool {
                Self::all().bits | self.bits == self.bits
            }
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                !(Self {
                    bits: self.bits & other.bits,
                })
                .is_empty()
            }
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits
            }
            #[inline]
            pub fn insert(&mut self, other: Self) {
                self.bits |= other.bits;
            }
            #[inline]
            pub fn remove(&mut self, other: Self) {
                self.bits &= !other.bits;
            }
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.bits ^= other.bits;
            }
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self {
                    bits: self.bits & other.bits,
                }
            }
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self {
                    bits: self.bits | other.bits,
                }
            }
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self {
                    bits: self.bits & !other.bits,
                }
            }
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self {
                    bits: self.bits ^ other.bits,
                }
            }
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self::from_bits_truncate(!self.bits)
            }
            pub fn iter(
                self,
            ) -> impl ::bitflags::__private::core::iter::Iterator<Item = (&'static str, u32)>
            {
                use ::bitflags::__private::core::iter::Iterator as _;
                const NUM_FLAGS: usize = {
                    let mut num_flags = 0;
                    {
                        num_flags += 1;
                    }
                    {
                        num_flags += 1;
                    }
                    {
                        num_flags += 1;
                    }
                    {
                        num_flags += 1;
                    }
                    num_flags
                };
                const OPTIONS: [u32; NUM_FLAGS] = [
                    <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::A.bits(),
                    <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::B.bits(),
                    <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::C.bits(),
                    <InternalFlags as ::bitflags::__private::InternalFlags>::PublicFlags::ABC
                        .bits(),
                ];
                const OPTIONS_NAMES: [&'static str; NUM_FLAGS] = ["A", "B", "C", "ABC"];
                let mut start = 0;
                let mut state = self;
                ::bitflags::__private::core::iter::from_fn(move || {
                    if state.is_empty() || NUM_FLAGS == 0 {
                        ::bitflags::__private::core::option::Option::None
                    } else {
                        for (flag, flag_name) in OPTIONS[start..NUM_FLAGS]
                            .iter()
                            .copied()
                            .zip(OPTIONS_NAMES[start..NUM_FLAGS].iter().copied())
                        {
                            start += 1;
                            if self.contains(Self { bits: flag }) {
                                state.remove(Self { bits: flag });
                                return ::bitflags::__private::core::option::Option::Some((
                                    flag_name, flag,
                                ));
                            }
                        }
                        ::bitflags::__private::core::option::Option::None
                    }
                })
            }
        }
        impl ::bitflags::__private::InternalFlags for InternalFlags {
            type PublicFlags = Flags;
        }
        impl ::bitflags::__private::PublicFlags for Flags {
            type InternalFlags = InternalFlags;
        }
    }
};
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &[::core::fmt::ArgumentV1::new_debug(&Flags::ABC)],
        ));
    };
}
