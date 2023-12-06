use std::process::ExitCode;

use day_4::scratchcard::{
    array_score, find_common_numbers, get_card_numbers, get_winning_numbers, ScratchCardParseError,
};
use lib::input::read_file_lines;

fn main() -> ExitCode {
    // let input_file_path = "./inputs/test.txt";
    let input_file_path = "./inputs/input.txt";
    let lines;

    match read_file_lines(input_file_path) {
        Ok(_lines) => {
            lines = _lines;
        }
        Err(e) => {
            eprintln!("Error reading the input file: {}", e);
            return ExitCode::FAILURE;
        }
    };

    match part_1_logic(&lines) {
        Ok(result) => {
            println!("Sum of winning scores: {}", result);
        }
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

fn part_1_logic(game_lines: &Vec<String>) -> Result<u32, ScratchCardParseError> {
    let mut total = 0;

    for line in game_lines {
        let winning_numbers = get_winning_numbers(line)?;
        let card_numbers = get_card_numbers(line)?;
        let common_numbers = find_common_numbers(&winning_numbers, &card_numbers);
        let score = array_score(&common_numbers);
        total += score;
    }

    Ok(total)
}
