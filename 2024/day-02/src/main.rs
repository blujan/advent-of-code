use anyhow::Result;
use thiserror::Error;
extern crate log;
extern crate pretty_env_logger;

const AOC_DAY: &str = "day-02";

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

fn parse_input_line(input: &str) -> Vec<i64> {
    let report = input
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    report
}

fn is_safe(report: Vec<i64>) -> bool {
    if report.len() <= 1 {
        return true;
    }
    let increasing = report.windows(2).all(|nums| {
        nums[0] < nums[1] && (nums[0].abs_diff(nums[1]) >= 1 && nums[0].abs_diff(nums[1]) <= 3)
    });
    let decreasing = report.windows(2).all(|nums| {
        nums[0] > nums[1] && (nums[0].abs_diff(nums[1]) >= 1 && nums[0].abs_diff(nums[1]) <= 3)
    });
    increasing || decreasing
}

fn initial_part_2_check(report: &[i64]) -> (bool, usize) {
    let mut unsafe_found = false;
    let mut increasing = false;
    for (i, num) in report.windows(2).enumerate() {
        if i == 0 {
            if num[1] > num[0] {
                increasing = true;
            }
            if num[1].abs_diff(num[0]) < 1 || num[1].abs_diff(num[0]) > 3 {
                unsafe_found = true;
            }
        } else {
            if (increasing && num[1] < num[0])
                || (!increasing && num[1] > num[0])
                || num[1].abs_diff(num[0]) < 1
                || num[1].abs_diff(num[0]) > 3
            {
                unsafe_found = true;
            }
            if unsafe_found {
                return (unsafe_found, i);
            }
        }
    }
    (false, 0)
}

fn is_safe_damper(report: Vec<i64>) -> bool {
    if report.len() <= 1 {
        return true;
    }
    let (unsafe_found, index) = initial_part_2_check(&report);
    if !unsafe_found {
        return true;
    }
    for offset in -1..2 {
        // Test removing before, at, and after the unsafe index
        let mut test_report = report.to_vec();
        let remove_index: usize = ((index as i32) + offset) as usize;
        test_report.remove(remove_index);
        if is_safe(test_report) {
            return true;
        }
    }
    false
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let safe_count = input
        .lines()
        .map(parse_input_line)
        .map(is_safe)
        .map(|result| match result {
            true => 1,
            false => 0,
        })
        .sum();
    Ok(safe_count)
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    let safe_count = input
        .lines()
        .map(parse_input_line)
        .map(is_safe_damper)
        .map(|result| match result {
            true => 1,
            false => 0,
        })
        .sum();
    Ok(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> Result<()> {
        let input_1 = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(process_part_1(input_1)?, 2);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let input_1 = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(process_part_2(input_1)?, 4);
        Ok(())
    }

    #[test]
    fn part_2_2() -> Result<()> {
        let input_1 = "9 1 2 3 4
1 2 3 4 1
1 5 2 3 4
1 2 3 4 9
4 3 2 1 9
1 4 3 2 1
4 3 2 5 1
9 4 3 2 1
1 9 10 11 12
1 6 2
2 0 4 5 6
8 11 9 11 14";
        assert_eq!(process_part_2(input_1)?, 12);
        Ok(())
    }
}
