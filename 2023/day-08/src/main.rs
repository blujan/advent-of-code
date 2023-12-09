use std::{collections::HashMap, iter::Cycle, str::Chars};
use anyhow::Result;
use thiserror::Error;

static AOC_DAY: &str = "08";

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Unable to parse the input `{0}`")]
    ParsingError(String),
    #[error("An unknown error has occurred (super duper helpful error)")]
    Unknown,
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

fn node_to_int(node: &str) -> Option<i32> {
    if node.len() != 3 {
        return None;
    }
    let mut value: i32 = 0;
    node.bytes()
        .rev()
        .enumerate()
        .for_each(|(index, c)| value += (c as i32) << (index * 8));
    Some(value)
}

fn get_moves(input: &str) -> Option<Cycle<Chars<'_>>> {
    Some(input.lines().next()?.chars().cycle())
}

fn parse_line(line: &str) -> Option<(i32, i32, i32)> {
    let (mut source, destinations) = line.split_once('=')?;
    let (mut left, mut right) = destinations.split_once(',')?;
    source = source.trim();
    left = left.trim().trim_matches('(');
    right = right.trim().trim_matches(')');
    let source_num = node_to_int(source)?;
    let left_num = node_to_int(left)?;
    let right_num = node_to_int(right)?;
    Some((source_num, left_num, right_num))
}

fn get_map(input: &str) -> Result<HashMap<i32, (i32, i32)>, AoCError> {
    let mut map: HashMap<i32, (i32, i32)> = HashMap::new();
    for line in input.lines().skip(2) {
        let (source_num, left_num, right_num) = match parse_line(line) {
            Some(x) => x,
             _ => return Err(AoCError::ParsingError(line.to_string())),
        };
        map.insert(source_num, (left_num, right_num));
    }
    Ok(map)
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let mut moves = match get_moves(input) {
        Some(x) => x,
        _=> return Err(AoCError::ParsingError("Unable to parse first line".to_string())),
    };
    let map = get_map(input)?;
    let mut result: i64 = 0;
    let mut current = node_to_int("AAA").unwrap();
    let destination = node_to_int("ZZZ").unwrap();
    while current != destination {
        let direction = moves.next().unwrap();
        let next = match map.get(&current) {
            Some(x) => x,
            _ => return Err(AoCError::Unknown),
        };
        current = match direction {
            'L' => next.0,
            'R' => next.1,
            _ => panic!("Unknown direction: {}", direction),
        };
        result += 1;
    }
    Ok(result)
}

fn get_ends_with(c: char, map: &HashMap<i32, (i32, i32)>) -> Vec<i32> {
    let value = c as i32;
    let nodes: Vec<i32> = map
        .keys()
        .filter(|node| (**node & 0xFF) == value)
        .copied()
        .collect();
    nodes
}

fn vec_lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    num::integer::lcm::<i64>(nums[0], vec_lcm(&nums[1..]))
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    let map = get_map(input)?;
    let starting_positions = get_ends_with('A', &map);
    let mut result: Vec<i64> = Vec::new();
    let moves_start = match get_moves(input) {
        Some(x) => x,
        _=> return Err(AoCError::ParsingError("Unable to parse first line".to_string())),
    };
    for pos in starting_positions.iter() {
        let mut current = *pos;
        let mut count: i64 = 0;
        let mut moves = moves_start.clone();
        while (current & 0xFF) != ('Z' as i32) {
            let direction = moves.next().unwrap();
            let next = match map.get(&current) {
                Some(x) => x,
                _ => return Err(AoCError::Unknown),
            };
            current = match direction {
                'L' => next.0,
                'R' => next.1,
                _ => panic!("Unknown direction: {}", direction),
            };
            count += 1;
        }
        result.push(count);
    }
    Ok(vec_lcm(&result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn par1_1() -> Result<()> {
        let input_1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let input_2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process_part_1(input_1)?, 2);
        assert_eq!(process_part_1(input_2)?, 6);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let input_2 = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(process_part_2(input_2)?, 6);
        Ok(())
    }
}
