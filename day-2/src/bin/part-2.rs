use std::process::ExitCode;

use day_2::game::Game;
use lib::input::read_file_lines;

fn main() -> ExitCode {
    // let path = "./inputs/test.txt";
    let path = "./inputs/part-1.txt";

    let lines = match read_file_lines(path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading the input file: {}", e);
            return ExitCode::FAILURE;
        }
    };

    match day_2_logic(&lines) {
        Ok(win_id_sum) => {
            println!("Sum of winning ids: {}", win_id_sum);
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::FAILURE;
        }
    };
}

fn day_2_logic(game_lines: &Vec<String>) -> Result<u32, String> {
    let mut min_game_sum = 0;

    for line in game_lines.iter() {
        let game = match Game::parse(line) {
            Ok(game) => game,
            Err(e) => {
                return Err(format!("Error parsing the game: {}", e));
            }
        };

        min_game_sum += game
            .min_color_match()
            .ok_or_else(|| "No valid color match found".to_string())?
            .power();
    }

    return Ok(min_game_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Example from the Advent of Code website
    /// https://adventofcode.com/2023/day/2
    #[test]
    fn website_example() {
        let website_example_input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];

        let min_game_power_sum = day_2_logic(&website_example_input).unwrap();
        assert_eq!(min_game_power_sum, 2286);
    }
}
