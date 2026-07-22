# bitfield-rs

A Rust bitfield manipulation library. Zero-cost abstractions for bitwise operations, register mapping, and hardware-level data structures.

## Features

- Zero-cost abstractions
- Const generics for bit widths
- Compile-time validation
- Hardware register mapping support

## Usage

```rust
use bitfield_rs::BitField;

let mut reg = BitField::<u32>::new();
reg.set_bits(0, 8, 0xFF);
assert_eq!(reg.get_bits(0, 8), 0xFF);
```

## Build

```bash
cargo build
cargo test
```

## License

MIT
