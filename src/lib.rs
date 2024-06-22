use std::ops::{Add, Not};

use alu::add_bytes;

pub mod alu;
pub mod gates;
pub mod hardware;
pub mod memory;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Byte([bool; Self::MAX]);

impl Byte {
    pub const MAX: usize = 8;
    pub const INT_MAP: [u8; Self::MAX] = [
        0b1000_0000,
        0b0100_0000,
        0b0010_0000,
        0b0001_0000,
        0b0000_1000,
        0b0000_0100,
        0b0000_0010,
        0b0000_0001,
    ];
}

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Self([
            value & Self::INT_MAP[0] > 0,
            value & Self::INT_MAP[1] > 0,
            value & Self::INT_MAP[2] > 0,
            value & Self::INT_MAP[3] > 0,
            value & Self::INT_MAP[4] > 0,
            value & Self::INT_MAP[5] > 0,
            value & Self::INT_MAP[6] > 0,
            value & Self::INT_MAP[7] > 0,
        ])
    }
}

impl Into<u8> for Byte {
    fn into(self) -> u8 {
        let mut int: u8 = 0;

        for i in 0..Self::MAX {
            if !self.0[i] {
                continue;
            }
            int |= Self::INT_MAP[i];
        }

        int
    }
}

impl Add for Byte {
    type Output = Result<Self, &'static str>;

    fn add(self, rhs: Self) -> Self::Output {
        add_bytes(self, rhs)
    }
}

#[derive(Debug, PartialEq)]
pub struct Bit(bool);

impl Not for Bit {
    type Output = bool;

    fn not(self) -> Self::Output {
        self.0
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        value.0
    }
}

impl PartialEq<bool> for Bit {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_creates() {
        let byte = Byte([false, false, false, false, false, false, false, false]);

        assert_eq!(byte.0.len(), 8);
        assert_eq!(byte.0.first(), Some(&false));
    }

    #[test]
    fn byte_from_u8() {
        assert_eq!(
            Byte::from(15),
            Byte([false, false, false, false, true, true, true, true])
        );
    }

    #[test]
    fn byte_to_u8() {
        let into: u8 = Byte::from(15).into();
        assert_eq!(into, 15);
    }

    #[test]
    fn add_bytes() {
        let left = Byte::from(8);
        let right = Byte::from(124);

        let result = left + right;

        assert_eq!(result.unwrap(), Byte::from(132));
    }

    #[test]
    fn bit_is_true() {
        let bit = Bit(true);

        assert_eq!(bit, true);
    }

    #[test]
    fn bit_is_false() {
        let bit = Bit(false);

        assert_eq!(bit, false);
    }
}
