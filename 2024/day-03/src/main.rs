use anyhow::Result;
use thiserror::Error;
extern crate log;
extern crate pretty_env_logger;
use lazy_static::lazy_static;
use regex::Regex;

const AOC_DAY: &str = "day-03";

lazy_static! {
    static ref MUL_MATCH: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
}

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

fn parse_part_1(line: &str) -> Result<Vec<(i64, i64)>, AoCError> {
    let number_pairs = MUL_MATCH
        .captures_iter(line)
        .map(|captures| {
            let (_, [l, r]) = captures.extract();
            (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap())
        })
        .collect();
    Ok(number_pairs)
}

fn parse_part_2(line: &str) -> Result<Vec<(usize, i64, i64)>, AoCError> {
    let number_pairs = MUL_MATCH
        .captures_iter(line)
        .map(|captures| {
            let (_, [l, r]) = captures.extract();
            let capture_match = captures.get(0).unwrap();
            (
                capture_match.start(),
                l.parse::<i64>().unwrap(),
                r.parse::<i64>().unwrap(),
            )
        })
        .collect();
    Ok(number_pairs)
}

fn get_all_actions(input: &str) -> Result<Vec<(usize, i64, i64)>, AoCError> {
    let pairs = match parse_part_2(input) {
        Ok(x) => x,
        _ => return Err(AoCError::ParsingError("".to_string())),
    };
    let do_pos: Vec<(usize, i64, i64)> = input
        .match_indices("do()")
        .map(|(index, _)| (index, -1, -1))
        .collect();
    let dont_pos: Vec<(usize, i64, i64)> = input
        .match_indices("don't()")
        .map(|(index, _)| (index, -2, -2))
        .collect();
    let mut actions = vec![pairs, do_pos, dont_pos].concat();
    actions.sort();
    Ok(actions)
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let pairs = input
        .lines()
        .map(parse_part_1)
        .collect::<Result<Vec<_>, _>>()?;
    let result = pairs.iter().flatten().map(|pair| pair.0 * pair.1).sum();
    Ok(result)
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    let actions = get_all_actions(input)?;
    let mut active = true;
    let sum = actions
        .iter()
        .map(|(_, l, r)| {
            if *l == -1 {
                active = true;
                return 0;
            }
            if *l == -2 {
                active = false;
                return 0;
            }
            if active {
                return *l * *r;
            }
            0
        })
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> Result<()> {
        let input_1 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process_part_1(input_1)?, 161);
        Ok(())
    }
    #[test]
    fn part_2() -> Result<()> {
        let input_1 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(process_part_2(input_1)?, 48);
        Ok(())
    }
}
