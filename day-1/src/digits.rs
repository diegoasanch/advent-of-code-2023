/// Returns the first numeric digit of a string (if present). Otherwise, returns None.
pub fn get_first_digit(text: &str) -> Option<char> {
    let mut digit = None;
    for c in text.chars() {
        if c.is_digit(10) {
            digit = Some(c);
            break;
        }
    }
    digit
}

/// Returns the last numeric digit of a string (if present). Otherwise, returns None.
pub fn get_last_digit(text: &str) -> Option<char> {
    get_first_digit(text.chars().rev().collect::<String>().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_digit() {
        assert_eq!(get_first_digit("123"), Some('1'));
        assert_eq!(get_first_digit("abc"), None);
        assert_eq!(get_first_digit("ab1ds2"), Some('1'));
        assert_eq!(get_first_digit("abds1sdasda"), Some('1'));
    }

    #[test]
    fn test_get_last_digit() {
        assert_eq!(get_last_digit("123"), Some('3'));
        assert_eq!(get_last_digit("abc"), None);
        assert_eq!(get_last_digit("ab1ds2"), Some('2'));
        assert_eq!(get_last_digit("abds1sdasda"), Some('1'));
    }
}
