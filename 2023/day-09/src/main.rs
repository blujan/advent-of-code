use anyhow::Result;
use thiserror::Error;

static AOC_DAY: &str = "09";

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Unable to parse the input `{0}`")]
    ParsingError(String),
    #[error("An unknown error has occurred (super duper helpful error)")]
    Unknown,
}

fn main() {
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

fn parse_line(input: &str) -> Result<Vec<i64>, AoCError> {
    let line = input.trim();
    let mut output: Vec<i64> = Vec::new();
    for num_str in line.split_whitespace() {
        let number = match num_str.parse::<i64>() {
            Ok(x) => x,
            _ => return Err(AoCError::ParsingError(line.to_string())),
        };
        output.push(number);
    }
    Ok(output)
}

fn parse_input(input: &str) -> Result<Vec<Vec<i64>>, AoCError> {
    let mut output: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let result = parse_line(line)?;
        output.push(result);
    }
    Ok(output)
}

fn build_nums(numbers: &[i64]) -> Vec<Vec<i64>> {
    let mut triangle: Vec<Vec<i64>> = Vec::new();
    triangle.push(numbers.to_vec());
    let mut check: i64 = 1;
    while check != 0 {
        check = 0;
        let next = triangle
            .last()
            .unwrap()
            .windows(2)
            .map(|nums| {
                let num = nums[1] - nums[0];
                check |= num;
                num
            })
            .collect();
        if check != 0 {
            triangle.push(next);
        }
    }
    triangle
}

fn extrapolate_forward(numbers: &[i64]) -> i64 {
    let triangle = build_nums(numbers);
    let value = triangle
        .iter()
        .map(|line| line.last().unwrap())
        .sum::<i64>();
    value
}

fn extrapolate_back(numbers: &[i64]) -> i64 {
    let triangle = build_nums(numbers);
    let mut last = 0;
    triangle.iter().rev().for_each(|line| {
        let value = line.first().unwrap() - last;
        last = value;
    });
    last
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let numbers = parse_input(input)?;
    let result = numbers.iter().map(|line| extrapolate_forward(line)).sum();
    Ok(result)
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    let numbers = parse_input(input)?;
    let result = numbers.iter().map(|line| extrapolate_back(line)).sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn par1_1() -> Result<()> {
        let input_1 = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
        assert_eq!(process_part_1(input_1)?, 114);
        Ok(())
    }
    #[test]
    fn part_2() -> Result<()> {
        let input_1 = "10 13 16 21 30 45";
        let input_2 = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(process_part_2(input_1)?, 5);
        assert_eq!(process_part_2(input_2)?, 2);
        Ok(())
    }
}
