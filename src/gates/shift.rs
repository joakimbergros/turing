use crate::Byte;

pub fn shift_left(mut byte: Byte) -> Result<Byte, &'static str> {
    let mut new_value = false;

    for i in (0..Byte::MAX).rev() {
        let value_to_shift = byte.0[i];
        byte.0[i] = new_value;
        new_value = value_to_shift;
    }

    if new_value == true {
        return Err("Overflow");
    }

    Ok(byte)
}

pub fn shift_right(mut byte: Byte) -> Result<Byte, &'static str> {
    let mut new_value = false;

    for i in 0..Byte::MAX {
        let value_to_shift = byte.0[i];
        byte.0[i] = new_value;
        new_value = value_to_shift;
    }

    Ok(byte)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_one_left_is_two() {
        let byte = Byte([false, false, false, false, false, false, false, true]);

        assert_eq!(
            shift_left(byte).unwrap(),
            Byte([false, false, false, false, false, false, true, false])
        )
    }

    #[test]
    fn shift_left_overflows() {
        let byte = Byte([true, false, false, false, false, false, false, true]);

        assert!(shift_left(byte).is_err())
    }

    #[test]
    fn shift_one_right_is_zero() {
        let byte = Byte([false, false, false, false, false, false, false, true]);

        assert_eq!(
            shift_right(byte).unwrap(),
            Byte([false, false, false, false, false, false, false, false])
        )
    }
}
