use std::ops::Not;

pub mod alu;
pub mod gates;
pub mod hardware;
pub mod memory;

#[derive(Debug, Default)]
pub struct Byte([bool; 8]);

impl Byte {
    pub const MAX: usize = 8;
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
