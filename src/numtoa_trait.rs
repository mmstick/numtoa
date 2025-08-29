use crate::numtoa_core::*;

/// Converts a number into a string representation, storing the conversion into a mutable byte slice.
pub trait NumToA {
    /// Given a base for encoding and a mutable byte slice, write the number into the byte slice and return the
    /// indice where the inner string begins. The inner string can be extracted by slicing the byte slice from
    /// that indice.
    ///
    /// # Panics
    /// If the supplied buffer is smaller than the number of bytes needed to write the integer, this will panic.
    /// On debug builds, this function will perform a check on base 10 conversions to ensure that the input array
    /// is large enough to hold the largest possible value in digits.
    ///
    /// # Example
    /// ```
    /// use numtoa::NumToA;
    /// use std::io::{self, Write};
    ///
    /// let stdout = io::stdout();
    /// let stdout = &mut io::stdout();
    ///
    /// // Allocate a buffer that will be reused in each iteration.
    /// let mut buffer = [0u8; 20];
    /// 
    /// let number = 15325;
    /// let _ = stdout.write(number.numtoa(10, &mut buffer));
    /// 
    /// let number = 1241;
    /// let _ = stdout.write(number.numtoa(10, &mut buffer));
    /// 
    /// assert_eq!(12345.numtoa(10, &mut buffer), b"12345");
    /// ```
    fn numtoa(self, base: Self, string: &mut [u8]) -> &[u8];

    /// Convenience method for quickly getting a string from the input's array buffer.
    fn numtoa_str(self, base: Self, buf: &mut [u8]) -> &str;
}

macro_rules! impl_numtoa_trait {
    (
        $type_name:ty,
        $core_function_name:ident,
        $str_function_name:ident
    ) => {
        impl NumToA for $type_name {
            fn numtoa(self, base: $type_name, string: &mut [u8]) -> &[u8] {
                $core_function_name(self, base, string)                
            }

            fn numtoa_str(self, base: $type_name, buf: &mut [u8]) -> &str {
                $str_function_name(self, base, buf)
            }
        }
    };
}

impl_numtoa_trait!(i8,numtoa_i8,numtoa_i8_str);
impl_numtoa_trait!(i16,numtoa_i16,numtoa_i16_str);
impl_numtoa_trait!(i32,numtoa_i32,numtoa_i32_str);
impl_numtoa_trait!(i64,numtoa_i64,numtoa_i64_str);
impl_numtoa_trait!(i128,numtoa_i128,numtoa_i128_str);
impl_numtoa_trait!(isize,numtoa_isize,numtoa_isize_str);
impl_numtoa_trait!(u8,numtoa_u8,numtoa_u8_str);
impl_numtoa_trait!(u16,numtoa_u16,numtoa_u16_str);
impl_numtoa_trait!(u32,numtoa_u32,numtoa_u32_str);
impl_numtoa_trait!(u64,numtoa_u64,numtoa_u64_str);
impl_numtoa_trait!(u128,numtoa_u128,numtoa_u128_str);
impl_numtoa_trait!(usize,numtoa_usize,numtoa_usize_str);


#[test]
fn str_convenience_trait() {
    assert_eq!("256123", 256123.numtoa_str(10, &mut [0u8; 20]));
}


#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn base10_u8_array_too_small_trait() {
    let _ = 0u8.numtoa(10, &mut [0u8; 2]);
}


#[test]
fn base10_u8_array_just_right_trait() {
    let _ = 0u8.numtoa(10, &mut [0u8; 3]);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn base10_i8_array_too_small() {
    let mut buffer = [0u8; 3];
    let _ = 0i8.numtoa(10, &mut buffer);
}

#[test]
fn base10_i8_array_just_right() {
    let mut buffer = [0u8; 4];
    assert_eq!((-127i8).numtoa(10, &mut buffer), b"-127");
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn base10_i16_array_too_small() {
    let mut buffer = [0u8; 5];
    let _ = 0i16.numtoa(10, &mut buffer);
}

#[test]
fn base10_i16_array_just_right() {
    let mut buffer = [0u8; 6];
    assert_eq!((-12768i16).numtoa(10, &mut buffer), b"-12768");
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn base10_u16_array_too_small() {
    let mut buffer = [0u8; 4];
    let _ = 0u16.numtoa(10, &mut buffer);
}

#[test]
fn base10_u16_array_just_right() {
    let mut buffer = [0u8; 5];
    let _ = 0u16.numtoa(10, &mut buffer);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn base10_i32_array_too_small() {
    let mut buffer = [0u8; 10];
    let _ = 0i32.numtoa(10, &mut buffer);
}

#[test]
fn base10_i32_array_just_right() {
    let mut buffer = [0u8; 11];
    let _ = 0i32.numtoa(10, &mut buffer);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn base10_u32_array_too_small() {
    let mut buffer = [0u8; 9];
    let _ = 0u32.numtoa(10, &mut buffer);
}

#[test]
fn base10_u32_array_just_right() {
    let mut buffer = [0u8; 10];
    let _ = 0u32.numtoa(10, &mut buffer);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn base10_i64_array_too_small() {
    let mut buffer = [0u8; 18];
    let _ = 0i64.numtoa(10, &mut buffer);
}

#[test]
fn base10_i64_array_just_right() {
    let mut buffer = [0u8; 19];
    let _ = 0i64.numtoa(10, &mut buffer);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn base10_u64_array_too_small() {
    let mut buffer = [0u8; 19];
    let _ = 0u64.numtoa(10, &mut buffer);
}

#[test]
fn base10_u64_array_just_right() {
    let mut buffer = [0u8; 20];
    let _ = 0u64.numtoa(10, &mut buffer);
}

#[test]
fn base10_i8_all_core() {
    for i in i8::MIN..i8::MAX {
        let _ = numtoa_i8(i, 10, &mut [0u8; 4]);
    }
}

#[test]
fn base10_i8_all_trait() {
    for i in i8::MIN..i8::MAX {
        let _ = i.numtoa(10, &mut [0u8; 4]);
    }
}

#[test]
fn base16_i8_all_trait() {
    for i in i8::MIN..i8::MAX {
        let _ = i.numtoa(16, &mut [0u8; 3]);
    }
}

#[test]
fn base10_u8_all_trait() {
    for i in u8::MIN..u8::MAX {
        let _ = i.numtoa(10, &mut [0u8; 3]);
    }
}

#[test]
fn base16_u8_all_trait() {
    for i in u8::MIN..u8::MAX {
        let _ = i.numtoa(16, &mut [0u8; 3]);
    }
}


#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn base10_i128_array_too_small() {
    let mut buffer = [0u8; 38];
    let _ = 0i128.numtoa(10, &mut buffer);
}

#[test]
fn base10_i128_array_just_right() {
    let mut buffer = [0u8; 39];
    let _ = 0i128.numtoa(10, &mut buffer);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn base10_u128_array_too_small() {
    let mut buffer = [0u8; 38];
    let _ = 0u128.numtoa(10, &mut buffer);
}

#[test]
fn base10_u128_array_just_right() {
    let mut buffer = [0u8; 39];
    let _ = 0u128.numtoa(10, &mut buffer);
}

#[test]
fn base8_min_signed_number() {
    let mut buffer = [0u8; 50];
    assert_eq!((-128i8).numtoa(8, &mut buffer), b"-200");
    assert_eq!((-32768i16).numtoa(8, &mut buffer), b"-100000");
    assert_eq!((-2147483648i32).numtoa(8, &mut buffer), b"-20000000000");
    assert_eq!((-9223372036854775808i64).numtoa(8, &mut buffer), b"-1000000000000000000000");
    assert_eq!((i128::MIN).numtoa(8, &mut buffer), b"-2000000000000000000000000000000000000000000");
}

#[test]
fn base16_min_signed_number() {
    let mut buffer = [0u8; 40];
    assert_eq!((-128i8).numtoa(16, &mut buffer), b"-80");
    assert_eq!((-32768i16).numtoa(16, &mut buffer), b"-8000");
    assert_eq!((-2147483648i32).numtoa(16, &mut buffer), b"-80000000");
    assert_eq!((-9223372036854775808i64).numtoa(16, &mut buffer), b"-8000000000000000");
    assert_eq!((i128::MIN).numtoa(16, &mut buffer), b"-80000000000000000000000000000000");
}