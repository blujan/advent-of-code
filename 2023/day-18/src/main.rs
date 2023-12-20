use anyhow::Result;
use day_18::{Command, MOVES};
use thiserror::Error;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

const AOC_DAY: &str = "18";

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Unable to parse the input `{0}`")]
    ParsingError(String),
    #[error("An unknown error has occurred (super duper helpful error)")]
    Unknown,
}

fn main() {
    let _ = pretty_env_logger::try_init();
    const INPUT: &str = include_str!("./input.txt");
    println!(
        "\n🎄🎄🎄🎄🎄 Advent of Code ||| Day {} 🎄🎄🎄🎄🎄\n",
        AOC_DAY
    );
    match process_part_1(INPUT) {
        Ok(result) => println!("Part 1 result\n\t{}\n", result),
        Err(e) => println!("Error: {}", e),
    }
    match process_part_2(INPUT) {
        Ok(result) => println!("Part 2 result\n\t{}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn create_command(input: &str) -> Result<Command, AoCError> {
    match Command::new(input) {
        Some(x) => Ok(x),
        _ => Err(AoCError::ParsingError(input.to_string())),
    }
}

fn polygon_area(commands: &[Command]) -> i64 {
    let mut area = 0;
    let mut path_length = 0;
    let mut row = 0;
    let mut col = 0;
    for command in commands.iter() {
        let next_col = col + MOVES[command.direction][0] * command.steps as i64;
        let next_row = row + MOVES[command.direction][1] * command.steps as i64;
        path_length += command.steps as i64;
        area += (row - next_row) * (col + next_col);
        row = next_row;
        col = next_col;
    }

    (area / 2).abs() + path_length / 2 + 1
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let command_result: Result<Vec<Command>, _> = input.lines().map(create_command).collect();
    let commands = command_result.unwrap();
    trace!("{:?}", commands);
    let area = polygon_area(&commands);
    Ok(area)
}

fn create_command_2(input: &str) -> Result<Command, AoCError> {
    match Command::new_part2(input) {
        Some(x) => Ok(x),
        _ => Err(AoCError::ParsingError(input.to_string())),
    }
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    let command_result: Result<Vec<Command>, _> = input.lines().map(create_command_2).collect();
    let commands = command_result.unwrap();
    let area = polygon_area(&commands);
    Ok(area)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() -> Result<()> {
        let _ = pretty_env_logger::try_init();
        let input_1 = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(process_part_1(input_1)?, 62);
        Ok(())
    }
    #[test]
    fn part_2() -> Result<()> {
        let _ = pretty_env_logger::try_init();
        let input_1 = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(process_part_2(input_1)?, 952408144115);
        Ok(())
    }
}
