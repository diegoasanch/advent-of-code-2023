use std::{collections::HashMap, process::ExitCode};

use day_4::scratchcard::{
    find_common_numbers, get_card_numbers, get_winning_numbers, ScratchCardParseError,
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

    match part_2_logic(&lines) {
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

fn part_2_logic(game_lines: &Vec<String>) -> Result<u32, ScratchCardParseError> {
    let mut total = 0;
    let mut winning_copies = HashMap::new();
    let mut ids_to_process = Vec::new();

    for (i, line) in game_lines.iter().enumerate() {
        let winning_numbers = get_winning_numbers(line)?;
        let card_numbers = get_card_numbers(line)?;
        let common_numbers = find_common_numbers(&winning_numbers, &card_numbers);
        let card_id = i + 1;
        total += 1;

        if common_numbers.len() > 0 {
            let new_cards: Vec<usize> =
                ((card_id + 1)..=(card_id + common_numbers.len())).collect();
            winning_copies.insert(card_id, new_cards.clone());
            ids_to_process.extend(new_cards);
        }
    }

    while !ids_to_process.is_empty() {
        let id = ids_to_process.pop().expect("Should have, we just checked");
        total += 1;
        if let Some(ids) = winning_copies.get(&id) {
            ids_to_process.extend(ids);
        }
    }

    Ok(total)
}
