use std::vec;

use thiserror::Error;

use crate::color::{self, Color, ColorCount};

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    rounds: Vec<ColorCount>,
}

impl Game {
    pub fn new(id: u32, rounds: Vec<ColorCount>) -> Self {
        Self { id, rounds }
    }

    pub fn parse(game_str: &str) -> Result<Self, GameParserError> {
        parse_game_str(game_str)
    }

    /// Determines if a game is valid
    /// Validation criteria:
    /// - All rounds must be valid
    pub fn is_valid(&self, available: &ColorCount) -> bool {
        self.rounds
            .iter()
            .all(|round| is_round_valid(round, available))
    }
}

/// Finds the minimum amount of colors required to pass a game
/// A color match is a color count that is less than or equal to the available count of that color
/// The matches are searched for in the RGB order on each game round
pub fn min_color_match(game: &Game) -> Option<ColorCount> {
    let mut result = ColorCount::default();
    let colors = vec![Color::Red, Color::Green, Color::Blue];

    for round in game.rounds.iter() {
        for color in colors.iter() {
            if round.gt(&result, color) {
                result.set(color, round.get(color));
            }
        }
    }
    if result.all_some() {
        return Some(result);
    }
    None
}

/// Determines if a round is valid
/// Validation criteria:
/// - The count of each color must be less than or equal to the available count of that color
fn is_round_valid(round: &ColorCount, available: &ColorCount) -> bool {
    let red = match round.red {
        Some(count) => count <= available.red.unwrap_or(0),
        None => true,
    };
    let green = match round.green {
        Some(count) => count <= available.green.unwrap_or(0),
        None => true,
    };
    let blue = match round.blue {
        Some(count) => count <= available.blue.unwrap_or(0),
        None => true,
    };
    red && green && blue
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum GameParserError {
    #[error("Invalid game record: {0}")]
    InvalidGameRecord(String),

    #[error("Invalid game id: {0}")]
    InvalidGameId(String),

    #[error("Invalid color: {0}")]
    InvalidColor(String),

    #[error("Invalid count: {0}")]
    InvalidCount(String),
}

/// Parses a game record into a game
/// Valid game records are in the format: "Game <id>: <color count>, <color count>, <color count>[; <more color counts>...]"
fn parse_game_str(game_str: &str) -> Result<Game, GameParserError> {
    let mut parts = game_str.trim().split(":");
    let id_str = parts
        .next()
        .ok_or_else(|| GameParserError::InvalidGameId(game_str.to_string()))?;
    let rounds_str = parts
        .next()
        .ok_or_else(|| GameParserError::InvalidGameRecord(game_str.to_string()))?;

    // Extract the id and color parts
    let id = id_str
        .trim()
        .strip_prefix("Game ")
        .ok_or_else(|| GameParserError::InvalidGameId(id_str.to_string()))?
        .parse::<u32>()
        .or_else(|_| Err(GameParserError::InvalidGameId(id_str.to_string())))?;

    let color_entries = rounds_str.split(";");

    let mut colors = Vec::new();
    for entry in color_entries {
        let color_counts = ColorCount::parse(entry)
            .or_else(|e| Err(GameParserError::InvalidColor(e.to_string())))?;
        colors.push(color_counts);
    }

    Ok(Game::new(id, colors))
}

#[cfg(test)]
mod test {
    use crate::color::Color;

    use super::*;

    #[test]
    fn parses_a_valid_game_string() {
        let game_str = "Game 1: 1 red, 3 blue, 2 green";
        let game = parse_game_str(game_str).unwrap();
        assert_eq!(
            game,
            Game::new(
                1,
                vec![ColorCount::new(
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(vec![Color::Red, Color::Blue, Color::Green])
                )]
            )
        );
    }

    #[test]
    fn parses_a_valid_game_string_with_multiple_rounds() {
        let game_str = "Game 1: 1 red, 2 green, 3 blue; 4 red, 5 green, 6 blue";
        let game = parse_game_str(game_str).unwrap();
        assert_eq!(
            game,
            Game::new(
                1,
                vec![
                    ColorCount::new(
                        Some(1),
                        Some(2),
                        Some(3),
                        Some(vec![Color::Red, Color::Green, Color::Blue])
                    ),
                    ColorCount::new(
                        Some(4),
                        Some(5),
                        Some(6),
                        Some(vec![Color::Red, Color::Green, Color::Blue])
                    ),
                ]
            )
        );
    }

    #[test]
    fn round_is_valid() {
        let round = ColorCount::new(Some(1), Some(2), Some(3), None);
        let available = ColorCount::new(Some(1), Some(2), Some(3), None);
        assert!(is_round_valid(&round, &available));
    }

    #[test]
    fn round_is_invalid() {
        let round = ColorCount::new(Some(1), Some(2), Some(3), None);
        let available = ColorCount::new(Some(1), Some(2), Some(2), None);
        assert!(!is_round_valid(&round, &available));
    }

    /// Example from the Advent of Code website
    #[test]
    fn gets_minimum_game_1() {
        let game_1_str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game_1 = Game::parse(game_1_str).unwrap();

        let result = min_color_match(&game_1).unwrap();
        assert_eq!(result, ColorCount::new(Some(4), Some(2), Some(6), None));
    }

    /// Example from the Advent of Code website
    #[test]
    fn gets_minimum_game_3() {
        let game_3_str = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game_3 = Game::parse(game_3_str).unwrap();

        let result = min_color_match(&game_3).unwrap();
        assert_eq!(result, ColorCount::new(Some(20), Some(13), Some(6), None));
    }

    /// Example from the Advent of Code website
    #[test]
    fn gets_minimum_game_5() {
        let game_5_str = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let game_5 = Game::parse(game_5_str).unwrap();

        let result = min_color_match(&game_5).unwrap();
        assert_eq!(result, ColorCount::new(Some(6), Some(3), Some(2), None));
    }
}
