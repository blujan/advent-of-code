use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

static AOC_DAY: &str = "07";

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

//#[derive(Copy, Clone, Debug, Error)]
pub struct Points {
    hand: i32,
    mult: i64,
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

fn card_to_points_1(c: char) -> i32 {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

fn card_to_points_2(c: char) -> i32 {
    match c {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

fn calc_points_1(hand: &str, multiplier: i64) -> Points {
    let mut hand_score = hand
        .chars()
        .map(card_to_points_1)
        .rev()
        .enumerate()
        .map(|(pos, card)| card << (pos * 4))
        .sum();
    let card_counts = hand
        .chars()
        .counts()
        .into_values()
        .map(|num| num as i32)
        .sorted_unstable_by(|a, b| b.cmp(a))
        .collect::<Vec<i32>>();
    hand_score += match (card_counts[0], card_counts.get(1)) {
        (5, _) => 6 << 20,
        (4, _) => 5 << 20,
        (3, Some(x)) => (x + 2) << 20,
        (2, Some(x)) => x << 20,
        _ => 0,
    };
    Points {
        hand: hand_score,
        mult: multiplier,
    }
}

fn calc_points_2(hand: &str, multiplier: i64) -> Points {
    let mut hand_score = hand
        .chars()
        .map(card_to_points_2)
        .rev()
        .enumerate()
        .map(|(pos, card)| card << (pos * 4))
        .sum();
    let mut initial_counts = hand.chars().counts();
    let num_jokers = match initial_counts.remove(&'J') {
        Some(x) => x as i32,
        _ => 0,
    };
    let card_counts = initial_counts
        .into_values()
        .map(|num| num as i32)
        .sorted_unstable_by(|a, b| b.cmp(a))
        .collect::<Vec<i32>>();
    let highest = match card_counts.first() {
        Some(x) => *x,
        _ => 0,
    };
    hand_score += match (highest + num_jokers, card_counts.get(1)) {
        (5, _) => 6 << 20,
        (4, _) => 5 << 20,
        (3, Some(x)) => (x + 2) << 20,
        (2, Some(x)) => x << 20,
        _ => 0,
    };
    Points {
        hand: hand_score,
        mult: multiplier,
    }
}

fn get_hand(input: &str) -> Result<(&str, i64), AoCError> {
    let (hand, mult) = match input.split(' ').collect_tuple() {
        Some(x) => x,
        _ => return Err(AoCError::ParsingError(input.to_string())),
    };
    let capture = match NUMBER_MATCH.captures(mult) {
        Some(x) => x,
        _ => return Err(AoCError::ParsingError(mult.to_string())),
    };
    let multiplier = capture[1].parse::<i64>().unwrap();
    Ok((hand, multiplier))
}

fn process(input: &str, calc: fn(&str, i64) -> Points) -> Result<i64, AoCError> {
    let points = input
        .lines()
        .map(|line| get_hand(line).unwrap())
        .map(|(hand, mult)| calc(hand, mult))
        .sorted_unstable_by(|a, b| a.hand.cmp(&b.hand))
        .collect::<Vec<Points>>();
    let result = points
        .iter()
        .enumerate()
        .map(|(pos, points)| ((pos as i64) + 1) * points.mult)
        .sum();
    Ok(result)
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    process(input, calc_points_1)
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    process(input, calc_points_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() -> Result<()> {
        let input_1 = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process_part_1(input_1)?, 6440);
        assert_eq!(process_part_2(input_1)?, 5905);
        Ok(())
    }
}
