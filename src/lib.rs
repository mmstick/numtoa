//! The standard library provides a convenient method of converting numbers into strings, but these strings are
//! heap-allocated. If you have an application which needs to convert large volumes of numbers into strings, but don't
//! want to pay the price of heap allocation, this crate provides an efficient `no_std`-compatible method of heaplessly converting numbers
//! into their string representations, storing the representation within a reusable byte array.
//!
//! In addition to supporting the standard base 10 conversion, this implementation allows you to select the base of
//! your choice. Therefore, if you want a binary representation, set the base to 2. If you want hexadecimal, set the
//! base to 16.
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
//! let mut start_indice = number.numtoa(10, &mut buffer);
//! let _ = stdout.write(&buffer[start_indice..]);
//! let _ = stdout.write(b"\n");
//! assert_eq!(&buffer[start_indice..], b"162392");
//!
//! let other_number: i32 = -6235;
//! start_indice = other_number.numtoa(10, &mut buffer);
//! let _ = stdout.write(&buffer[start_indice..]);
//! let _ = stdout.write(b"\n");
//! assert_eq!(&buffer[start_indice..], b"-6235");
//!
//! let other_number: i16 = -256;
//! start_indice = other_number.numtoa(10, &mut buffer);
//! let _ = stdout.write(&buffer[start_indice..]);
//! let _ = stdout.write(b"\n");
//! assert_eq!(&buffer[start_indice..], b"-256");
//!
//! let other_number: i16 = -32768;
//! start_indice = other_number.numtoa(10, &mut buffer);
//! let _ = stdout.write(&buffer[start_indice..]);
//! let _ = stdout.write(b"\n");
//! assert_eq!(&buffer[start_indice..], b"-32768");
//!
//! let large_num: u64 = 35320842;
//! start_indice = large_num.numtoa(10, &mut buffer);
//! let _ = stdout.write(&buffer[start_indice..]);
//! let _ = stdout.write(b"\n");
//! assert_eq!(&buffer[start_indice..], b"35320842");
//!
//! let max_u64: u64 = 18446744073709551615;
//! start_indice = max_u64.numtoa(10, &mut buffer);
//! let _ = stdout.write(&buffer[start_indice..]);
//! let _ = stdout.write(b"\n");
//! assert_eq!(&buffer[start_indice..], b"18446744073709551615");
//! ```

#![no_std]

/// Converts a number into a string representation, storing the conversion into a mutable byte slice.
pub trait NumToA<T> {
    /// Given a base for encoding and a mutable byte slice, write the number into the byte slice and return the
    /// indice where the inner string begins. The inner string can be extracted by slicing the byte slice from
    /// that indice.
    ///
    /// # Panics
    /// If the supplied buffer is smaller than the number of bytes needed to write the integer, this will panic.
    ///
    /// # Example
    /// ```
    /// use numtoa::NumToA;
    /// use std::io::{self, Write};
    ///
    /// let stdout = io::stdout();
    /// let stdout = &mut io::stdout();
    ///
    /// let mut buffer = [0u8; 20];
    /// let number = 15325;
    /// let start_indice = number.numtoa(10, &mut buffer);
    /// let _ = stdout.write(&buffer[start_indice..]);
    /// assert_eq!(&buffer[start_indice..], b"15325");
    /// ```
    fn numtoa(self, base: T, string: &mut [u8]) -> usize;
}

// A lookup table to prevent the need for conditional branching
// The value of the remainder of each step will be used as the index
const LOOKUP: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// A lookup table optimized for decimal lookups. Each two indices represents one possible number.
const DEC_LOOKUP: &'static [u8; 200] = b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899";

macro_rules! base_10_rev {
    ($number:ident, $index:ident, $string:ident) => {
        // Decode four characters at the same time
        while $number > 9999 {
            let rem = $number % 10000;
            let (first_section, second_section) = ((rem / 100) as usize * 2, (rem % 100) as usize * 2);
            $string[$index-3..$index-1].copy_from_slice(&DEC_LOOKUP[first_section..first_section+2]);
            $string[$index-1..$index+1].copy_from_slice(&DEC_LOOKUP[second_section..second_section+2]);
            $index = $index.wrapping_sub(4);
            $number /= 10000;
        }

        if $number > 999 {
            let (first_section, second_section) = (($number / 100) as usize * 2, ($number % 100) as usize * 2);
            $string[$index-3..$index-1].copy_from_slice(&DEC_LOOKUP[first_section..first_section+2]);
            $string[$index-1..$index+1].copy_from_slice(&DEC_LOOKUP[second_section..second_section+2]);
            $index = $index.wrapping_sub(4);
        } else if $number > 99 {
            let section = ($number / 10) as usize * 2;
            $string[$index-2..$index].copy_from_slice(&DEC_LOOKUP[section..section+2]);
            $string[$index] = LOOKUP[($number % 10) as usize];
            $index = $index.wrapping_sub(3);
        } else if $number > 9 {
            $string[$index-1] = LOOKUP[($number / 10) as usize];
            $string[$index]   = LOOKUP[($number % 10) as usize];
            $index = $index.wrapping_sub(2);
        } else {
            $string[$index] = LOOKUP[$number as usize];
            $index = $index.wrapping_sub(1);
        }
    }
}

macro_rules! impl_unsized_numtoa_for {
    ($t:ty) => {
        impl NumToA<$t> for $t {
            fn numtoa(mut self, base: $t, string: &mut [u8]) -> usize {
                let mut index = string.len() - 1;
                if self == 0 {
                    string[index] = b'0';
                    return index;
                }

                if base == 10 {
                    base_10_rev!(self, index, string);
                } else {
                    while self != 0 {
                        let rem = self % base;
                        string[index] = LOOKUP[rem as usize];
                        index = index.wrapping_sub(1);
                        self /= base;
                    }
                }

                index.wrapping_add(1)
            }
        }
    }
}

macro_rules! impl_sized_numtoa_for {
    ($t:ty) => {
        impl NumToA<$t> for $t {
            fn numtoa(mut self, base: $t, string: &mut [u8]) -> usize {
                let mut index = string.len() - 1;
                let mut is_negative = false;

                if self < 0 {
                    is_negative = true;
                    self = match self.checked_abs() {
                        Some(value) => value,
                        None        => {
                            let value = <$t>::max_value();
                            string[index] = LOOKUP[(value % 10 + 1) as usize];
                            index -= 1;
                            value / 10
                        }
                    };
                } else if self == 0 {
                    string[index] = b'0';
                    return index;
                }

                if base == 10 {
                    base_10_rev!(self, index, string);
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
                    index -= 1;
                }

                index.wrapping_add(1)
            }
        }

    }
}

impl_sized_numtoa_for!(i16);
impl_sized_numtoa_for!(i32);
impl_sized_numtoa_for!(i64);
impl_sized_numtoa_for!(isize);
impl_unsized_numtoa_for!(u16);
impl_unsized_numtoa_for!(u32);
impl_unsized_numtoa_for!(u64);
impl_unsized_numtoa_for!(usize);

impl NumToA<i8> for i8 {
    fn numtoa(mut self, base: i8, string: &mut [u8]) -> usize {
        let mut index = string.len() - 1;
        let mut is_negative = false;

        if self < 0 {
            is_negative = true;
            self = match self.checked_abs() {
                Some(value) => value,
                None        => {
                    let value = <i8>::max_value();
                    string[index] = LOOKUP[(value % 10 + 1) as usize];
                    index -= 1;
                    value / 10
                }
            };
        } else if self == 0 {
            string[index] = b'0';
            return index;
        }

        while self != 0 {
            let rem = self % base;
            string[index] = LOOKUP[rem as usize];
            index = index.wrapping_sub(1);
            self /= base;
        }

        if is_negative {
            string[index] = b'-';
            index -= 1;
        }

        index.wrapping_add(1)
    }
}

impl NumToA<u8> for u8 {
    fn numtoa(mut self, base: u8, string: &mut [u8]) -> usize {
        let mut index = string.len() - 1;
        if self == 0 {
            string[index] = b'0';
            return index;
        }

        while self != 0 {
            let rem = self % base;
            string[index] = LOOKUP[rem as usize];
            index = index.wrapping_sub(1);
            self /= base;
        }

        index.wrapping_add(1)
    }
}
