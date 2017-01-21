# NumToA

## `#![no_std]` Compatible with Zero Heap Allocations

The standard library provides a convenient method of converting numbers into strings, but these strings are
heap-allocated. If you have an application which needs to convert large volumes of numbers into strings, but don't
want to pay the price of heap allocation, this crate provides an efficient `no_std`-compatible method of heaplessly converting numbers
into their string representations, storing the representation within a reusable byte array.

## Supports Multiple Bases

In addition to supporting the standard base 10 conversion, this implementation allows you to select the base of
your choice. Therefore, if you want a binary representation, set the base to 2. If you want hexadecimal, set the
base to 16.

## No Unsafe

Both the standard library and itoa crate rely on unsafe functions, but this implementation has been able to avoid
the use of unsafe entirely.

## Fast

Performance is roughly 8% better than the `itoa` crate when performing base 10 conversions.
Below is a benchmark of printing 0 through 100,000,000 to `/dev/null`

```
numtoa: 14592 ms
itoa:   16020 ms
std:    22252 ms
```

## How Base 10 Conversion Works

For all integers above the 8-bit size (`u8`/`i8`), integers will be repeatedly divided by `10_000` until they are
no longer divisible by `10_000`. Before dividing each successive number by `10_000`, the remainder of each number
is calculated by getting the modulo of `10_000` with the modulus operator.

> If the number is equal to 1532626

> number % 10_000 == 2626

> number / 10_000 == 153

To convert these values into their character representations, this is when a lookup table comes into play. There are
four possible lookup tables for base 10 conversions: single-digit, double-digit, triple-digit, and quad-digit. Each
remainder of a division by `10_000` will use a quad-digit lookup table, which converts the value into the index
where that value is located at.

> 2626 value * 4 digits = 10504 index location

Once we have the starting index location, we can perform a `memcpy` using Rust's safe `copy_from_slice` method,
writing into our provided byte array in reverse.

```rust
string[index-3..index+1].copy_from_slice(&DEC_QUAD_LOOKUP[number..number+4])
```
Once the value of the number is less than `10_000`, we get the last N digits by using either the single-digit,
double-digit, or triple-digit lookup by comparing the value of the final number. If the number is greater than 99,
use the triple-digit lookup; else if it is greater than 9, use the double-digit lookup; and finally use a single-digit
lookup.

By the time the process is finished, we will have the value indicating where in the array that the text-representation
starts. This number is then used in the creation of a slice of the underlying array.

```rust
let index = 125235.numtoa(10, &mut buffer);
let _ = stdout.write(buffer[index..]);
```

## Base 10 Example

```rust
use numtoa::NumToA;
use std::io::{self, Write};

let stdout = io::stdout();
let mut stdout = stdout.lock();
let mut buffer = [0u8; 20];

let number: u32 = 162392;
let mut start_index = number.numtoa(10, &mut buffer);
let _ = stdout.write(&buffer[start_index..]);
let _ = stdout.write(b"\n");
assert_eq!(&buffer[start_index..], b"162392");

let other_number: i32 = -6235;
start_index = other_number.numtoa(10, &mut buffer);
let _ = stdout.write(&buffer[start_index..]);
let _ = stdout.write(b"\n");
assert_eq!(&buffer[start_index..], b"-6235");

let large_num: u64 = 35320842;
start_index = large_num.numtoa(10, &mut buffer);
let _ = stdout.write(&buffer[start_index..]);
let _ = stdout.write(b"\n");
assert_eq!(&buffer[start_index..], b"35320842");

let max_u64: u64 = 18446744073709551615;
start_index = max_u64.numtoa(10, &mut buffer);
let _ = stdout.write(&buffer[start_index..]);
let _ = stdout.write(b"\n");
assert_eq!(&buffer[start_index..], b"18446744073709551615");
```
