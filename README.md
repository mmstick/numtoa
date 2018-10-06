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

## `&str` Example

```rust
use numtoa::NumToA;
use std::io::{self, Write};

let mut buffer = [u8; 20];
println!("{}", 12345.numtoa(10, &mut buffer));
println!("{}", 256652.numtoa(10, &mut buffer));
```

## `&[u8]` Example

```rust
use numtoa::NumToA;
use std::io::{self, Write};

let stdout = io::stdout();
let mut stdout = stdout.lock();
let mut buffer = [0u8; 20];

let number: u32 = 162392;
let _ = stdout.write(number.numtoa(10, &mut buffer));
let _ = stdout.write(b"\n");
assert_eq!(number.numtoa(10, &mut buffer), b"162392");

let number: i32 = -6235;
let _ = stdout.write(number.numtoa(10, &mut buffer));
let _ = stdout.write(b"\n");

let number: i8 = -128;
let _ = stdout.write(number.numtoa(10, &mut buffer));
let _ = stdout.write(b"\n");

let number: i8 = 53;
let _ = stdout.write(number.numtoa(10, &mut buffer));
let _ = stdout.write(b"\n");

let number: i16 = -256;
let _ = stdout.write(number.numtoa(10, &mut buffer));
let _ = stdout.write(b"\n");

let number: i16 = -32768;
let _ = stdout.write(number.numtoa(10, &mut buffer));
let _ = stdout.write(b"\n");

let number: u64 = 35320842;
let _ = stdout.write(number.numtoa(10, &mut buffer));
let _ = stdout.write(b"\n");

let number: u64 = 18446744073709551615;
let _ = stdout.write(number.numtoa(10, &mut buffer));
let _ = stdout.write(b"\n");
```