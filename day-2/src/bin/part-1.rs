use std::process::ExitCode;

use day_2::{
    color::ColorCount,
    game::{is_game_valid, Game},
};
use lib::input::read_file_lines;

fn main() -> ExitCode {
    // let path = "./inputs/test.txt";
    let path = "./inputs/part-1.txt";
    let available_colors = ColorCount::new(Some(12), Some(13), Some(14));

    let lines = match read_file_lines(path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading the input file: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let mut win_id_sum = 0;

    for line in lines.iter() {
        let game = match Game::parse(line) {
            Ok(game) => game,
            Err(e) => {
                eprintln!("Error parsing the game: {}", e);
                return ExitCode::FAILURE;
            }
        };

        if is_game_valid(&game, &available_colors) {
            win_id_sum += game.id;
        }
    }

    println!("Sum of winning ids: {}", win_id_sum);
    return ExitCode::SUCCESS;
}
