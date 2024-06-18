use crate::gates::not;
use crate::gates::or;

/// Returns rue if any input is also true
///
/// # Truth table
///
/// | left | right | result |
/// | - | - | - |
/// | 0 | 0 | 1 |
/// | 1 | 0 | 0 |
/// | 0 | 1 | 0 |
/// | 1 | 1 | 0 |
///
/// # Examples
///
/// ```
/// use turing::gates::nor;
///
/// let should_be_true = nor(false, false);
///
/// assert_eq!(should_be_true, true);
/// ```
///
/// ```
/// use turing::gates::nor;
///
/// let should_be_false = nor(true, false);
///
/// assert_eq!(should_be_false, false);
/// ```
pub fn nor(left: bool, right: bool) -> bool {
    not(or(left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nor_true_when_all_are_false() {
        assert_eq!(nor(false, false), true);
    }

    #[test]
    fn nor_false_when_any_is_true() {
        assert_eq!(nor(false, true), false);
    }
}
