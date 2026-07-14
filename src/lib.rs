#![no_std]
use core::ops::{
    Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
    BitAnd, BitOr, BitXor, Shl, Shr,
    BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign,
    Not, Neg
};
use core::fmt::{Debug, Display, Binary, Octal, UpperHex, LowerHex};
use core::hash::Hash;
use core::str::FromStr;

// traits primitve integers share
pub trait IntTraits:
    Sized + Copy + Default + Hash + Ord + Eq + PartialOrd + PartialEq + Send + Sync + 'static
    + Display + Debug + Binary + Octal + LowerHex + UpperHex
    + FromStr
    + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Rem<Output = Self>
    + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign
    + Not<Output = Self>
    + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self>
    + BitAndAssign + BitOrAssign + BitXorAssign
    + Shl<u32, Output = Self> + Shr<u32, Output = Self>
    + Shl<usize, Output = Self> + Shr<usize, Output = Self>
    + ShlAssign<u32> + ShrAssign<u32>
    + ShlAssign<usize> + ShrAssign<usize>
{}

macro_rules! int_traits {
    ($($type:ty),*) => {
        $(impl IntTraits for $type {})*
    };
}
int_traits!(u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize);
// methods and consts primitve integers share
pub trait IntOps: Sized+IntTraits {
    type SignedSelf: IntTraits + Neg<Output = Self::SignedSelf>;
    type UnsignedSelf: IntTraits;
    type BytesArray: AsRef<[u8]> + AsMut<[u8]> + Copy + Clone + Send + Sync + 'static; // this is always: [u8; Self::BYTES]. this fixes: "generic parameters may not be used in const operations"

    const BITS: u32;
    const BYTES: usize;
    const MAX: Self;
    const MIN: Self;
    const ONE: Self;
    const ZERO: Self;

    fn checked_add(self, rhs: Self) -> Option<Self>;
    //fn checked_add_signed(self, rhs: Self::SignedSelf) -> Option<Self>;
    //fn checked_add_unsigned(self, rhs: Self::UnsignedSelf) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_div_euclid(self, rhs: Self) -> Option<Self>;
    fn checked_ilog(self, base: Self) -> Option<u32>;
    fn checked_ilog10(self) -> Option<u32>;
    fn checked_ilog2(self) -> Option<u32>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_neg(self) -> Option<Self>;
    //fn checked_next_multiple_of(self, rhs: Self) -> Option<Self>;
    fn checked_pow(self, exp: u32) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;
    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;
    fn checked_shl(self, rhs: u32) -> Option<Self>;
    fn checked_shr(self, rhs: u32) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    //fn checked_sub_unsigned(self, rhs: Self::UnsignedSelf) -> Option<Self>;
    fn count_ones(self) -> u32;
    fn count_zeros(self) -> u32;
    //fn div_ceil(self, rhs: Self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self;
    //fn div_floor(self, rhs: Self) -> Self;
    fn from_be(x: Self) -> Self;
    fn from_be_bytes(bytes: Self::BytesArray) -> Self;
    fn from_le(x: Self) -> Self;
    fn from_le_bytes(bytes: Self::BytesArray) -> Self;
    fn from_ne_bytes(bytes: Self::BytesArray) -> Self;
    fn ilog(self, base: Self) -> u32;
    fn ilog10(self) -> u32;
    fn ilog2(self) -> u32;
    fn leading_ones(self) -> u32;
    fn leading_zeros(self) -> u32;
    //fn next_multiple_of(self, rhs: Self) -> Self;
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    //fn overflowing_add_signed(self, rhs: Self::SignedSelf) -> (Self, bool);
    //fn overflowing_add_unsigned(self, rhs: Self::UnsignedSelf) -> (Self, bool);
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
    fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    fn overflowing_neg(self) -> (Self, bool);
    fn overflowing_pow(self, exp: u32) -> (Self, bool);
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);
    fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);
    fn overflowing_shl(self, rhs: u32) -> (Self, bool);
    fn overflowing_shr(self, rhs: u32) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    //fn overflowing_sub_unsigned(self, rhs: Self::UnsignedSelf) -> (Self, bool);
    fn rem_euclid(self, rhs: Self) -> Self;
    fn reverse_bits(self) -> Self;
    fn rotate_left(self, n: u32) -> Self;
    fn rotate_right(self, n: u32) -> Self;
    fn saturating_add(self, rhs: Self) -> Self;
    //fn saturating_add_signed(self, rhs: Self::SignedSelf) -> Self;
    //fn saturating_add_unsigned(self, rhs: Self::UnsignedSelf) -> Self;
    fn saturating_div(self, rhs: Self) -> Self;
    fn saturating_mul(self, rhs: Self) -> Self;
    fn saturating_pow(self, exp: u32) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
    //fn saturating_sub_unsigned(self, rhs: Self::UnsignedSelf) -> Self;
    fn strict_add(self, rhs: Self) -> Self;
    //fn strict_add_signed(self, rhs: Self::SignedSelf) -> Self;
    //fn strict_add_unsigned(self, rhs: Self::UnsignedSelf) -> Self;
    fn strict_div(self, rhs: Self) -> Self;
    fn strict_div_euclid(self, rhs: Self) -> Self;
    fn strict_mul(self, rhs: Self) -> Self;
    fn strict_neg(self) -> Self;
    fn strict_pow(self, exp: u32) -> Self;
    fn strict_rem(self, rhs: Self) -> Self;
    fn strict_shl(self, rhs: u32) -> Self;
    fn strict_shr(self, rhs: u32) -> Self;
    fn strict_sub(self, rhs: Self) -> Self;
    //fn strict_sub_unsigned(self, rhs: Self::UnsignedSelf) -> Self;
    fn swap_bytes(self) -> Self;
    fn to_be(self) -> Self;
    fn to_be_bytes(self) -> Self::BytesArray;
    fn to_le(self) -> Self;
    fn to_le_bytes(self) -> Self::BytesArray;
    fn to_ne_bytes(self) -> Self::BytesArray;
    fn trailing_ones(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn wrapping_add(self, rhs: Self) -> Self;
    //fn wrapping_add_signed(self, rhs: Self::SignedSelf) -> Self;
    //fn wrapping_add_unsigned(self, rhs: Self::UnsignedSelf) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn wrapping_div_euclid(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_neg(self) -> Self;
    fn wrapping_pow(self, exp: u32) -> Self;
    fn wrapping_rem(self, rhs: Self) -> Self;
    fn wrapping_rem_euclid(self, rhs: Self) -> Self;
    fn wrapping_shl(self, rhs: u32) -> Self;
    fn wrapping_shr(self, rhs: u32) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    //fn wrapping_sub_unsigned(self, rhs: Self::UnsignedSelf) -> Self;
}
macro_rules! intops {
    ( $( ($type:ty, $type_signed:ty, $type_unsigned:ty) ),* ) => {
        $(
            impl IntOps for $type {
                type SignedSelf = $type_signed;
                type UnsignedSelf = $type_unsigned;
                type BytesArray = [u8; Self::BYTES];

                const BITS: u32 = Self::BITS;
                const BYTES: usize = Self::BITS as usize/8;
                const MAX: Self = Self::MAX;
                const MIN: Self = Self::MIN;
                const ONE: Self = 1;
                const ZERO: Self = 0;

                #[inline] fn checked_add(self, rhs: Self) -> Option<Self> { <$type>::checked_add(self, rhs) }
                //#[inline] fn checked_add_signed(self, rhs: Self::SignedSelf) -> Option<Self> { <$type>::checked_add_signed(self, rhs) }
                //#[inline] fn checked_add_unsigned(self, rhs: Self::UnsignedSelf) -> Option<Self> { <$type>::checked_add_unsigned(self, rhs) }
                #[inline] fn checked_div(self, rhs: Self) -> Option<Self> { <$type>::checked_div(self, rhs) }
                #[inline] fn checked_div_euclid(self, rhs: Self) -> Option<Self> { <$type>::checked_div_euclid(self, rhs) }
                #[inline] fn checked_ilog(self, base: Self) -> Option<u32> { <$type>::checked_ilog(self, base) }
                #[inline] fn checked_ilog10(self) -> Option<u32> { <$type>::checked_ilog10(self) }
                #[inline] fn checked_ilog2(self) -> Option<u32> { <$type>::checked_ilog2(self) }
                #[inline] fn checked_mul(self, rhs: Self) -> Option<Self> { <$type>::checked_mul(self, rhs) }
                #[inline] fn checked_neg(self) -> Option<Self> { <$type>::checked_neg(self) }
                //#[inline] fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> { <$type>::checked_next_multiple_of(self, rhs) }
                #[inline] fn checked_pow(self, exp: u32) -> Option<Self> { <$type>::checked_pow(self, exp) }
                #[inline] fn checked_rem(self, rhs: Self) -> Option<Self> { <$type>::checked_rem(self, rhs) }
                #[inline] fn checked_rem_euclid(self, rhs: Self) -> Option<Self> { <$type>::checked_rem_euclid(self, rhs) }
                #[inline] fn checked_shl(self, rhs: u32) -> Option<Self> { <$type>::checked_shl(self, rhs) }
                #[inline] fn checked_shr(self, rhs: u32) -> Option<Self> { <$type>::checked_shr(self, rhs) }
                #[inline] fn checked_sub(self, rhs: Self) -> Option<Self> { <$type>::checked_sub(self, rhs) }
                //#[inline] fn checked_sub_unsigned(self, rhs: Self::UnsignedSelf) -> Option<Self> { <$type>::checked_sub_unsigned(self, rhs) }
                #[inline] fn count_ones(self) -> u32 { <$type>::count_ones(self) }
                #[inline] fn count_zeros(self) -> u32 { <$type>::count_zeros(self) }
                //#[inline] fn div_ceil(self, rhs: Self) -> Self { <$type>::div_ceil(self, rhs) }
                #[inline] fn div_euclid(self, rhs: Self) -> Self { <$type>::div_euclid(self, rhs) }
                //#[inline] fn div_floor(self, rhs: Self) -> Self { <$type>::div_floor(self, rhs) }
                #[inline] fn from_be(x: Self) -> Self { <$type>::from_be(x) }
                #[inline] fn from_be_bytes(bytes: Self::BytesArray) -> Self { <$type>::from_be_bytes(bytes) }
                #[inline] fn from_le(x: Self) -> Self { <$type>::from_le(x) }
                #[inline] fn from_le_bytes(bytes: Self::BytesArray) -> Self { <$type>::from_le_bytes(bytes) }
                #[inline] fn from_ne_bytes(bytes: Self::BytesArray) -> Self { <$type>::from_ne_bytes(bytes) }
                #[inline] fn ilog(self, base: Self) -> u32 { <$type>::ilog(self, base) }
                #[inline] fn ilog10(self) -> u32 { <$type>::ilog10(self) }
                #[inline] fn ilog2(self) -> u32 { <$type>::ilog2(self) }
                #[inline] fn leading_ones(self) -> u32 { <$type>::leading_ones(self) }
                #[inline] fn leading_zeros(self) -> u32 { <$type>::leading_zeros(self) }
                //#[inline] fn next_multiple_of(self, rhs: Self) -> Self { <$type>::next_multiple_of(self, rhs) }
                #[inline] fn overflowing_add(self, rhs: Self) -> (Self, bool) { <$type>::overflowing_add(self, rhs) }
                //#[inline] fn overflowing_add_signed(self, rhs: Self::SignedSelf) -> (Self, bool) { <$type>::overflowing_add_signed(self, rhs) }
                //#[inline] fn overflowing_add_unsigned(self, rhs: Self::UnsignedSelf) -> (Self, bool) { <$type>::overflowing_add_unsigned(self, rhs) }
                #[inline] fn overflowing_div(self, rhs: Self) -> (Self, bool) { <$type>::overflowing_div(self, rhs) }
                #[inline] fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) { <$type>::overflowing_div_euclid(self, rhs) }
                #[inline] fn overflowing_mul(self, rhs: Self) -> (Self, bool) { <$type>::overflowing_mul(self, rhs) }
                #[inline] fn overflowing_neg(self) -> (Self, bool) { <$type>::overflowing_neg(self) }
                #[inline] fn overflowing_pow(self, exp: u32) -> (Self, bool) { <$type>::overflowing_pow(self, exp) }
                #[inline] fn overflowing_rem(self, rhs: Self) -> (Self, bool) { <$type>::overflowing_rem(self, rhs) }
                #[inline] fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) { <$type>::overflowing_rem_euclid(self, rhs) }
                #[inline] fn overflowing_shl(self, rhs: u32) -> (Self, bool) { <$type>::overflowing_shl(self, rhs) }
                #[inline] fn overflowing_shr(self, rhs: u32) -> (Self, bool) { <$type>::overflowing_shr(self, rhs) }
                #[inline] fn overflowing_sub(self, rhs: Self) -> (Self, bool) { <$type>::overflowing_sub(self, rhs) }
                //#[inline] fn overflowing_sub_unsigned(self, rhs: Self::UnsignedSelf) -> (Self, bool) { <$type>::overflowing_sub_unsigned(self, rhs) }
                #[inline] fn rem_euclid(self, rhs: Self) -> Self { <$type>::rem_euclid(self, rhs) }
                #[inline] fn reverse_bits(self) -> Self { <$type>::reverse_bits(self) }
                #[inline] fn rotate_left(self, n: u32) -> Self { <$type>::rotate_left(self, n) }
                #[inline] fn rotate_right(self, n: u32) -> Self { <$type>::rotate_right(self, n) }
                #[inline] fn saturating_add(self, rhs: Self) -> Self { <$type>::saturating_add(self, rhs) }
                //#[inline] fn saturating_add_signed(self, rhs: Self::SignedSelf) -> Self { <$type>::saturating_add_signed(self, rhs) }
                //#[inline] fn saturating_add_unsigned(self, rhs: Self::UnsignedSelf) -> Self { <$type>::saturating_add_unsigned(self, rhs) }
                #[inline] fn saturating_div(self, rhs: Self) -> Self { <$type>::saturating_div(self, rhs) }
                #[inline] fn saturating_mul(self, rhs: Self) -> Self { <$type>::saturating_mul(self, rhs) }
                #[inline] fn saturating_pow(self, exp: u32) -> Self { <$type>::saturating_pow(self, exp) }
                #[inline] fn saturating_sub(self, rhs: Self) -> Self { <$type>::saturating_sub(self, rhs) }
                //#[inline] fn saturating_sub_unsigned(self, rhs: Self::UnsignedSelf) -> Self { <$type>::saturating_sub_unsigned(self, rhs) }
                #[inline] fn strict_add(self, rhs: Self) -> Self { <$type>::strict_add(self, rhs) }
                //#[inline] fn strict_add_signed(self, rhs: Self::SignedSelf) -> Self { <$type>::strict_add_signed(self, rhs) }
                //#[inline] fn strict_add_unsigned(self, rhs: Self::UnsignedSelf) -> Self { <$type>::strict_add_unsigned(self, rhs) }
                #[inline] fn strict_div(self, rhs: Self) -> Self { <$type>::strict_div(self, rhs) }
                #[inline] fn strict_div_euclid(self, rhs: Self) -> Self { <$type>::strict_div_euclid(self, rhs) }
                #[inline] fn strict_mul(self, rhs: Self) -> Self { <$type>::strict_mul(self, rhs) }
                #[inline] fn strict_neg(self) -> Self { <$type>::strict_neg(self) }
                #[inline] fn strict_pow(self, exp: u32) -> Self { <$type>::strict_pow(self, exp) }
                #[inline] fn strict_rem(self, rhs: Self) -> Self { <$type>::strict_rem(self, rhs) }
                #[inline] fn strict_shl(self, rhs: u32) -> Self { <$type>::strict_shl(self, rhs) }
                #[inline] fn strict_shr(self, rhs: u32) -> Self { <$type>::strict_shr(self, rhs) }
                #[inline] fn strict_sub(self, rhs: Self) -> Self { <$type>::strict_sub(self, rhs) }
                //#[inline] fn strict_sub_unsigned(self, rhs: Self::UnsignedSelf) -> Self { <$type>::strict_sub_unsigned(self, rhs) }
                #[inline] fn swap_bytes(self) -> Self { <$type>::swap_bytes(self) }
                #[inline] fn to_be(self) -> Self { <$type>::to_be(self) }
                #[inline] fn to_be_bytes(self) -> Self::BytesArray { <$type>::to_be_bytes(self) }
                #[inline] fn to_le(self) -> Self { <$type>::to_le(self) }
                #[inline] fn to_le_bytes(self) -> Self::BytesArray { <$type>::to_le_bytes(self) }
                #[inline] fn to_ne_bytes(self) -> Self::BytesArray { <$type>::to_ne_bytes(self) }
                #[inline] fn trailing_ones(self) -> u32 { <$type>::trailing_ones(self) }
                #[inline] fn trailing_zeros(self) -> u32 { <$type>::trailing_zeros(self) }
                #[inline] fn wrapping_add(self, rhs: Self) -> Self { <$type>::wrapping_add(self, rhs) }
                //#[inline] fn wrapping_add_signed(self, rhs: Self::SignedSelf) -> Self { <$type>::wrapping_add_signed(self, rhs) }
                //#[inline] fn wrapping_add_unsigned(self, rhs: Self::UnsignedSelf) -> Self { <$type>::wrapping_add_unsigned(self, rhs) }
                #[inline] fn wrapping_div(self, rhs: Self) -> Self { <$type>::wrapping_div(self, rhs) }
                #[inline] fn wrapping_div_euclid(self, rhs: Self) -> Self { <$type>::wrapping_div_euclid(self, rhs) }
                #[inline] fn wrapping_mul(self, rhs: Self) -> Self { <$type>::wrapping_mul(self, rhs) }
                #[inline] fn wrapping_neg(self) -> Self { <$type>::wrapping_neg(self) }
                #[inline] fn wrapping_pow(self, exp: u32) -> Self { <$type>::wrapping_pow(self, exp) }
                #[inline] fn wrapping_rem(self, rhs: Self) -> Self { <$type>::wrapping_rem(self, rhs) }
                #[inline] fn wrapping_rem_euclid(self, rhs: Self) -> Self { <$type>::wrapping_rem_euclid(self, rhs) }
                #[inline] fn wrapping_shl(self, rhs: u32) -> Self { <$type>::wrapping_shl(self, rhs) }
                #[inline] fn wrapping_shr(self, rhs: u32) -> Self { <$type>::wrapping_shr(self, rhs) }
                #[inline] fn wrapping_sub(self, rhs: Self) -> Self { <$type>::wrapping_sub(self, rhs) }
                //#[inline] fn wrapping_sub_unsigned(self, rhs: Self::UnsignedSelf) -> Self { <$type>::wrapping_sub_unsigned(self, rhs) }
            }
        )*
    };
}
intops!(
    (u8,i8,u8),(u16,i16,u16),(u32,i32,u32),(u64,i64,u64),(u128,i128,u128),
    (i8,i8,u8),(i16,i16,u16),(i32,i32,u32),(i64,i64,u64),(i128,i128,u128),
    (usize,isize,usize),(isize,isize,usize)
    );
