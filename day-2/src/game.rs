use thiserror::Error;

use crate::color::ColorCount;

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
}

/// Determines if a game is valid
/// Validation criteria:
/// - All rounds must be valid
pub fn is_game_valid(game: &Game, available: &ColorCount) -> bool {
    game.rounds
        .iter()
        .all(|round| is_round_valid(round, available))
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
    use super::*;

    #[test]
    fn parses_a_valid_game_string() {
        let game_str = "Game 1: 1 red, 2 green, 3 blue";
        let game = parse_game_str(game_str).unwrap();
        assert_eq!(
            game,
            Game::new(
                1,
                vec![ColorCount {
                    red: Some(1),
                    green: Some(2),
                    blue: Some(3)
                }]
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
                    ColorCount {
                        red: Some(1),
                        green: Some(2),
                        blue: Some(3)
                    },
                    ColorCount {
                        red: Some(4),
                        green: Some(5),
                        blue: Some(6)
                    }
                ]
            )
        );
    }

    #[test]
    fn round_is_valid() {
        let round = ColorCount {
            red: Some(1),
            green: Some(2),
            blue: Some(3),
        };
        let available = ColorCount {
            red: Some(1),
            green: Some(2),
            blue: Some(3),
        };
        assert!(is_round_valid(&round, &available));
    }

    #[test]
    fn round_is_invalid() {
        let round = ColorCount {
            red: Some(1),
            green: Some(2),
            blue: Some(3),
        };
        let available = ColorCount {
            red: Some(1),
            green: Some(2),
            blue: Some(2),
        };
        assert!(!is_round_valid(&round, &available));
    }
}
