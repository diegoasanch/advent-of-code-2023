use std::process::ExitCode;

use day_1::digits::{get_first_digit, get_last_digit};
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
    let mut line_number;
    for line in lines.iter() {
        if let (Some(first_digit), Some(last_digit)) = (get_first_digit(line), get_last_digit(line))
        {
            line_number = [first_digit, last_digit]
                .iter()
                .collect::<String>()
                .parse::<i32>();
            match line_number {
                Ok(number) => {
                    sum += number;
                }
                Err(e) => {
                    eprintln!("Error parsing line number: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        }
    }

    println!("Result: {}", sum);

    return ExitCode::SUCCESS;
}
