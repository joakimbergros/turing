use crate::gates::not;
use crate::gates::xor;

/// Returns the negation or an xor operation
///
/// # Truth table
///
/// | left | right | result |
/// | - | - | - |
/// | 0 | 0 | 1 |
/// | 1 | 0 | 0 |
/// | 0 | 1 | 0 |
/// | 1 | 1 | 1 |
///
/// # Examples
///
/// ```
/// use turing::gates::xnor;
///
/// let should_be_true = xnor(false, false);
///
/// assert_eq!(should_be_true, true);
/// ```
///
/// ```
/// use turing::gates::xnor;
///
/// let should_be_false = xnor(true, false);
///
/// assert_eq!(should_be_false, false);
/// ```
pub fn xnor(left: bool, right: bool) -> bool {
    not(xor(left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xnor_true_when_both_inputs_are_same() {
        assert_eq!(xnor(true, true), true);
    }

    #[test]
    fn or_false_when_one_is_true() {
        assert_eq!(xnor(true, false), false);
    }
}
