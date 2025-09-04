use core::{
    fmt::{Debug, Display, Formatter},
    ops::Deref,
};

use crate::numtoa_core::*;

/// API to convert numbers into ascii string in base N. Infallible & const-friendly. Returns an [AsciiNumber] of fixed size based on the selected base and numeric type.
pub struct BaseN<const N: usize> {}

/// The immutable result of a [BaseN] number conversion to ascii, containing a string containing at most N bytes / N ascii characters.
#[derive(Clone, Copy)]
pub struct AsciiNumber<const N: usize> {
    string: [u8; N],
    start: usize,
}

impl<const N: usize> AsciiNumber<N> {
    pub const MAX_CAPACITY: usize = N;

    #[allow(dead_code)]
    const MIN_LEN_ASSERTION: () = assert!(N > 0);

    pub const ZERO: AsciiNumber<N> = {
        let mut string = [0_u8; N];
        string[N - 1] = b'0';
        let start = N - 1;
        AsciiNumber { string, start }
    };

    pub const ONE: AsciiNumber<N> = {
        let mut string = [0_u8; N];
        string[N - 1] = b'0';
        let start = N - 1;
        AsciiNumber { string, start }
    };

    /// Get the ascii representation of the number as a byte slice
    pub const fn as_slice(&self) -> &[u8] {
        self.string.split_at(self.start).1
    }
    /// Get the ascii representation of the number as a string slice
    pub const fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(Self::as_slice(self)) }
    }
    /// Consume this AsciiNumber to return the underlying buffer & string start position
    pub const fn into_inner(self) -> ([u8; N], usize) {
        (self.string, self.start)
    }
}

impl<const N: usize> PartialEq for AsciiNumber<N> {
    fn eq(&self, other: &AsciiNumber<N>) -> bool {
        PartialEq::eq(self.as_slice(), other.as_slice())
    }
}

impl<const N: usize> Eq for AsciiNumber<N> {}

impl<const N: usize> Default for AsciiNumber<N> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const N: usize> Deref for AsciiNumber<N> {
    type Target = str;
    fn deref(&self) -> &<Self as Deref>::Target {
        self.as_str()
    }
}

impl<const N: usize> Display for AsciiNumber<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        Display::fmt(self.as_str(), f)
    }
}

impl<const N: usize> Debug for AsciiNumber<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        Debug::fmt(self.as_str(), f)
    }
}

macro_rules! impl_numtoa_const_for_base_on_type {
    (
    $type_name:ty,
    $base:expr,
    $core_function_name:ident,
    $base_n_function_name:ident,
    $padded_function_name:ident,
    $required_space_constant_name:ident,
    $needed_space_bytes:expr
) => {
        pub const $required_space_constant_name: usize = $needed_space_bytes;

        pub const fn $base_n_function_name(
            num: $type_name,
        ) -> AsciiNumber<{ Self::$required_space_constant_name }> {
            let mut string = [0_u8; Self::$required_space_constant_name];
            let start = Self::$required_space_constant_name
                - $core_function_name(num, $base, &mut string).len();
            return AsciiNumber { string, start };
        }

        pub const fn $padded_function_name<const LENGTH: usize>(
            num: $type_name,
            padding: u8,
        ) -> AsciiNumber<LENGTH> {
            const { assert!(LENGTH >= { Self::$required_space_constant_name }) }
            let mut string = [padding; LENGTH];
            let _ = $core_function_name(num, $base, &mut string);
            return AsciiNumber { string, start: 0 };
        }
    };
}

macro_rules! impl_numtoa_const_for_base_n {
    ($base_value:expr) => {
        impl BaseN<$base_value> {
            impl_numtoa_const_for_base_on_type!(
                u8,
                $base_value,
                numtoa_u8,
                u8,
                u8_padded,
                REQUIRED_SPACE_U8,
                required_space($base_value as u128, u8::MAX as u128, false)
            );
            impl_numtoa_const_for_base_on_type!(
                u16,
                $base_value,
                numtoa_u16,
                u16,
                u16_padded,
                REQUIRED_SPACE_U16,
                required_space($base_value as u128, u16::MAX as u128, false)
            );
            impl_numtoa_const_for_base_on_type!(
                u32,
                $base_value,
                numtoa_u32,
                u32,
                u32_padded,
                REQUIRED_SPACE_U32,
                required_space($base_value as u128, u32::MAX as u128, false)
            );
            impl_numtoa_const_for_base_on_type!(
                u64,
                $base_value,
                numtoa_u64,
                u64,
                u64_padded,
                REQUIRED_SPACE_U64,
                required_space($base_value as u128, u64::MAX as u128, false)
            );
            impl_numtoa_const_for_base_on_type!(
                u128,
                $base_value,
                numtoa_u128,
                u128,
                u128_padded,
                REQUIRED_SPACE_U128,
                required_space($base_value as u128, u128::MAX as u128, false)
            );
            impl_numtoa_const_for_base_on_type!(
                usize,
                $base_value,
                numtoa_usize,
                usize,
                usize_padded,
                REQUIRED_SPACE_USIZE,
                required_space($base_value as u128, usize::MAX as u128, false)
            );
            impl_numtoa_const_for_base_on_type!(
                i8,
                $base_value,
                numtoa_i8,
                i8,
                i8_padded,
                REQUIRED_SPACE_I8,
                required_space($base_value as u128, i8::MIN.unsigned_abs() as u128, true)
            );
            impl_numtoa_const_for_base_on_type!(
                i16,
                $base_value,
                numtoa_i16,
                i16,
                i16_padded,
                REQUIRED_SPACE_I16,
                required_space($base_value as u128, i16::MIN.unsigned_abs() as u128, true)
            );
            impl_numtoa_const_for_base_on_type!(
                i32,
                $base_value,
                numtoa_i32,
                i32,
                i32_padded,
                REQUIRED_SPACE_I32,
                required_space($base_value as u128, i32::MIN.unsigned_abs() as u128, true)
            );
            impl_numtoa_const_for_base_on_type!(
                i64,
                $base_value,
                numtoa_i64,
                i64,
                i64_padded,
                REQUIRED_SPACE_I64,
                required_space($base_value as u128, i64::MIN.unsigned_abs() as u128, true)
            );
            impl_numtoa_const_for_base_on_type!(
                i128,
                $base_value,
                numtoa_i128,
                i128,
                i128_padded,
                REQUIRED_SPACE_I128,
                required_space($base_value as u128, i128::MIN.unsigned_abs() as u128, true)
            );
            impl_numtoa_const_for_base_on_type!(
                isize,
                $base_value,
                numtoa_isize,
                isize,
                isize_padded,
                REQUIRED_SPACE_ISIZE,
                required_space($base_value as u128, isize::MIN.unsigned_abs() as u128, true)
            );
        }
    };
}

impl_numtoa_const_for_base_n!(2);
impl_numtoa_const_for_base_n!(3);
impl_numtoa_const_for_base_n!(4);
impl_numtoa_const_for_base_n!(5);
impl_numtoa_const_for_base_n!(6);
impl_numtoa_const_for_base_n!(7);
impl_numtoa_const_for_base_n!(8);
impl_numtoa_const_for_base_n!(9);
impl_numtoa_const_for_base_n!(10);
impl_numtoa_const_for_base_n!(11);
impl_numtoa_const_for_base_n!(12);
impl_numtoa_const_for_base_n!(13);
impl_numtoa_const_for_base_n!(14);
impl_numtoa_const_for_base_n!(15);
impl_numtoa_const_for_base_n!(16);

#[test]
fn str_convenience_base2() {
    assert_eq!("111110100001111011", BaseN::<2>::i32(256123).as_str());
}

#[test]
fn str_convenience_base8() {
    assert_eq!("764173", BaseN::<8>::i32(256123).as_str());
}

#[test]
fn str_convenience_base10() {
    assert_eq!("256123", BaseN::<10>::i32(256123).as_str());
}

#[test]
fn str_convenience_base10_padded() {
    assert_eq!(
        "00000000000000256123",
        BaseN::<10>::i32_padded::<20>(256123, b'0').as_str()
    );
}

#[test]
fn str_convenience_base16() {
    assert_eq!("3E87B", BaseN::<16>::i32(256123).as_str());
}

#[test]
fn str_convenience_base16_padded() {
    assert_eq!(
        "0000000000000003E87B",
        BaseN::<16>::i32_padded::<20>(256123, b'0').as_str()
    );
}

#[test]
fn str_convenience_wacky_padding() {
    assert_eq!(
        "##############-3E87B",
        BaseN::<16>::i32_padded::<20>(-256123, b'#').as_str()
    );
    assert_eq!(
        "@@@@@@@@@@@-111",
        BaseN::<10>::i8_padded::<15>(-111, b'@').as_str()
    );
}

#[test]
fn base10_i8_all_base10() {
    for i in i8::MIN..i8::MAX {
        let _ = BaseN::<10>::i8(i);
    }
}

#[test]
fn base16_i8_all_base16() {
    for i in i8::MIN..i8::MAX {
        let _ = BaseN::<16>::i8(i);
    }
}

#[test]
fn base10_u8_all_base10() {
    for i in u8::MIN..u8::MAX {
        let _ = BaseN::<10>::u8(i);
    }
}

#[test]
fn base16_u8_all_base16() {
    for i in u8::MIN..u8::MAX {
        let _ = BaseN::<16>::u8(i);
    }
}
