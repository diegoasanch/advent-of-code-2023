use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchematicParserError {
    #[error("Invalid number {0}")]
    InvalidNumber(String),
}

pub struct Schematic {
    pub part_numbers: Vec<u32>,
    pub non_part_numbers: Vec<u32>,
}

impl Schematic {
    pub fn parse(lines: &Vec<String>) -> Result<Self, SchematicParserError> {
        let mut part_numbers = Vec::new();
        let mut non_part_numbers = Vec::new();
        let mut current_num = String::new();
        let mut current_is_part = false;

        for (y, line) in lines.iter().enumerate() {
            for (x, character) in line.chars().enumerate() {
                if character.is_digit(10) {
                    current_num.push(character);
                    if !current_is_part {
                        current_is_part = is_part(x, y, lines);
                    }
                } else {
                    if !current_num.is_empty() {
                        if current_is_part {
                            part_numbers.push(current_num.clone());
                        } else {
                            non_part_numbers.push(current_num.clone());
                        }
                        current_num.clear();
                        current_is_part = false;
                    }
                }
            }
            if !current_num.is_empty() {
                if current_is_part {
                    part_numbers.push(current_num.clone());
                } else {
                    non_part_numbers.push(current_num.clone());
                }
                current_num.clear();
                current_is_part = false;
            }
        }

        let part_numbers = parse_numbers(&part_numbers)?;
        let non_part_numbers = parse_numbers(&non_part_numbers)?;

        Ok(Self {
            part_numbers,
            non_part_numbers,
        })
    }
}

fn parse_numbers(string_nums: &Vec<String>) -> Result<Vec<u32>, SchematicParserError> {
    Ok(string_nums
        .iter()
        .map(|num| {
            num.parse::<u32>()
                .map_err(|_| SchematicParserError::InvalidNumber(num.clone()))
        })
        .collect::<Result<Vec<u32>, SchematicParserError>>())?
}

#[derive(PartialEq, Eq)]
enum CheckPosition {
    Prev,
    Current,
    Next,
}

fn is_part(x: usize, y: usize, lines: &Vec<String>) -> bool {
    let mut part = false;
    let positions = vec![
        CheckPosition::Prev,
        CheckPosition::Current,
        CheckPosition::Next,
    ];

    'outer: for i in positions.iter() {
        for j in positions.iter() {
            if *i == CheckPosition::Current && *j == CheckPosition::Current {
                continue; // skip the center
            }

            if let (Some(x), Some(y)) = (get_check_position(x, i), get_check_position(y, j)) {
                if let Some(line) = lines.get(y) {
                    if let Some(character) = line.chars().nth(x) {
                        if !character.is_digit(10) && character != '.' {
                            part = true;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    part
}

fn get_check_position(position: usize, check_position: &CheckPosition) -> Option<usize> {
    match check_position {
        CheckPosition::Prev => position.checked_sub(1),
        CheckPosition::Current => Some(position),
        CheckPosition::Next => position.checked_add(1),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_part() {
        let lines = vec!["467..114..".to_string(), "...*......".to_string()];
        assert_eq!(is_part(0, 0, &lines), false);
        assert_eq!(is_part(2, 0, &lines), true);
        assert_eq!(is_part(5, 0, &lines), false);
    }

    #[test]
    fn parses_schematic() {
        let lines = vec!["467..114..".to_string(), "...*......".to_string()];
        let result = Schematic::parse(&lines).unwrap();
        assert_eq!(result.part_numbers, vec![467]);
        assert_eq!(result.non_part_numbers, vec![114]);
    }

    /// Test from the website's problem description
    #[test]
    fn parses_full_schematic() {
        let lines = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        let result = Schematic::parse(&lines).unwrap();
        assert_eq!(
            result.part_numbers,
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        );
        assert_eq!(result.non_part_numbers, vec![114, 58]);
    }
}
