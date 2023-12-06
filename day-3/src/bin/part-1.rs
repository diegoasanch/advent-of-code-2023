use std::process::ExitCode;

use day_3::numbers::Schematic;
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

    match part_1_logic(&lines) {
        Ok(result) => {
            println!("Sum of part nums: {}", result);
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::FAILURE;
        }
    };
}

fn part_1_logic(game_lines: &Vec<String>) -> Result<u32, String> {
    let schematic = Schematic::parse(game_lines)
        .or_else(|err| Err(format!("Error parsing the schematic: {}", err.to_string())))?;
    Ok(schematic.get_part_numbers().iter().sum())
}
