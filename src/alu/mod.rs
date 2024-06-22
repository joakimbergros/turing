use std::u8;

use crate::gates::and;
use crate::gates::or;
use crate::gates::xor;
use crate::Byte;

pub fn add_bytes(left: Byte, right: Byte) -> Result<Byte, &'static str> {
    let mut carry = false;
    let mut byte = Byte::default();

    for i in (0..Byte::MAX).rev() {
        let result = add_bits(left.0[i].clone(), right.0[i].clone(), carry);

        dbg!(i, left.0[i], right.0[i], carry, result);

        carry = result.1;
        byte.0[i] = result.0;
    }

    if carry {
        return Err("Overflow");
    }

    Ok(byte)
}

pub fn add_bits(left: bool, right: bool, carry: bool) -> (bool, bool) {
    (
        xor(xor(left, right), carry),
        or(or(and(left, right), and(left, carry)), and(right, carry)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_bytes_overflows() {
        let mut left = Byte::default();
        let mut right = Byte::default();

        left.0[0] = true;
        right.0[0] = true;

        let result = add_bytes(left, right);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn can_add_5_plus_14() {
        let five = Byte([false, false, false, false, false, true, false, true]);
        let fourteen = Byte([false, false, false, false, true, true, true, false]);

        let result = add_bytes(five, fourteen);

        dbg!(&result);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Byte([false, false, false, true, false, false, true, true])
        )
    }

    #[test]
    fn add_gives_false_when_all_are_false() {
        assert_eq!(add_bits(false, false, false), (false, false));
    }

    #[test]
    fn add_gives_true_when_at_least_one_is_true() {
        assert_eq!(add_bits(true, false, false), (true, false));
    }

    #[test]
    fn add_gives_carry_when_majority_are_true() {
        assert_eq!(add_bits(true, true, false), (false, true));
    }

    #[test]
    fn add_one_plus_one_should_be_two() {
        let left = true;
        let right = true;

        let first_result = add_bits(left, right, false);

        assert_eq!(first_result, (false, true));

        let second_result = add_bits(false, false, first_result.1);

        assert_eq!(second_result, (true, false));

        assert_eq!((first_result.0, second_result.0), (false, true))
    }

    #[test]
    fn add_one_plus_two_should_be_three() {
        let left = (false, true); // 01
        let right = (true, false); // 10

        let first_result = add_bits(left.0, right.0, false);

        assert_eq!(first_result, (true, false));

        let second_result = add_bits(left.1, right.1, first_result.1);

        assert_eq!(second_result, (true, false));

        assert_eq!((first_result.0, second_result.0), (true, true))
    }
}
