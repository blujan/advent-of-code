use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

static AOC_DAY: &str = "05";

lazy_static! {
    static ref NUMBER_MATCH: Regex = Regex::new(r"(\d+)").unwrap();
    static ref RANGES: Regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
}

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

fn get_seed_numbers(input: &str) -> Result<Vec<i64>, AoCError> {
    let mut seed_numbers: Vec<i64> = Vec::new();
    let numbers = input.lines().next().unwrap().split(':').nth(1).unwrap();
    for capture in NUMBER_MATCH.captures_iter(numbers) {
        let number = match capture[1].parse::<i64>() {
            Ok(x) => x,
            _ => return Err(AoCError::ParsingError(numbers.to_string())),
        };
        seed_numbers.push(number);
    }
    Ok(seed_numbers)
}

fn get_maps(input: &str) -> Result<Vec<Vec<(i64, i64, i64)>>, AoCError> {
    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    input
        .lines()
        .skip(2)
        .filter(|&line| !line.is_empty())
        .for_each(|line| {
            if line.starts_with(|c: char| c.is_alphabetic()) {
                maps.push(Vec::new());
                return;
            }
            let capture = match RANGES.captures(line) {
                Some(x) => x,
                None => return,
            };
            let dest = capture[1].parse::<i64>().unwrap();
            let source = capture[2].parse::<i64>().unwrap();
            let range = capture[3].parse::<i64>().unwrap() - 1;
            let diff = dest - source;
            maps.last_mut()
                .unwrap()
                .push((source + range, source, diff));
        });
    for map in maps.iter_mut() {
        map.sort();
    }
    Ok(maps)
}

/*
fn flatten_map(maps: &[Vec<(i64, i64, i64)>]) -> Vec<(i64, i64, i64)> {
}
*/

fn get_land_from_seed(seed: i64, maps: &[Vec<(i64, i64, i64)>]) -> Option<i64> {
    //println!("\n\n Entering get_land_from seed with seed = {}", seed);
    let mut curr = seed;
    for map in maps.iter() {
        let mut left = 0;
        let mut right = map.len() - 1;
        //println!("Now on: {:?} with curr = {}", map, curr);
        if curr < map[0].1 || curr > map.last().unwrap().0 {
            continue;
        }
        while left < right {
            let mid = left + (right - left) / 2;
            if curr >= map[left].1 && curr <= map[left].0 {
                break;
            }
            if map[mid].0 < curr {
                left = mid + 1;
            } else if map[mid].1 > curr {
                right = mid - 1;
            } else {
                left = mid;
            }
        }
        if curr >= map[left].1 && curr <= map[left].0 {
            curr += map[left].2;
        }
    }
    Some(curr)
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let seeds = get_seed_numbers(input)?;
    //println!("Seeds: {:?}", seeds);
    let maps = get_maps(input)?;
    let mut min_land = i64::MAX;
    for seed in seeds.iter() {
        let land = match get_land_from_seed(*seed, &maps) {
            Some(x) => x,
            None => return Err(AoCError::Unknown),
        };
        min_land = min_land.min(land);
    }

    Ok(min_land)
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    let seeds = get_seed_numbers(input)?;
    let maps = get_maps(input)?;
    let mut min_land = i64::MAX;
    for seed_range in seeds.chunks(2) {
        // @TODO Dealing with ranges might be tedious, but this is the very definition of brute force
        // Thankfully, Rust release builds are quite fast
        let base = seed_range[0];
        let end = base + seed_range[1];
        for i in base..end {
            let land = match get_land_from_seed(i, &maps) {
                Some(x) => x,
                None => return Err(AoCError::Unknown),
            };
            min_land = min_land.min(land);
        }
    }

    Ok(min_land)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() -> Result<()> {
        let input_1 = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process_part_1(input_1).unwrap(), 35);
        assert_eq!(process_part_2(input_1).unwrap(), 46);
        Ok(())
    }
}
