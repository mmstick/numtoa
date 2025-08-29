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

mod numtoa_core;
#[cfg(feature = "api-core")]
pub use numtoa_core::*;

mod numtoa_trait;
#[cfg(feature = "api-trait")]
pub use numtoa_trait::*;

mod numtoa_const;
#[cfg(feature = "api-const")]
pub use numtoa_const::*;
