//! The standard library provides a convenient method of converting numbers into strings, but these strings are
//! heap-allocated. If you have an application which needs to convert large volumes of numbers into strings, but don't
//! want to pay the price of heap allocation, this crate provides an efficient `no_std`-compatible method of heaplessly converting numbers
//! into their string representations, storing the representation within a reusable byte array.
//!
//! In addition to supporting the standard base 10 conversion, this implementation allows you to select the base of
//! your choice. Therefore, if you want a binary representation, set the base to 2. If you want hexadecimal, set the
//! base to 16.
//!
//! # `NumToA` Trait Example
//!
//! ```
//! use numtoa::NumToA;
//!
//! let mut buffer = [0u8; 20];
//! assert_eq!(162392_u32.numtoa_str(10, &mut buffer), "162392");
//! assert_eq!((-6235_i32).numtoa(10, &mut buffer), b"-6235");
//! assert_eq!(i8::MIN.numtoa(10, &mut buffer), b"-128");
//! ```
//!
//! The same buffer can be reused across many conversions instead of allocating one per number:
//!
//! ```
//! use numtoa::NumToA;
//!
//! let mut buffer = [0u8; 20];
//! let mut joined = String::new();
//! for number in 1..=3 {
//!     joined.push_str(number.numtoa_str(10, &mut buffer));
//!     joined.push(',');
//! }
//! assert_eq!(joined, "1,2,3,");
//! ```
//!
//! ## Const Context Example
//!
//! The [`BaseN`] API (selecting the base via a const generic) is fully `const fn`, so conversions
//! can run entirely at compile time — as shown below, `HEX` is computed by the compiler, not at
//! runtime. It returns an [`AsciiNumber`], whose buffer is sized automatically to the minimum
//! space required for the given type and base. [`AsciiNumber`] implements `Display` and
//! `Deref<Target = str>`, so it can be used directly wherever a string is expected:
//!
//! ```
//! use numtoa::{AsciiNumber, BaseN};
//!
//! const HEX: AsciiNumber<{ BaseN::<16>::REQUIRED_SPACE_U32 }> = BaseN::<16>::u32(48879);
//! assert_eq!(format!("0x{HEX}"), "0xBEEF"); // `Display`
//! assert_eq!(HEX.len(), 4); // `Deref<Target = str>`
//! ```
//!
//! `_padded` produces a fixed-width `AsciiNumber<LENGTH>`, so `LENGTH` must already be large
//! enough to hold the type's largest possible value. `_filled` instead pads up to *at least*
//! `LENGTH` digits but grows past it for larger numbers rather than truncating:
//!
//! ```
//! use numtoa::BaseN;
//!
//! assert_eq!(BaseN::<10>::i32_padded::<11>(42, b'0').as_str(), "00000000042");
//!
//! assert_eq!(BaseN::<10>::i32_filled::<5>(42, b'0').as_str(), "00042");
//! assert_eq!(BaseN::<10>::i32_filled::<5>(123456, b'0').as_str(), "123456");
//! ```

#![no_std]

mod numtoa_core;
pub use numtoa_core::*;

mod numtoa_trait;
pub use numtoa_trait::*;

mod numtoa_const;
pub use numtoa_const::*;
