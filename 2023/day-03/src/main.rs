use anyhow::Result;
use phf::{phf_set, Set};
use std::collections::{HashMap, HashSet};
use std::fmt::Error;

// Standard ascii symbols in PSF (perfect hash function) static set
// I feel like treating everything like ASCII violates the sprit of
// Rust's UTF-8 default, hence the set rather than just a set of if's
// on the integer value of the char
static SYMBOLS: Set<char> = phf_set! {
    '!',
    '"',
    '#',
    '$',
    '%',
    '&',
    '\'',
    '(',
    ')',
    '*',
    '+',
    ',',
    '-',
    '/',
    ':',
    ';',
    '<',
    '=',
    '>',
    '?',
    '@',
    '[',
    '\\',
    ']',
    '^',
    '_',
    '`',
    '{',
    '|',
    '}',
    '~',
};

static MOVES: [[i32; 2]; 8] = [
    [-1, 0],
    [1, 0],
    [0, 1],
    [0, -1],
    [1, 1],
    [1, -1],
    [-1, 1],
    [-1, -1],
];

fn is_symbol(c: char) -> bool {
    SYMBOLS.contains(&c)
}

fn main() {
    let input: &str = include_str!("./input.txt");
    println!("Part 1 result: {}", process_input_part_1(input).unwrap());
    println!("Part 2 result: {}", process_input_part_2(input).unwrap());
}

fn get_sym_locations(input: &str, sym_locations: &mut HashSet<(i32, i32)>) {
    for (num, line) in input.lines().enumerate() {
        for (index, c) in line.chars().enumerate() {
            if is_symbol(c) {
                sym_locations.insert((index.try_into().unwrap(), num.try_into().unwrap()));
            }
        }
    }
}

fn is_connected(x: i32, y: i32, sym_locations: &mut HashSet<(i32, i32)>) -> bool {
    for direction in MOVES.iter() {
        let next_x = x + direction[0];
        let next_y = y + direction[1];
        if sym_locations.contains(&(next_x, next_y)) {
            return true;
        }
    }
    false
}

fn get_unconnected_nums(
    num: usize,
    line: &str,
    sym_locations: &mut HashSet<(i32, i32)>,
) -> Result<u32, Error> {
    let mut result: u32 = 0;
    let mut found_num: u32 = 0;
    let mut connected: bool = false;
    for (index, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if !connected {
                let y: i32 = num.try_into().unwrap();
                let x: i32 = index.try_into().unwrap();
                connected = is_connected(x, y, sym_locations);
            }
            found_num = (found_num * 10) + c.to_digit(10).unwrap();
        } else {
            if connected && found_num > 0 {
                result += found_num;
            }
            found_num = 0;
            connected = false;
        }
    }
    if connected && found_num > 0 {
        result += found_num;
    }
    Ok(result)
}

fn process_input_part_1(input: &str) -> Result<u32, Error> {
    let mut sym_locations: HashSet<(i32, i32)> = HashSet::new();
    get_sym_locations(input, &mut sym_locations);
    let count = input
        .lines()
        .enumerate()
        .map(|(num, line)| get_unconnected_nums(num, line, &mut sym_locations).unwrap())
        .sum();
    Ok(count)
}

fn get_gear_points(input: &str, gear_points: &mut HashMap<(i32, i32), Vec<u32>>) {
    for (num, line) in input.lines().enumerate() {
        for (index, c) in line.chars().enumerate() {
            if c == '*' {
                gear_points.insert(
                    (index.try_into().unwrap(), num.try_into().unwrap()),
                    Vec::<u32>::new(),
                );
            }
        }
    }
}

fn is_geared(x: i32, y: i32, gear_points: &mut HashMap<(i32, i32), Vec<u32>>) -> (bool, i32, i32) {
    for direction in MOVES.iter() {
        let next_x = x + direction[0];
        let next_y = y + direction[1];
        if gear_points.contains_key(&(next_x, next_y)) {
            return (true, next_x, next_y);
        }
    }
    (false, 0, 0)
}

fn connect_nums_to_gears(num: usize, line: &str, gear_points: &mut HashMap<(i32, i32), Vec<u32>>) {
    let mut found_num: u32 = 0;
    let mut connected: bool = false;
    let mut gear_x = 0;
    let mut gear_y = 0;
    for (index, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if !connected {
                let y: i32 = num.try_into().unwrap();
                let x: i32 = index.try_into().unwrap();
                (connected, gear_x, gear_y) = is_geared(x, y, gear_points);
            }
            found_num = (found_num * 10) + c.to_digit(10).unwrap();
        } else {
            if connected {
                if let Some(nums) = gear_points.get_mut(&(gear_x, gear_y)) {
                    nums.push(found_num);
                }
            }
            found_num = 0;
            connected = false;
        }
    }
    if connected {
        if let Some(nums) = gear_points.get_mut(&(gear_x, gear_y)) {
            nums.push(found_num);
        }
    }
}

fn process_input_part_2(input: &str) -> Result<u32, Error> {
    let mut gear_points: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
    get_gear_points(input, &mut gear_points);
    for (num, line) in input.lines().enumerate() {
        connect_nums_to_gears(num, line, &mut gear_points);
    }
    let count: u32 = gear_points
        .iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, nums)| nums.iter().product::<u32>())
        .sum();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() -> Result<()> {
        let input_1 = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process_input_part_1(input_1).unwrap(), 4361);
        Ok(())
    }
    #[test]
    fn test_2() -> Result<()> {
        let input_2 = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process_input_part_2(input_2).unwrap(), 467835);
        Ok(())
    }
}
