use anyhow::Result;
use regex::Regex;
use std::fmt::Error;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {
    let input: &str = include_str!("./input.txt");
    let result_1: i32 = input.split('\n').map(|line| part_1(line).unwrap()).sum();
    println!("Part 1 result: {}", result_1);
    let result_2: i32 = input.split('\n').map(|line| part_2(line).unwrap()).sum();
    println!("Part 2 result: {}", result_2);
}

fn part_1(input: &str) -> Result<i32, Error> {
    let cube_match = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let line: Vec<&str> = input.split(':').collect();
    let game_id = line[0][5..].parse::<i32>().unwrap();
    let games = line[1].split(';');
    for game in games {
        let cubes = game.split(',');
        for cube in cubes {
            let capture = cube_match.captures(cube).unwrap();
            let count = capture[1].parse::<i32>().unwrap();
            let cube_max = match &capture[2] {
                "red" => MAX_RED,
                "green" => MAX_GREEN,
                "blue" => MAX_BLUE,
                _ => return Err(Error),
            };
            if count > cube_max {
                return Ok(0);
            }
        }
    }
    Ok(game_id)
}

fn part_2(input: &str) -> Result<i32, Error> {
    let cube_match = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let line: Vec<&str> = input.split(':').collect();
    let games = line[1].split(';');
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for game in games {
        let cubes = game.split(',');
        for cube in cubes {
            let capture = cube_match.captures(cube).unwrap();
            let count = capture[1].parse::<i32>().unwrap();
            match &capture[2] {
                "red" => red = std::cmp::max(red, count),
                "green" => green = std::cmp::max(green, count),
                "blue" => blue = std::cmp::max(blue, count),
                _ => return Err(Error),
            };
        }
    }
    Ok(red * green * blue)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() -> Result<()> {
        let input_1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let input_2 = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let input_3 = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let input_4 = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let input_5 = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_1(input_1).unwrap(), 1);
        assert_eq!(part_1(input_2).unwrap(), 2);
        assert_eq!(part_1(input_3).unwrap(), 0);
        assert_eq!(part_1(input_4).unwrap(), 0);
        assert_eq!(part_1(input_5).unwrap(), 5);
        Ok(())
    }
    #[test]
    fn test_2() -> Result<()> {
        let input_1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let input_2 = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let input_3 = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let input_4 = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let input_5 = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_2(input_1).unwrap(), 48);
        assert_eq!(part_2(input_2).unwrap(), 12);
        assert_eq!(part_2(input_3).unwrap(), 1560);
        assert_eq!(part_2(input_4).unwrap(), 630);
        assert_eq!(part_2(input_5).unwrap(), 36);
        Ok(())
    }
}
