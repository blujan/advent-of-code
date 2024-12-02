use anyhow::Result;
use thiserror::Error;
extern crate pretty_env_logger;
extern crate log;

const AOC_DAY: &str = "{{project-name}}";

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Unable to parse the input `{0}`")]
    ParsingError(String),
    #[error("Unable to calc the input `{0}`")]
    CalcError(String),
    #[error("Unable to output value `{0}`")]
    OutputError(String),
    #[error("An unknown error has occurred")]
    Unknown,
}

fn main() {
    let _ = pretty_env_logger::try_init();
    const INPUT: &str = include_str!("./input");
    println!(
        "\n🎄🎄🎄🎄🎄 Advent of Code 2024 ||| {} 🎄🎄🎄🎄🎄\n",
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

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    todo!("part1")
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    todo!("part2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> Result<()> {
        let input_1 = "";
        assert_eq!(process_part_1(input_1)?, 11);
        Ok(())
    }

}
