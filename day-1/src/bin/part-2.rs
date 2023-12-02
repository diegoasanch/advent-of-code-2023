use std::process::ExitCode;

use day_1::digits::{get_first_number, get_last_number};
use lib::input::read_file_lines;

fn main() -> ExitCode {
    // let input_file_path = "./inputs/test.txt";
    let input_file_path = "./inputs/part-1.txt";
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

    let mut sum = 0;
    let mut line_number: u32;
    for line in lines.iter() {
        if let (Some(first_digit), Some(last_digit)) =
            (get_first_number(line), get_last_number(line))
        {
            line_number = (first_digit as u32) * 10 + (last_digit as u32);
            sum += line_number;
        }
    }

    println!("Result: {}", sum);

    return ExitCode::SUCCESS;
}
