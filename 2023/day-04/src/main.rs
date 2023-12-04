use anyhow::Result;
use regex::Regex;
use std::{collections::HashSet, fmt::Error};

fn main() {
    let input: &str = include_str!("./input.txt");
    println!("Part 1 result: {}", process_part_1(input).unwrap());
    println!("Part 2 result: {}", process_part_2(input).unwrap());
}

fn get_matches(input: &str) -> Result<i32, Error> {
    let number_match = Regex::new(r"(\d+)").unwrap();
    let numbers = input.split(':').nth(1).unwrap();
    let (plays, win_nums) = numbers.split_once('|').unwrap();
    let mut winning_numbers: HashSet<i32> = HashSet::new();
    for capture in number_match.captures_iter(win_nums) {
        let number = capture[1].parse::<i32>().unwrap();
        winning_numbers.insert(number);
    }
    let mut wins = 0;
    for capture in number_match.captures_iter(plays) {
        let number = capture[1].parse::<i32>().unwrap();
        if winning_numbers.contains(&number) {
            wins += 1;
        }
    }
    Ok(wins)
}

fn process_part_1(input: &str) -> Result<i32, Error> {
    let result = input
        .lines()
        .map(|line| get_matches(line).unwrap())
        .filter(|x| *x > 0)
        .map(|x| 1 << (x - 1))
        .sum();
    Ok(result)
}

fn process_part_2(input: &str) -> Result<i32, Error> {
    let number_cards = input.lines().count();
    let mut card_count = vec![1; number_cards];
    for (num, line) in input.lines().enumerate() {
        let mut wins = get_matches(line).unwrap();
        let mut index = num + 1;
        while wins > 0 && index < number_cards {
            card_count[index] += card_count[num];
            index += 1;
            wins -= 1;
        }
    }
    let result = card_count.iter().sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() -> Result<()> {
        let input_1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process_part_1(input_1).unwrap(), 13);
        Ok(())
    }
    #[test]
    fn test_2() -> Result<()> {
        let input_2 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process_part_2(input_2).unwrap(), 30);
        Ok(())
    }
}
