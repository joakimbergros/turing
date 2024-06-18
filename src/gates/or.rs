/// Returns true if any input is also true
///
/// # Truth table
///
/// | left | right | result |
/// | ---- | ----- | ------ |
/// | 0    | 0     | 0      |
/// | 1    | 0     | 1      |
/// | 0    | 1     | 1      |
/// | 1    | 1     | 1      |
///
/// # Examples
///
/// ```
/// use turing::gates::or;
///
/// let should_be_true = or(true, false);
/// let should_be_false = or(false, false);
///
/// assert_eq!(should_be_true, true);
/// assert_eq!(should_be_false, false);
/// ```
pub fn or(left: bool, right: bool) -> bool {
    left || right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn or_true_when_at_least_one_is_true() {
        assert_eq!(or(true, false), true);
        assert_eq!(or(false, true), true);
    }

    #[test]
    fn or_true_when_all_are_true() {
        assert_eq!(or(true, true), true);
    }

    #[test]
    fn or_false_when_all_are_false() {
        assert_eq!(or(false, false), false);
    }
}
