use thiserror::Error;

pub fn get_winning_numbers(card_data: &str) -> Result<Vec<u32>, ScratchCardParseError> {
    let content = card_data.split(":").collect::<Vec<&str>>();
    let card_numbers_data = content
        .get(1)
        .ok_or_else(|| ScratchCardParseError::InvalidCardData(card_data.to_string()))?
        .trim();
    let winning_numbers_data = card_numbers_data
        .split("|")
        .next()
        .ok_or_else(|| ScratchCardParseError::InvalidCardData(card_data.to_string()))?;

    let winning_numbers = winning_numbers_data
        .trim()
        .replace("  ", " ")
        .split(" ")
        .map(|num| {
            num.parse::<u32>()
                .map_err(|_| ScratchCardParseError::InvalidNumber(num.to_string()))
        })
        .collect::<Result<Vec<u32>, ScratchCardParseError>>()?;
    Ok(winning_numbers)
}

pub fn get_card_numbers(card_data: &str) -> Result<Vec<u32>, ScratchCardParseError> {
    let content = card_data.split(":").collect::<Vec<&str>>();
    let card_numbers_data = content
        .get(1)
        .ok_or_else(|| ScratchCardParseError::InvalidCardData(card_data.to_string()))?
        .trim()
        .split("|")
        .collect::<Vec<&str>>();
    let card_numbers_data = card_numbers_data
        .get(1)
        .ok_or_else(|| ScratchCardParseError::InvalidCardData(card_data.to_string()))?;
    let card_numbers = card_numbers_data
        .trim()
        .replace("  ", " ")
        .split(" ")
        .map(|num| {
            num.parse::<u32>()
                .map_err(|_| ScratchCardParseError::InvalidNumber(num.to_string()))
        })
        .collect::<Result<Vec<u32>, ScratchCardParseError>>()?;
    Ok(card_numbers)
}

pub fn find_common_numbers(nums_1: &Vec<u32>, nums_2: &Vec<u32>) -> Vec<u32> {
    nums_1
        .iter()
        .filter(|num| nums_2.contains(num))
        .copied()
        .collect()
}

pub fn array_score(nums: &Vec<u32>) -> u32 {
    let len = nums.len();
    if len > 0 {
        2u32.pow((len as u32) - 1)
    } else {
        0
    }
}

#[derive(Error, Debug)]
pub enum ScratchCardParseError {
    #[error("Invalid number {0}")]
    InvalidNumber(String),

    #[error("Invalid card data {0}")]
    InvalidCardData(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_winning_numbers() {
        let card_data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let winning_numbers = get_winning_numbers(card_data).unwrap();
        assert_eq!(winning_numbers, vec![41, 48, 83, 86, 17]);
    }

    #[test]
    fn test_get_card_numbers() {
        let card_data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card_numbers = get_card_numbers(card_data).unwrap();
        assert_eq!(card_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_find_common_numbers() {
        let nums_1 = vec![83, 86, 6, 31, 17, 9, 48, 53];
        let nums_2 = vec![41, 48, 83, 86, 17];
        let common_numbers = find_common_numbers(&nums_1, &nums_2);
        assert_eq!(common_numbers, vec![83, 86, 17, 48]);
    }
}
