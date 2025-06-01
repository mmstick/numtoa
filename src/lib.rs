//! The standard library provides a convenient method of converting numbers into strings, but these strings are
//! heap-allocated. If you have an application which needs to convert large volumes of numbers into strings, but don't
//! want to pay the price of heap allocation, this crate provides an efficient `no_std`-compatible method of heaplessly converting numbers
//! into their string representations, storing the representation within a reusable byte array.
//!
//! In addition to supporting the standard base 10 conversion, this implementation allows you to select the base of
//! your choice. Therefore, if you want a binary representation, set the base to 2. If you want hexadecimal, set the
//! base to 16.
//!
//! # Convenience Example
//!
//! ```
//! use numtoa::NumToA;
//!
//! let mut buf = [0u8; 20];
//! let mut string = String::new();
//!
//! for number in (1..10) {
//!     string.push_str(number.numtoa_str(10, &mut buf));
//!     string.push('\n');
//! }
//!
//! println!("{}", string);
//! ```
//!
//! ## Base 10 Example
//! ```
//! use numtoa::NumToA;
//! use std::io::{self, Write};
//!
//! let stdout = io::stdout();
//! let mut stdout = stdout.lock();
//! let mut buffer = [0u8; 20];
//!
//! let number: u32 = 162392;
//! let _ = stdout.write(number.numtoa(10, &mut buffer));
//! let _ = stdout.write(b"\n");
//! assert_eq!(number.numtoa(10, &mut buffer), b"162392");
//!
//! let number: i32 = -6235;
//! let _ = stdout.write(number.numtoa(10, &mut buffer));
//! let _ = stdout.write(b"\n");
//! assert_eq!(number.numtoa(10, &mut buffer), b"-6235");
//!
//! let number: i8 = -128;
//! let _ = stdout.write(number.numtoa(10, &mut buffer));
//! let _ = stdout.write(b"\n");
//! assert_eq!(number.numtoa(10, &mut buffer), b"-128");
//!
//! let number: i8 = 53;
//! let _ = stdout.write(number.numtoa(10, &mut buffer));
//! let _ = stdout.write(b"\n");
//! assert_eq!(number.numtoa(10, &mut buffer), b"53");
//!
//! let number: i16 = -256;
//! let _ = stdout.write(number.numtoa(10, &mut buffer));
//! let _ = stdout.write(b"\n");
//! assert_eq!(number.numtoa(10, &mut buffer), b"-256");
//!
//! let number: i16 = -32768;
//! let _ = stdout.write(number.numtoa(10, &mut buffer));
//! let _ = stdout.write(b"\n");
//! assert_eq!(number.numtoa(10, &mut buffer), b"-32768");
//!
//! let number: u64 = 35320842;
//! let _ = stdout.write(number.numtoa(10, &mut buffer));
//! let _ = stdout.write(b"\n");
//! assert_eq!(number.numtoa(10, &mut buffer), b"35320842");
//!
//! let number: u64 = 18446744073709551615;
//! let _ = stdout.write(number.numtoa(10, &mut buffer));
//! let _ = stdout.write(b"\n");
//! assert_eq!(number.numtoa(10, &mut buffer), b"18446744073709551615");
//! ```

#![no_std]
use core::mem::size_of;
use core::str;

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

// A lookup table to prevent the need for conditional branching
// The value of the remainder of each step will be used as the index
const LOOKUP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// A lookup table optimized for decimal lookups. Each two indices represents one possible number.
const DEC_LOOKUP: &[u8; 200] = b"0001020304050607080910111213141516171819\
                                 2021222324252627282930313233343536373839\
                                 4041424344454647484950515253545556575859\
                                 6061626364656667686970717273747576777879\
                                 8081828384858687888990919293949596979899";

macro_rules! copy_3_dec_lut_bytes {
    ($to:ident,$to_index:expr,$lut_index:expr) => {
        $to[($to_index as usize) % $to.len()] = DEC_LOOKUP[($lut_index as usize) % DEC_LOOKUP.len()];
        $to[($to_index as usize+1) % $to.len()] = DEC_LOOKUP[($lut_index as usize+1) % DEC_LOOKUP.len()];
        $to[($to_index as usize+2) % $to.len()] = DEC_LOOKUP[($lut_index as usize+2) % DEC_LOOKUP.len()];
    };
}

macro_rules! base_10 {
    ($number:ident, $index:ident, $string:ident) => {
        // Decode four characters at the same time
        while $number > 9999 {
            let rem = ($number % 10000) as u16;
            let (frst, scnd) = ((rem / 100) * 2, (rem % 100) * 2);
            copy_3_dec_lut_bytes!($string, $index-3, frst);
            copy_3_dec_lut_bytes!($string, $index-1, scnd);
            $index = $index.wrapping_sub(4);
            $number /= 10000;
        }
        if $number > 999 {
            let (frst, scnd) = (($number / 100) * 2, ($number % 100) * 2);
            copy_3_dec_lut_bytes!($string, $index-3, frst);
            copy_3_dec_lut_bytes!($string, $index-1, scnd);
            $index = $index.wrapping_sub(4);
        } else if $number > 99 {
            let section = ($number as u16 / 10) * 2;
            copy_3_dec_lut_bytes!($string, $index-2, section);
            $string[$index] = LOOKUP[($number % 10) as usize];
            $index = $index.wrapping_sub(3);
        } else if $number > 9 {
            $number *= 2;
            copy_3_dec_lut_bytes!($string, $index-1, $number);
            $index = $index.wrapping_sub(2);
        } else {
            $string[$index] = LOOKUP[$number as usize];
            $index = $index.wrapping_sub(1);
        }
    }
}

macro_rules! impl_unsigned_numtoa_for {
    (
        $type_name:ty,
        $core_function_name:ident,
        $str_function_name:ident
    ) => {

        pub const fn $core_function_name(mut num: $type_name, base: $type_name, string: &mut [u8]) -> &[u8] {
            // Check if the buffer is large enough and panic on debug builds if it isn't
            if cfg!(debug_assertions) {
                if base == 10 {
                    match size_of::<$type_name>() {
                        2 => debug_assert!(string.len() >= 5,  "u16 base 10 conversions require at least 5 bytes"),
                        4 => debug_assert!(string.len() >= 10, "u32 base 10 conversions require at least 10 bytes"),
                        8 => debug_assert!(string.len() >= 20, "u64 base 10 conversions require at least 20 bytes"),
                        16 => debug_assert!(string.len() >= 39, "u128 base 10 conversions require at least 39 bytes"),
                        _ => unreachable!()
                    }
                }
            }

            let mut index = string.len() - 1;
            if num == 0 {
                string[index] = b'0';
                return string.split_at(index).1;
            }

            if base == 10 {
                // Convert using optimized base 10 algorithm
                base_10!(num, index, string);
            } else {
                while num != 0 {
                    let rem = num % base;
                    string[index] = LOOKUP[rem as usize];
                    index = index.wrapping_sub(1);
                    num /= base;
                }
            }

            string.split_at(index.wrapping_add(1)).1
        }

        pub const fn $str_function_name(num: $type_name, base: $type_name, string: &mut [u8]) -> &str {
            unsafe { core::str::from_utf8_unchecked($core_function_name(num, base, string)) }
        }

        impl NumToA for $type_name {
            fn numtoa(self, base: $type_name, string: &mut [u8]) -> &[u8] {
               $core_function_name(self, base, string)
            }
            fn numtoa_str(self, base: $type_name, buf: &mut [u8]) -> &str {
                $str_function_name(self, base, buf)
            }
        }

    }
}

macro_rules! impl_signed_numtoa_for {
    (
        $type_name:ty,
        $core_function_name:ident,
        $str_function_name:ident
    ) => {
        impl NumToA for $type_name {
            fn numtoa(mut self, base: $type_name, string: &mut [u8]) -> &[u8] {
                if cfg!(debug_assertions) {
                    if base == 10 {
                        match size_of::<$type_name>() {
                            2 => debug_assert!(string.len() >= 6,  "i16 base 10 conversions require at least 6 bytes"),
                            4 => debug_assert!(string.len() >= 11, "i32 base 10 conversions require at least 11 bytes"),
                            8 => debug_assert!(string.len() >= 19, "i64 base 10 conversions require at least 19 bytes"),
                            16 => debug_assert!(string.len() >= 39, "i128 base 10 conversions require at least 39 bytes"),
                            _ => unreachable!()
                        }
                    }
                }

                let mut index = string.len() - 1;
                let mut is_negative = false;

                if self < 0 {
                    is_negative = true;
                    self = match self.checked_abs() {
                        Some(value) => value,
                        None        => {
                            let value = <$type_name>::max_value();
                            string[index] = LOOKUP[((value % base + 1) % base) as usize];
                            index -= 1;
                            value / base + ((value % base == base - 1) as $type_name)
                        }
                    };
                } else if self == 0 {
                    string[index] = b'0';
                    return string.split_at(index).1;
                }

                if base == 10 {
                    // Convert using optimized base 10 algorithm
                    base_10!(self, index, string);
                } else {
                    while self != 0 {
                        let rem = self % base;
                        string[index] = LOOKUP[rem as usize];
                        index = index.wrapping_sub(1);
                        self /= base;
                    }
                }

                if is_negative {
                    string[index] = b'-';
                    index = index.wrapping_sub(1);
                }

                string.split_at(index.wrapping_add(1)).1
            }

    
            fn numtoa_str(self, base: $type_name, buf: &mut [u8]) -> &str {
                unsafe { str::from_utf8_unchecked(self.numtoa(base, buf)) }
            }
        }
    }
}

impl_signed_numtoa_for!(i16,numtoa_i16,numtoa_str_i16);
impl_signed_numtoa_for!(i32,numtoa_i32,numtoa_str_i32);
impl_signed_numtoa_for!(i64,numtoa_i64,numtoa_str_i64);
impl_signed_numtoa_for!(i128,numtoa_i128,numtoa_str_i128);
impl_signed_numtoa_for!(isize,numtoa_isize,numtoa_str_isize);
impl_unsigned_numtoa_for!(u16,numtoa_u16,numtoa_str_u16);
impl_unsigned_numtoa_for!(u32,numtoa_u32,numtoa_str_u32);
impl_unsigned_numtoa_for!(u64,numtoa_u64,numtoa_str_u64);
impl_unsigned_numtoa_for!(u128,numtoa_u128,numtoa_str_u128);
impl_unsigned_numtoa_for!(usize,numtoa_usize,numtoa_str_usize);

pub const fn numtoa_i8(mut num: i8, base: i8, string: &mut [u8]) -> &[u8] {
    if cfg!(debug_assertions) {
        if base == 10 {
            debug_assert!(string.len() >= 4, "i8 conversions need at least 4 bytes");
        }
    }

    let mut index = string.len() - 1;
    let mut is_negative = false;

    if num < 0 {
        is_negative = true;
        num = match num.checked_abs() {
            Some(value) => value,
            None        => {
                let value = <i8>::max_value();
                string[index] = LOOKUP[((value % base + 1) % base) as usize];
                index -= 1;
                value / base + ((value % base == base - 1) as i8)
            }
        };
    } else if num == 0 {
        string[index] = b'0';
        return string.split_at(index).1;
    }

    if base == 10 {
        if num > 99 {
            let section = (num / 10) * 2;
            copy_3_dec_lut_bytes!(string, index-2, section);
            string[index] = LOOKUP[(num % 10) as usize];
            index = index.wrapping_sub(3);
        } else if num > 9 {
            let idx = num as usize * 2;
            copy_3_dec_lut_bytes!(string, index-1, idx);
            index = index.wrapping_sub(2);
            } else {
            string[index] = LOOKUP[num as usize];
            index = index.wrapping_sub(1);
        }
    } else {
        while num != 0 {
            let rem = num % base;
            string[index] = LOOKUP[rem as usize];
            index = index.wrapping_sub(1);
            num /= base;
        }
    }

    if is_negative {
        string[index] = b'-';
        index = index.wrapping_sub(1);
    }

    string.split_at(index.wrapping_add(1)).1
}

pub const fn numtoa_str_i8(num: i8, base: i8, string: &mut [u8]) -> &str {
    unsafe { str::from_utf8_unchecked(numtoa_i8(num, base, string)) }
}

impl NumToA for i8 {
    fn numtoa(self, base: i8, string: &mut [u8]) -> &[u8] {
        numtoa_i8(self, base, string)
    }

    fn numtoa_str(self, base: Self, buf: &mut [u8]) -> &str {
        numtoa_str_i8(self, base, buf)
    }
}

pub const fn numtoa_u8(mut num: u8, base: u8, string: &mut [u8]) -> &[u8] {
    if cfg!(debug_assertions) {
        if base == 10 {
            debug_assert!(string.len() >= 3, "u8 conversions need at least 3 bytes");
        }
    }

    let mut index = string.len() - 1;
    if num == 0 {
        string[index] = b'0';
        return string.split_at(index).1;
    }

    if base == 10 {
        if num > 99 {
            let section = (num / 10) * 2;
            copy_3_dec_lut_bytes!(string, index-2, section);
            string[index] = LOOKUP[(num % 10) as usize];
            index = index.wrapping_sub(3);
        } else if num > 9 {
            num *= 2;
            copy_3_dec_lut_bytes!(string, index-1, num);
            index = index.wrapping_sub(2);
        } else {
            string[index] = LOOKUP[num as usize];
            index = index.wrapping_sub(1);
        }
    } else {
        while num != 0 {
            let rem = num % base;
            string[index] = LOOKUP[rem as usize];
            index = index.wrapping_sub(1);
            num /= base;
        }
    }

    string.split_at(1).1
}

pub const fn numtoa_str_u8(num: u8, base: u8, string: &mut [u8]) -> &str {
    unsafe { str::from_utf8_unchecked(numtoa_u8(num, base, string)) }
}

impl NumToA for u8 {
    fn numtoa(self, base: u8, string: &mut [u8]) -> &[u8] {
        numtoa_u8(self, base, string)
    }

    fn numtoa_str(self, base: Self, buf: &mut [u8]) -> &str {
        numtoa_str_u8(self, base, buf)
    }
}

#[test]
fn str_convenience() {
    let mut buffer = [0u8; 20];
    assert_eq!("256123", 256123.numtoa_str(10, &mut buffer));
}

#[test]
#[should_panic]
fn base10_u8_array_too_small() {
    let mut buffer = [0u8; 2];
    let _ = 0u8.numtoa(10, &mut buffer);
}

#[test]
fn base10_u8_array_just_right() {
    let mut buffer = [0u8; 3];
    let _ = 0u8.numtoa(10, &mut buffer);
}

#[test]
#[should_panic]
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
fn base10_i8_all() {
    let mut buffer = [0u8; 4];
    for i in i8::MIN..i8::MAX {
        let _ = i.numtoa(10, &mut buffer);
    }
}

#[test]
fn base10_u8_all() {
    let mut buffer = [0u8; 3];
    for i in u8::MIN..u8::MAX {
        let _ = i.numtoa(10, &mut buffer);
    }
}

#[test]
#[should_panic]
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
