use super::{nand, not, or};

/// Returns true if only one input is true
///
/// # Truth table
///
/// | left | right | result |
/// | - | - | - |
/// | 0 | 0 | 0 |
/// | 1 | 0 | 1 |
/// | 0 | 1 | 1 |
/// | 1 | 1 | 0 |
///
/// # Examples
///
/// ```
/// use turing::gates::xor;
///
/// let should_be_true = xor(true, false);
///
/// assert_eq!(should_be_true, true);
/// ```
///
/// ```
/// use turing::gates::xor;
///
/// let should_be_false = xor(true, true);
///
/// assert_eq!(should_be_false, false);
/// ```
pub fn xor(left: bool, right: bool) -> bool {
    nand(or(left, not(right)), or(right, not(left)))

    // left != right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_false_when_both_are_same() {
        assert_eq!(xor(true, true), false);
        assert_eq!(xor(false, false), false);
    }

    #[test]
    fn xor_true_when_only_one_are_true() {
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(true, false), true);
    }
}
