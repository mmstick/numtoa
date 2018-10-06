# 0.2.0

- Increased ergonomics of the `numtoa` method to return a `&[u8]` instead of a `usize`.
- Removed the restriction on only accepting `&[u8; 20]` for `numtoa_str`.

# 0.1.0

- Added the `std` feature to add the `numtoa_str` method.
- `numtoa_str` returns a `&str` rather than a `usize`.
