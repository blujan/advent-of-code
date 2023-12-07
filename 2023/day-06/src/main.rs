use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

static AOC_DAY: &str = "06";

lazy_static! {
    static ref NUMBER_MATCH: Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Unable to parse the input `{0}`")]
    ParsingError(String),
    #[error("An unknown error has occurred (super duper helpful error)")]
    Unknown,
}

#[derive(Copy, Clone, Debug, Error)]
pub struct Race<T> {
    time: T,
    distance: T,
}

fn main() {
    let input: &str = include_str!("./input.txt");
    println!(
        "\n🎄🎄🎄🎄🎄 Advent of Code ||| Day {} 🎄🎄🎄🎄🎄\n",
        AOC_DAY
    );
    println!("Part 1 result\n\t{}\n", process_part_1(input).unwrap());
    println!("Part 2 result\n\t{}", process_part_2(input).unwrap());
}

fn parse_races(
    num: impl Into<String>,
    dist: impl Into<String>,
) -> Result<Vec<Race<i64>>, AoCError> {
    let numbers = num.into();
    let distances = dist.into();
    let mut races: Vec<Race<i64>> = Vec::new();
    for (num_capture, dist_capture) in std::iter::zip(
        NUMBER_MATCH.captures_iter(&numbers),
        NUMBER_MATCH.captures_iter(&distances),
    ) {
        let time = match num_capture[1].parse::<i64>() {
            Ok(x) => x,
            _ => return Err(AoCError::ParsingError(numbers)),
        };
        let distance = match dist_capture[1].parse::<i64>() {
            Ok(x) => x,
            _ => return Err(AoCError::ParsingError(distances)),
        };
        races.push(Race {
            time,
            distance: distance,
        });
    }
    Ok(races)
}

fn get_races(input: &str) -> Result<Vec<Race<i64>>, AoCError> {
    let mut iter = input.lines();
    let time_line = iter.next().unwrap();
    let dist_line = iter.next().unwrap();

    let numbers = time_line.split(':').nth(1).unwrap();
    let distances = dist_line.split(':').nth(1).unwrap();
    let races = parse_races(numbers, distances)?;
    Ok(races)
}

fn get_races_kerning(input: &str) -> Result<Vec<Race<i64>>, AoCError> {
    let mut iter = input.lines();
    let time_line = iter.next().unwrap();
    let dist_line = iter.next().unwrap();

    let numbers = time_line.split(':').nth(1).unwrap().replace(" ", "");
    let distances = dist_line.split(':').nth(1).unwrap().replace(" ", "");
    let races = parse_races(numbers, distances)?;
    Ok(races)
}

fn get_way_to_win(race: &Race<i64>) -> i64 {
    // Standard form: -x^2 + Time*x - Dist > 0
    // This is the number of integer points on the parabola > 0
    // Thanks to the answer from Hagen von Eitzen
    // https://math.stackexchange.com/questions/1867236/number-of-integers-between-two-real-numbers
    let time = race.time as f64;
    let dist = race.distance as f64;
    let sqrt = ((-dist + (time / 2.).powi(2)) * 4.).sqrt();
    let left = (-sqrt + time) / 2.;
    let right = (sqrt + time) / 2.;
    let count = (right.ceil() - left.floor() - 1.) as i64;
    count.max(0)
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let races = get_races(input)?;
    let result = races.iter().map(|race| get_way_to_win(race)).product();
    Ok(result)
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    let races = get_races_kerning(input)?;
    let result = races.iter().map(|race| get_way_to_win(race)).product();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() -> Result<()> {
        let input_1 = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process_part_1(input_1)?, 288);
        assert_eq!(process_part_2(input_1)?, 71503);
        Ok(())
    }
}
