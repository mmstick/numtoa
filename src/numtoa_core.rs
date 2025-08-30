/// returns the number of bytes required for a base N number conversion
pub const fn required_space(base: u128, number: u128, negative: bool) -> usize {
    assert!(base > 0);
    if number == 0 {
        return 1 + (negative as usize)
    }
    let mut number_bytes_required = 0;
    let mut n = number;
    while n > 0 {
        n /= base as u128;
        number_bytes_required += 1;
    }
    number_bytes_required + (negative as usize)
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

// The maximum supported base given the standard alphabet
const MAX_SUPPORTED_BASE: u128 = LOOKUP.len() as u128;

macro_rules! copy_2_dec_lut_bytes {
    ($to:ident,$to_index:expr,$lut_index:expr) => {
        $to[$to_index as usize] = DEC_LOOKUP[$lut_index as usize];
        $to[$to_index as usize+1] = DEC_LOOKUP[$lut_index as usize+1];
    };
}

macro_rules! base_10 {
    ($number:ident, $index:ident, $string:ident) => {
        // Decode four characters at the same time
        while $number > 9999 {
            let rem = ($number % 10000) as u16;
            let (frst, scnd) = ((rem / 100) * 2, (rem % 100) * 2);
            copy_2_dec_lut_bytes!($string, $index-3, frst);
            copy_2_dec_lut_bytes!($string, $index-1, scnd);
            $index = $index.wrapping_sub(4);
            $number /= 10000;
        }
        if $number > 999 {
            let (frst, scnd) = (($number / 100) * 2, ($number % 100) * 2);
            copy_2_dec_lut_bytes!($string, $index-3, frst);
            copy_2_dec_lut_bytes!($string, $index-1, scnd);
            $index = $index.wrapping_sub(4);
        } else if $number > 99 {
            let section = ($number as u16 / 10) * 2;
            copy_2_dec_lut_bytes!($string, $index-2, section);
            $string[$index] = LOOKUP[($number % 10) as usize];
            $index = $index.wrapping_sub(3);
        } else if $number > 9 {
            $number *= 2;
            copy_2_dec_lut_bytes!($string, $index-1, $number);
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
                debug_assert!(base > 1 && base as u128 <= MAX_SUPPORTED_BASE, "unsupported base");
                debug_assert!(string.len() >= required_space(base as u128,<$type_name>::MAX as u128,false));
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

    }
}

macro_rules! impl_signed_numtoa_for {
    (
        $type_name:ty,
        $core_function_name:ident,
        $str_function_name:ident
    ) => {

        pub const fn $core_function_name(mut num: $type_name, base: $type_name, string: &mut [u8]) -> &[u8] {
            if cfg!(debug_assertions) {
                debug_assert!(base > 1 && base as u128 <= MAX_SUPPORTED_BASE, "unsupported base");
                debug_assert!(string.len() >= required_space(base as u128,<$type_name>::MIN.unsigned_abs() as u128,true));
            }

            let mut index = string.len() - 1;
            let mut is_negative = false;

            if num < 0 {
                is_negative = true;
                num = match num.checked_abs() {
                    Some(value) => value,
                    None        => {
                        let value = <$type_name>::max_value();
                        string[index] = LOOKUP[((value % base + 1) % base) as usize];
                        index -= 1;
                        value / base + ((value % base == base - 1) as $type_name)
                    }
                };
            } else if num == 0 {
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

            if is_negative {
                string[index] = b'-';
                index = index.wrapping_sub(1);
            }

            string.split_at(index.wrapping_add(1)).1
        }

        pub const fn $str_function_name(num: $type_name, base: $type_name, string: &mut [u8]) -> &str {
            unsafe { core::str::from_utf8_unchecked($core_function_name(num, base, string)) }
        }

    }
}

impl_signed_numtoa_for!(i16,numtoa_i16,numtoa_i16_str);
impl_signed_numtoa_for!(i32,numtoa_i32,numtoa_i32_str);
impl_signed_numtoa_for!(i64,numtoa_i64,numtoa_i64_str);
impl_signed_numtoa_for!(i128,numtoa_i128,numtoa_i128_str);
impl_signed_numtoa_for!(isize,numtoa_isize,numtoa_isize_str);
impl_unsigned_numtoa_for!(u16,numtoa_u16,numtoa_u16_str);
impl_unsigned_numtoa_for!(u32,numtoa_u32,numtoa_u32_str);
impl_unsigned_numtoa_for!(u64,numtoa_u64,numtoa_u64_str);
impl_unsigned_numtoa_for!(u128,numtoa_u128,numtoa_u128_str);
impl_unsigned_numtoa_for!(usize,numtoa_usize,numtoa_usize_str);

pub const fn numtoa_i8(mut num: i8, base: i8, string: &mut [u8]) -> &[u8] {
    if cfg!(debug_assertions) {
        debug_assert!(base > 1 && base as u128 <= MAX_SUPPORTED_BASE, "unsupported base");
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
            copy_2_dec_lut_bytes!(string, index-2, section);
            string[index] = LOOKUP[(num % 10) as usize];
            index = index.wrapping_sub(3);
        } else if num > 9 {
            let idx = num as usize * 2;
            copy_2_dec_lut_bytes!(string, index-1, idx);
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

pub const fn numtoa_i8_str(num: i8, base: i8, string: &mut [u8]) -> &str {
    unsafe { str::from_utf8_unchecked(numtoa_i8(num, base, string)) }
}

pub const fn numtoa_u8(mut num: u8, base: u8, string: &mut [u8]) -> &[u8] {
    if cfg!(debug_assertions) {
        debug_assert!(base > 1 && base as u128 <= MAX_SUPPORTED_BASE, "unsupported base");
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
            copy_2_dec_lut_bytes!(string, index-2, section);
            string[index] = LOOKUP[(num % 10) as usize];
            index = index.wrapping_sub(3);
        } else if num > 9 {
            num *= 2;
            copy_2_dec_lut_bytes!(string, index-1, num);
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

    string.split_at(index.wrapping_add(1)).1
}

pub const fn numtoa_u8_str(num: u8, base: u8, string: &mut [u8]) -> &str {
    unsafe { str::from_utf8_unchecked(numtoa_u8(num, base, string)) }
}

#[cfg(test)]
mod core_test {

    use super::*;

    fn verify_required_sizes_for_base(
        base: u128,
        expected_space_u8: usize,
        expected_space_u16: usize,
        expected_space_u32: usize,
        expected_space_u64: usize,
        expected_space_u128: usize,
        expected_space_i8: usize,
        expected_space_i16: usize,
        expected_space_i32: usize,
        expected_space_i64: usize,
        expected_space_i128: usize,
    ) {
        assert_eq!(expected_space_u8, required_space(base,u8::MAX as u128,false));
        assert_eq!(expected_space_u16, required_space(base,u16::MAX as u128,false));
        assert_eq!(expected_space_u32, required_space(base,u32::MAX as u128,false));
        assert_eq!(expected_space_u64, required_space(base,u64::MAX as u128,false));
        assert_eq!(expected_space_u128, required_space(base,u128::MAX as u128,false));
        assert_eq!(expected_space_i8, required_space(base,i8::MIN.unsigned_abs() as u128,true));
        assert_eq!(expected_space_i16, required_space(base,i16::MIN.unsigned_abs() as u128,true));
        assert_eq!(expected_space_i32, required_space(base,i32::MIN.unsigned_abs() as u128,true));
        assert_eq!(expected_space_i64, required_space(base,i64::MIN.unsigned_abs() as u128,true));
        assert_eq!(expected_space_i128, required_space(base,i128::MIN.unsigned_abs() as u128,true));
    }

    
    #[test]
    fn sanity_check_required_size() {

        // test zero
        assert_eq!(1, required_space(2,0,false));
        // test positive one
        assert_eq!(1, required_space(2,1,false));
        // test negative one
        assert_eq!(2, required_space(2,1,true));

        verify_required_sizes_for_base(2,
        8,
        16,
        32,
        64,
        128,
        9,
        17,
        33,
        65,
        129
        );

        verify_required_sizes_for_base(8,
        3,  // 377
        6,  // 177777
        11, // 37777777777
        22, // 1777777777777777777777
        43, // 3777777777777777777777777777777777777777777
        4,  // -200
        7,  // -100000
        12, // -20000000000
        23, // -1000000000000000000000
        44,  // -2000000000000000000000000000000000000000000
        );

        verify_required_sizes_for_base(10,
        3,  // 255
        5,  // 65535
        10, // 4294967295
        20, // 18446744073709551615
        39, // 340282366920938463463374607431768211455
        4,  // -128
        6,  // -32768
        11, // -2147483648
        20, // -9223372036854775808
        40,  // -170141183460469231731687303715884105728
        );

        verify_required_sizes_for_base(16,
        2,
        4,
        8,
        16,
        32,
        3,
        5,
        9,
        17,
        33,
        );

    }

    #[test]
    fn sanity() {
        assert_eq!(b"256123", numtoa_i32(256123_i32, 10, &mut [0u8; 20]));
    }

    #[test]
    #[should_panic]
    fn base_too_low() {
        numtoa_i32(50, 1, &mut [0u8; 100]);
    }

    #[test]
    #[should_panic]
    fn base_too_high() {
        numtoa_i32(36, 37, &mut [0u8; 100]);
    }

    #[test]
    fn str_convenience_core() {
        assert_eq!("256123", numtoa_i32_str(256123_i32, 10, &mut [0u8; 20]));
    }

    #[test]
    fn str_convenience_core_u8() {
        assert_eq!("42", numtoa_u8_str(42u8, 10, &mut [b'X'; 20]));
    }

    #[test]
    fn str_convenience_core_i8() {
        assert_eq!("42", numtoa_i8_str(42i8, 10, &mut [b'X'; 20]));
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn base10_u8_array_too_small_core() {
        let _ = numtoa_u8(0_u8, 10, &mut [0u8; 2]);
    }

    #[test]
    fn base10_u8_array_just_right_core() {
        let _ = numtoa_u8(0, 10, &mut [0u8; 3]);
    }

    #[test]
    fn base16_i8_all_core() {
        for i in i8::MIN..i8::MAX {
            let _ = numtoa_i8(i, 16, &mut [0u8; 3]);
        }
    }

    #[test]
    fn base10_u8_all_core() {
        for i in u8::MIN..u8::MAX {
            let _ = numtoa_u8(i, 10, &mut [0u8; 3]);
        }
    }

    #[test]
    fn base16_u8_all_core() {
        for i in u8::MIN..u8::MAX {
            let _ = numtoa_u8(i, 16, &mut [0u8; 3]);
        }
    }
}
