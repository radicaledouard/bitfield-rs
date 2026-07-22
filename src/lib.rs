//! # bitfield-rs
//!
//! Zero-cost abstractions for bit-level data manipulation.
//! Perfect for hardware register mapping, protocol parsing,
//! and any scenario where you need fine-grained control over bits.

#![no_std]

use core::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitField<T: Copy + Default> {
    value: T,
}

impl<T: Copy + Default> BitField<T> {
    #[inline]
    pub fn new() -> Self {
        Self { value: T::default() }
    }

    #[inline]
    pub fn from_raw(value: T) -> Self {
        Self { value }
    }

    #[inline]
    pub fn raw(&self) -> T {
        self.value
    }
}

macro_rules! impl_bitfield {
    ($t:ty) => {
        impl BitField<$t> {
            #[inline]
            pub fn get_bits(&self, start: u32, end: u32) -> $t {
                let width = end - start;
                let mask = (1 as $t).wrapping_shl(width).wrapping_sub(1);
                (self.value >> start) & mask
            }

            #[inline]
            pub fn set_bits(&mut self, start: u32, end: u32, val: $t) {
                let width = end - start;
                let mask = (1 as $t).wrapping_shl(width).wrapping_sub(1);
                self.value &= !(mask << start);
                self.value |= (val & mask) << start;
            }

            #[inline]
            pub fn test_bit(&self, bit: u32) -> bool {
                (self.value >> bit) & 1 == 1
            }

            #[inline]
            pub fn set_bit(&mut self, bit: u32) {
                self.value |= 1 << bit;
            }

            #[inline]
            pub fn clear_bit(&mut self, bit: u32) {
                self.value &= !(1 << bit);
            }

            #[inline]
            pub fn toggle_bit(&mut self, bit: u32) {
                self.value ^= 1 << bit;
            }

            #[inline]
            pub fn popcount(&self) -> u32 {
                self.value.count_ones()
            }
        }
    };
}

impl_bitfield!(u8);
impl_bitfield!(u16);
impl_bitfield!(u32);
impl_bitfield!(u64);
impl_bitfield!(u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_set_get() {
        let mut bf = BitField::<u32>::new();
        bf.set_bits(0, 8, 0xAB);
        assert_eq!(bf.get_bits(0, 8), 0xAB);
    }

    #[test]
    fn test_bit_operations() {
        let mut bf = BitField::<u32>::new();
        bf.set_bit(0);
        assert!(bf.test_bit(0));
        bf.toggle_bit(0);
        assert!(!bf.test_bit(0));
    }

    #[test]
    fn test_popcount() {
        let bf = BitField::<u32>::from_raw(0b1010_1010);
        assert_eq!(bf.popcount(), 4);
    }

    #[test]
    fn test_multiple_ranges() {
        let mut bf = BitField::<u64>::new();
        bf.set_bits(0, 8, 0xFF);
        bf.set_bits(8, 16, 0x00);
        bf.set_bits(16, 24, 0xAA);
        assert_eq!(bf.get_bits(0, 8), 0xFF);
        assert_eq!(bf.get_bits(8, 16), 0x00);
        assert_eq!(bf.get_bits(16, 24), 0xAA);
    }
}
