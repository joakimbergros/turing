use crate::gates::and;
use crate::gates::not;

/// Returns true if both are not true
///
/// # Truth table
///
/// | left | right | result |
/// | - | - | - |
/// | 0 | 0 | 1 |
/// | 1 | 0 | 1 |
/// | 0 | 1 | 1 |
/// | 1 | 1 | 0 |
///
/// # Examples
///
/// ```
/// use turing::gates::nand;
///
/// let should_be_true = nand(true, false);
///
/// assert_eq!(should_be_true, true);
/// ```
///
/// ```
/// use turing::gates::nand;
///
/// let should_be_false = nand(true, true);
///
/// assert_eq!(should_be_false, false);
/// ```
pub fn nand(left: bool, right: bool) -> bool {
    not(and(left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nand_true_when_all_are_false() {
        assert_eq!(nand(false, false), true);
    }

    #[test]
    fn nand_false_when_all_are_true() {
        assert_eq!(nand(true, true), false);
    }
}
