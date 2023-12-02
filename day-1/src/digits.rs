/// Returns the first numeric digit of a string (if present). Otherwise, returns None.
pub fn get_first_digit(text: &str) -> Option<u8> {
    let mut digit = None;
    for c in text.chars() {
        if c.is_digit(10) {
            if let Some(d) = c.to_digit(10) {
                digit = Some(d as u8);
                break;
            }
        }
    }
    digit
}

/// Returns the last numeric digit of a string (if present). Otherwise, returns None.
pub fn get_last_digit(text: &str) -> Option<u8> {
    get_first_digit(text.chars().rev().collect::<String>().as_str())
}

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const REVERSED_NUMBERS: [&str; 10] = [
    "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

/// Returns the first number of a string (if present). Otherwise, returns None.
/// The number can be a numeric digit or the word of a number e.g. "one"
fn find_number(text: &str, reference: &[&str]) -> Option<u8> {
    let mut number = None;
    // TODO: improve this with a trie instead of a vector
    let mut candidates = Vec::<String>::new();

    'outer: for c in text.chars() {
        if c.is_digit(10) {
            candidates = Vec::new();
            if let Some(d) = c.to_digit(10) {
                number = Some(d as u8);
                break;
            }
        } else {
            candidates.push(String::new());
            for cand in candidates.iter_mut() {
                cand.push(c);
            }

            candidates.retain(|cand| any_starts_with_text(cand, reference));
            for cand in candidates.iter() {
                if let Some(n) = get_number_from_word(cand, reference) {
                    number = Some(n);
                    break 'outer;
                }
            }
        }
    }

    number
}

/// Returns true if any of the words in the reference array starts with the text.
fn any_starts_with_text(text: &str, reference: &[&str]) -> bool {
    for word in reference.iter() {
        if word.starts_with(text) {
            return true;
        }
    }
    false
}

/// Returns the first number of a string (digit or word) if present. Otherwise, returns None.
pub fn get_first_number(text: &str) -> Option<u8> {
    find_number(text, &NUMBERS)
}

/// Returns the last number of a string (digit or word) if present. Otherwise, returns None.
pub fn get_last_number(text: &str) -> Option<u8> {
    let reversed_text = text.chars().rev().collect::<String>();
    find_number(&reversed_text, &REVERSED_NUMBERS)
}

/// Returns the index of a word from the reference array if present. Otherwise, returns None.
fn get_number_from_word(text: &str, reference: &[&str]) -> Option<u8> {
    let mut number = None;
    for (index, word) in reference.iter().enumerate() {
        if word == &text {
            number = Some(index as u8);
            break;
        }
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_digit() {
        assert_eq!(get_first_digit("123"), Some(1));
        assert_eq!(get_first_digit("abc"), None);
        assert_eq!(get_first_digit("ab1ds2"), Some(1));
        assert_eq!(get_first_digit("abds1sdasda"), Some(1));
    }

    #[test]
    fn test_get_last_digit() {
        assert_eq!(get_last_digit("123"), Some(3));
        assert_eq!(get_last_digit("abc"), None);
        assert_eq!(get_last_digit("ab1ds2"), Some(2));
        assert_eq!(get_last_digit("abds1sdasda"), Some(1));
    }

    #[test]
    fn test_gets_number_from_word() {
        assert_eq!(get_first_number("one"), Some(1));
        assert_eq!(get_first_number("two"), Some(2));
        assert_eq!(get_first_number("three2"), Some(3));
        assert_eq!(get_first_number("2four"), Some(2));
        assert_eq!(get_first_number("asvsd"), None);
        assert_eq!(get_first_number("eno"), None);
    }

    #[test]
    fn test_gets_last_number_from_word() {
        assert_eq!(get_last_number("one"), Some(1));
        assert_eq!(get_last_number("two"), Some(2));
        assert_eq!(get_last_number("three2"), Some(2));
        assert_eq!(get_last_number("fourthree"), Some(3));
        assert_eq!(get_last_number("2abconeeigh"), Some(1));
    }

    #[test]
    fn test_any_starts_with() {
        assert_eq!(any_starts_with_text("one", &NUMBERS), true);
        assert_eq!(any_starts_with_text("two", &NUMBERS), true);
        assert_eq!(any_starts_with_text("three2", &NUMBERS), false);
        assert_eq!(any_starts_with_text("2four", &NUMBERS), false);
        assert_eq!(any_starts_with_text("asvsd", &NUMBERS), false);
        assert_eq!(any_starts_with_text("eig", &NUMBERS), true);
    }

    #[test]
    fn test_get_number_from_word() {
        assert_eq!(get_number_from_word("one", &NUMBERS), Some(1));
        assert_eq!(get_number_from_word("two", &NUMBERS), Some(2));
        assert_eq!(get_number_from_word("three2", &NUMBERS), None);
        assert_eq!(get_number_from_word("2four", &NUMBERS), None);
        assert_eq!(get_number_from_word("asvsd", &NUMBERS), None);
        assert_eq!(get_number_from_word("eight", &NUMBERS), Some(8));
    }

    #[test]
    fn test_get_number_from_reversed_word() {
        assert_eq!(get_number_from_word("eno", &REVERSED_NUMBERS), Some(1));
        assert_eq!(get_number_from_word("owt", &REVERSED_NUMBERS), Some(2));
        assert_eq!(get_number_from_word("eerht2", &REVERSED_NUMBERS), None);
        assert_eq!(get_number_from_word("2ruof", &REVERSED_NUMBERS), None);
        assert_eq!(get_number_from_word("asvsd", &REVERSED_NUMBERS), None);
        assert_eq!(get_number_from_word("thgie", &REVERSED_NUMBERS), Some(8));
    }
}
