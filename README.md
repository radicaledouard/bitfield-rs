# bitfield-rs

> A Rust bitfield manipulation library

## Overview

High-performance, zero-overhead abstractions for bit-level data manipulation.

## Features

- Zero-cost abstractions
- Const generics for bit widths
- Compile-time validation
- Hardware register mapping support

## Usage

```rust
use bitfield_rs::BitField;

let mut reg = BitField::<u32>::new();
reg.set_bits(0..8, 0xFF);
assert_eq!(reg.get_bits(0..8), 0xFF);
```

## License

MIT

## Co-authored with [@SamyAlderson](https://github.com/SamyAlderson)
