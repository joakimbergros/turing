/// Returns true if all inputs are also true.
///
/// # Truth table
///
/// | left | right | result |
/// |------|-------|--------|
/// | 0    | 0     | 0      |
/// | 1    | 0     | 0      |
/// | 0    | 1     | 0      |
/// | 1    | 1     | 1      |
///
/// # Examples
///
/// ```
/// use turing::gates::and;
///
/// let should_be_true = and(true, true);
/// assert_eq!(should_be_true, true);
///
/// let should_be_false = and(true, false);
/// assert_eq!(should_be_false, false);
///
/// let should_be_false_too = and(false, true);
/// assert_eq!(should_be_false_too, false);
///
/// let should_be_false_all = and(false, false);
/// assert_eq!(should_be_false_all, false);
/// ```
pub fn and(left: bool, right: bool) -> bool {
    left && right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and_true_when_all_are_true() {
        assert_eq!(and(true, true), true);
    }

    #[test]
    fn and_false_when_only_one_is_true() {
        assert_eq!(and(true, false), false);
        assert_eq!(and(false, true), false);
    }

    #[test]
    fn and_false_when_all_are_false() {
        assert_eq!(and(false, false), false);
    }
}

