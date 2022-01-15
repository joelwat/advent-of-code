mod sub_command;

use std::path::Path;

use sub_command::{
    Command,
    SubCommand,
};
use utils::{
    app_errors::AppError,
    file_reader::load_data,
};

pub fn run_part1() -> Result<(), AppError> {
    let mut depth: u64 = 0;
    let mut distance: u64 = 0;

    let file_path = Path::new("2021/day2/input");

    let file_data = load_data::<SubCommand>(file_path, &parse_command)?;

    for line in file_data {
        handle_command(&line, &mut depth, &mut distance);
    }

    println!("Depth: {}", depth);
    println!("Distance: {}", distance);
    println!();

    println!("Puzzle answer: {}", depth * distance);

    Ok(())
}

pub fn run_part2() -> Result<(), AppError> {
    let mut aim: u64 = 0;
    let mut depth: u64 = 0;
    let mut distance: u64 = 0;

    let file_path = Path::new("2021/day2/input");

    let file_data = load_data::<SubCommand>(file_path, &parse_command)?;

    for line in file_data {
        handle_command_part2(&line, &mut depth, &mut distance, &mut aim);
    }

    println!("Depth: {}", depth);
    println!("Distance: {}", distance);
    println!();

    println!("Puzzle answer: {}", depth * distance);

    Ok(())
}

pub fn parse_command(str: &str) -> Result<SubCommand, AppError> {
    let parts: Vec<&str> = str.split_whitespace().clone().into_iter().collect();

    let parsed = SubCommand::try_from(parts)?;

    Ok(parsed)
}

fn handle_command(command: &SubCommand, depth: &mut u64, distance: &mut u64) {
    match command.0 {
        Command::Down(val) => *depth += val,
        Command::Forward(val) => *distance += val,
        Command::Up(val) => *depth -= val,
    }
}

fn handle_command_part2(command: &SubCommand, depth: &mut u64, distance: &mut u64, aim: &mut u64) {
    match command.0 {
        Command::Down(x) => *aim += x,
        Command::Forward(x) => {
            *distance += x;
            *depth += *aim * x;
        },
        Command::Up(x) => *aim -= x,
    }
}
