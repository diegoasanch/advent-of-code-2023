use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchematicParserError {
    #[error("Invalid number {0}")]
    InvalidNumber(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub character: char,
    pub pos_x: usize,
    pub pos_y: usize,
}

pub struct Num {
    pub value: u32,
    pub adjacent_symbol: Option<Symbol>,
}

impl Num {
    pub fn new(value: u32, adjacent_symbol: Option<Symbol>) -> Self {
        Self {
            value,
            adjacent_symbol,
        }
    }
}

pub struct Schematic {
    pub parts: Vec<Num>,
}

impl Schematic {
    pub fn parse(lines: &Vec<String>) -> Result<Self, SchematicParserError> {
        let mut current_num = String::new();
        let mut parts = Vec::new();
        let mut adjacent_symbol = None;

        for (y, line) in lines.iter().enumerate() {
            for (x, character) in line.chars().enumerate() {
                if character.is_digit(10) {
                    current_num.push(character);
                    if adjacent_symbol.is_none() {
                        adjacent_symbol = get_adjacent_symbol(x, y, lines);
                    }
                } else {
                    if !current_num.is_empty() {
                        let num = current_num.parse::<u32>().map_err(|_| {
                            SchematicParserError::InvalidNumber(current_num.to_string())
                        })?;
                        parts.push(Num::new(num, adjacent_symbol));
                        current_num.clear();
                        adjacent_symbol = None;
                    }
                }
            }
            if !current_num.is_empty() {
                let num = current_num
                    .parse::<u32>()
                    .map_err(|_| SchematicParserError::InvalidNumber(current_num.to_string()))?;
                parts.push(Num::new(num, adjacent_symbol));
                current_num.clear();
                adjacent_symbol = None;
            }
        }

        Ok(Self { parts })
    }

    pub fn get_part_numbers(&self) -> Vec<u32> {
        self.parts
            .iter()
            .filter(|num| num.adjacent_symbol.is_some())
            .map(|num| num.value)
            .collect()
    }

    pub fn get_non_part_numbers(&self) -> Vec<u32> {
        self.parts
            .iter()
            .filter(|num| num.adjacent_symbol.is_none())
            .map(|num| num.value)
            .collect()
    }
}

#[derive(PartialEq, Eq)]
enum CheckPosition {
    Prev,
    Current,
    Next,
}

fn get_adjacent_symbol(x: usize, y: usize, lines: &Vec<String>) -> Option<Symbol> {
    let mut symbol = None;
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
                            symbol = Some(Symbol {
                                character,
                                pos_x: x,
                                pos_y: y,
                            });
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    symbol
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
        assert_eq!(get_adjacent_symbol(0, 0, &lines), None);
        assert_eq!(
            get_adjacent_symbol(2, 0, &lines),
            Some(Symbol {
                character: '*',
                pos_x: 3,
                pos_y: 1
            })
        );
        assert_eq!(get_adjacent_symbol(5, 0, &lines), None);
    }

    #[test]
    fn parses_schematic() {
        let lines = vec!["467..114..".to_string(), "...*......".to_string()];
        let result = Schematic::parse(&lines).unwrap();
        assert_eq!(result.get_part_numbers(), vec![467]);
        assert_eq!(result.get_non_part_numbers(), vec![114]);
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
            result.get_part_numbers(),
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        );
        assert_eq!(result.get_non_part_numbers(), vec![114, 58]);
    }
}
