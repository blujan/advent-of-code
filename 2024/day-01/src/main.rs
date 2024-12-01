use anyhow::Result;
use thiserror::Error;
use std::collections::HashMap;
extern crate pretty_env_logger;
extern crate log;

const AOC_DAY: &str = "01";

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
        "\n🎄🎄🎄🎄🎄 Advent of Code 2024 ||| Day {} 🎄🎄🎄🎄🎄\n",
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

fn parse_line(input: &str) -> Result<(i64, i64), AoCError> {
    let line = input.trim();
    let mut num_split = line.split_whitespace();

    let left = match num_split.next().unwrap().parse::<i64>() {
        Ok(x) => x,
        _ => return Err(AoCError::ParsingError(line.to_string())),
    };
    let right = match num_split.next().unwrap().parse::<i64>() {
        Ok(x) => x,
        _ => return Err(AoCError::ParsingError(line.to_string())),
    };
    Ok((left, right))
}

fn parse_part_1(input: &str) -> Result<(Vec<i64>, Vec<i64>), AoCError> {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for line in input.lines() {
        let (l_num, r_num) = parse_line(line)?;
        left.push(l_num);
        right.push(r_num);
    }
    Ok((left, right))
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    // Parse
    let (mut left, mut right) = parse_part_1(input)?;
    left.sort();
    right.sort();

    // Calc
    let acc = left
        .iter()
        .zip(right)
        .map(|num| (num.0 - num.1).abs())
        .sum();

    Ok(acc)
}

fn get_freq_map(input: &[i64]) -> HashMap<i64, i64> {
    let mut freq_map = HashMap::new();
    for num in input {
        freq_map
            .entry(*num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    freq_map
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    // Parse
    let (left, right) = parse_part_1(input)?;

    // Get freq map
    let freq_map = get_freq_map(&right);

    // Calculate answer
    let acc = left
        .iter()
        .map(|num| {
            let count = match freq_map.get(num) {
                Some(x) => *x,
                _ => 0,
            };
            count * num
        })
        .sum();

    Ok(acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() -> Result<()> {
        let input_1 = "00003   00004
00004   00003
00002   00005
00001   00003
00003   00009
00003   00003";
        assert_eq!(process_part_1(input_1)?, 11);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let input_1 = "00003   00004
00004   00003
00002   00005
00001   00003
00003   00009
00003   00003";
        assert_eq!(process_part_2(input_1)?, 31);
        Ok(())
    }
}
