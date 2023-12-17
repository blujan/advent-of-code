use anyhow::Result;
use ordered_hash_map::OrderedHashMap;
use std::collections::HashMap;
use thiserror::Error;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

const AOC_DAY: &str = "15";

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Unable to parse the input `{0}`")]
    ParsingError(String),
    #[error("An unknown error has occurred (super duper helpful error)")]
    Unknown,
}

fn main() {
    pretty_env_logger::init();
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

fn hash(input: &str) -> i64 {
    input
        .chars()
        .fold(0, |acc, ch| ((acc + ch as i64) * 17) % 256)
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let result = input.split(',').map(hash).sum();
    Ok(result)
}

fn process_command<'a>(
    input: &'a str,
    box_array: &mut [OrderedHashMap<&'a str, i32>],
    label_map: &mut HashMap<&'a str, usize>,
) -> Result<(), AoCError> {
    if input.ends_with('-') {
        let label = &input[0..input.len() - 1];
        let box_result = label_map.remove(label);
        if let Some(box_num) = box_result {
            box_array[box_num].remove(label);
        }
    } else {
        let (label, f_size) = match input.split_once('=') {
            Some(x) => x,
            _ => return Err(AoCError::ParsingError(input.to_string())),
        };
        let f_number = f_size.parse::<i32>().unwrap();
        // Update or add
        let box_result = label_map.get(label);
        if let Some(box_num) = box_result {
            let current_value = match box_array[*box_num].get_mut(label) {
                Some(x) => x,
                _ => return Err(AoCError::ParsingError(input.to_string())),
            };
            *current_value = f_number;
            return Ok(());
        }
        let box_num = hash(label) as usize;
        trace!("Adding {}, {} to box {}", label, f_number, box_num);
        label_map.insert(label, box_num);
        box_array[box_num].insert(label, f_number);
    }
    Ok(())
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    let mut box_array: Vec<OrderedHashMap<&str, i32>> = vec![OrderedHashMap::new(); 256];
    let mut label_map: HashMap<&str, usize> = HashMap::new(); // Label -> Box# storage
    for command in input.split(',') {
        process_command(command, &mut box_array, &mut label_map)?;
    }
    let result = box_array
        .iter()
        .enumerate()
        .filter(|(_, box_map)| !box_map.is_empty())
        .fold(0, |acc_1, (box_num, box_map)| {
            let box_count = box_map
                .values()
                .enumerate()
                .fold(0, |acc_2, (slot, f_length)| {
                    acc_2 + ((box_num + 1) * (slot + 1) * *f_length as usize)
                });
            info!("Box: {}, total = {}", box_num + 1, box_count);
            acc_1 + box_count
        });
    Ok(result as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() -> Result<()> {
        // pretty_env_logger::init();
        let input_1 = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(process_part_1(input_1)?, 1320);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        pretty_env_logger::init();
        let input_1 = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(process_part_2(input_1)?, 145);
        Ok(())
    }
}
