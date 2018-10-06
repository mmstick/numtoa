# 0.2.3

- Fix the `&str` example in the readme.

# 0.2.2

- Update the readme to reflect the API change in 0.2.1

# 0.2.1

- `str` is in `core`, so the `std` feature can be removed.

# 0.2.0

- Increased ergonomics of the `numtoa` method to return a `&[u8]` instead of a `usize`.
- Removed the restriction on only accepting `&[u8; 20]` for `numtoa_str`.

# 0.1.0

- Added the `std` feature to add the `numtoa_str` method.
- `numtoa_str` returns a `&str` rather than a `usize`.
