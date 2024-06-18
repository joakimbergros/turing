/// Inverts the input
///
/// # Truth table
///
/// | input | result |
/// | - | - |
/// | 0 | 1 |
/// | 1 | 0 |
///
/// # Examples
///
/// ```
/// use turing::gates::not;
///
/// let should_be_true = not(false);
///
/// assert_eq!(should_be_true, true);
/// ```
///
/// ```
/// use turing::gates::not;
///
/// let should_be_false = not(true);
///
/// assert_eq!(should_be_false, false);
/// ```
pub fn not(input: bool) -> bool {
    !input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_true_when_input_is_false() {
        assert_eq!(not(false), true);
    }

    #[test]
    fn not_false_when_input_is_true() {
        assert_eq!(not(true), false);
    }
}
