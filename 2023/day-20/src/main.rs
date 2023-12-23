use anyhow::Result;
use day_20::{Logic, LogicType, Pulse};
use log::{log_enabled, Level};
use queue::Queue;
use std::collections::HashMap;
use thiserror::Error;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

const AOC_DAY: &str = "20";

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Unable to parse the input `{0}`")]
    ParsingError(String),
    #[error("An unknown error has occurred (super duper helpful error)")]
    Unknown,
}

fn main() {
    let _ = pretty_env_logger::try_init();
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

fn get_token<'a>(
    input: &'a str,
    token_map: &mut HashMap<&'a str, usize>,
    count: &mut usize,
) -> usize {
    let result = token_map.entry(input).or_insert_with(|| {
        let token = *count;
        *count += 1;
        token
    });
    *result
}

fn print_inventory(inventory: &[Logic], token_map: &HashMap<&str, usize>) {
    if !log_enabled!(Level::Trace) {
        return;
    }
    let mut print_map = HashMap::new();
    for (k, v) in token_map {
        print_map.insert(v, k);
    }
    inventory.iter().enumerate().for_each(|(index, block)| {
        if let Some(label) = print_map.get(&index) {
            trace!("{} -> {:?}", label, block);
            if !block.dest.is_empty() {
                trace!(" -> ( ");
                for dest in block.dest.iter() {
                    if let Some(dest_label) = print_map.get(&dest) {
                        print!("{} ", dest_label);
                    }
                }
                trace!(")");
            }
            trace!("");
        }
    });
}

fn get_logic<'a>(
    input: &'a str,
    token_map: &mut HashMap<&'a str, usize>,
    count: &mut usize,
) -> Result<(usize, Logic), AoCError> {
    let (left, right) = match input.split_once("->") {
        Some(x) => x,
        _ => return Err(AoCError::ParsingError(input.to_string())),
    };
    let mut label = left.trim();
    let destinations = right
        .trim()
        .split(',')
        .map(|dest| dest.trim())
        .map(|dest| get_token(dest, token_map, count))
        .collect::<Vec<_>>();
    let logic_type = match label.chars().next().unwrap() {
        '%' => LogicType::FlipFlop,
        '&' => LogicType::Conjunction,
        _ => LogicType::Broadcaster,
    };
    if label.starts_with('%') || label.starts_with('&') {
        label = &label[1..];
    }
    let label_token = get_token(label, token_map, count);
    let logic = Logic::new(logic_type, destinations);
    Ok((label_token, logic))
}

fn connect_conj(inventory: Vec<Logic>) -> Vec<Logic> {
    let mut update = inventory.clone();
    for (index, block) in inventory.iter().enumerate() {
        for dest in block.dest.iter() {
            update[*dest].add_input(index, inventory.len());
        }
    }
    update
}

fn get_inventory(input: &str) -> Result<(Vec<Logic>, HashMap<&str, usize>), AoCError> {
    let mut count = 0;
    let mut token_map: HashMap<&str, usize> = HashMap::new();
    let inventory_pre = input
        .lines()
        .map(|line| get_logic(line, &mut token_map, &mut count))
        .collect::<Result<Vec<_>, _>>()?;
    let mut inventory = vec![Logic::blank(); count];
    for (index, block) in inventory_pre {
        inventory[index] = block;
    }
    inventory = connect_conj(inventory);
    Ok((inventory, token_map))
}

fn run_sim(
    inv: &mut [Logic],
    watch_list: &mut [(Pulse, &str, usize)],
    start: usize,
    count: usize,
) -> (i64, usize) {
    let mut states = HashMap::new();
    let mut queue: Queue<(usize, usize, Pulse)> = Queue::new();
    let mut cycle = 0;
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    for push in 0..count {
        if let Some(x) = states.insert(inv.to_owned(), push) {
            println!("found loop at from {} <-> {}", x, push);
            cycle = push - x;
            break;
        }
        queue.queue((start, 0, Pulse::Low)).unwrap();
        let mut cycle = 0;
        while !queue.is_empty() {
            let (target, source, pulse) = queue.dequeue().unwrap();
            if pulse == Pulse::High {
                high_pulse_count += 1;
            } else {
                low_pulse_count += 1;
            }
            if watch_list[target].0 == pulse {
                watch_list[target].2 = push + 1;
                trace!(
                    "At {}, pulse {:?}, push, {}, cycle {}, source {}",
                    watch_list[target].1,
                    pulse,
                    push + 1,
                    cycle,
                    source
                );
            }
            inv[target].set(pulse, source);
            if let Some(next_pulse) = inv[target].get() {
                for dest in inv[target].dest.iter() {
                    queue.queue((*dest, target, next_pulse)).unwrap();
                }
            }
            cycle += 1;
        }
    }
    trace!(
        "Result - low: {}, high: {}",
        low_pulse_count,
        high_pulse_count
    );
    (low_pulse_count * high_pulse_count, cycle)
}

fn get_watch_list<'a>(
    labels: &'a [(&'a str, Pulse)],
    token_map: &HashMap<&'a str, usize>,
    size: usize,
) -> Vec<(Pulse, &'a str, usize)> {
    let mut watch_list = Vec::new();
    watch_list.resize(size, (Pulse::Z, "", 0));
    for (label, pulse) in labels {
        if let Some(index) = token_map.get(label) {
            watch_list[*index] = (*pulse, *label, 0);
        }
    }
    watch_list
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let (mut inventory, token_map) = get_inventory(input)?;
    print_inventory(&inventory, &token_map);
    let start = match token_map.get("broadcaster") {
        Some(x) => *x,
        _ => panic!("unable to find broadcaster block"),
    };
    let mut watch_list = get_watch_list(&[], &token_map, inventory.len());
    let sim_result = run_sim(&mut inventory, &mut watch_list, start, 1000);
    let result = if sim_result.1 > 0 {
        let count = 1000 / sim_result.1;
        sim_result.0 * count as i64 * count as i64
    } else {
        sim_result.0
    };
    Ok(result)
}

fn get_parts(inv: &Vec<Logic>, start: usize) -> Vec<Vec<Logic>> {
    let mut sub_trees: Vec<Vec<Logic>> = Vec::new();
    for dest in inv[start].dest.iter() {
        let mut visited = vec![false; inv.len()];
        let mut tree: Vec<usize> = Vec::new();
        let mut q = Queue::new();
        visited[*dest] = true;
        q.queue(*dest).unwrap();
        while let Some(current) = q.dequeue() {
            tree.push(current);
            for next in inv[current].dest.iter() {
                if !visited[*next] {
                    visited[*next] = true;
                    q.queue(*next).unwrap();
                }
            }
        }
        let mut sub_tree = vec![Logic::blank(); inv.len()];
        for node in tree.iter() {
            sub_tree[*node] = inv[*node].clone();
        }
        sub_tree[start] = inv[start].clone();
        let remove = sub_tree
            .iter()
            .map(|logic| logic.kind == LogicType::Probe)
            .collect::<Vec<_>>();
        sub_tree
            .iter_mut()
            .for_each(|logic| logic.adj_input(&remove));
        sub_trees.push(sub_tree);
    }
    sub_trees
}

fn vec_lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    num::integer::lcm::<usize>(nums[0], vec_lcm(&nums[1..]))
}

fn process_part_2(input: &str) -> Result<usize, AoCError> {
    let (inventory, token_map) = get_inventory(input)?;
    let start = match token_map.get("broadcaster") {
        Some(x) => *x,
        _ => panic!("unable to find broadcaster block"),
    };
    let mut parts = get_parts(&inventory, start);
    let mut watch_list = get_watch_list(&[("rx", Pulse::Low)], &token_map, inventory.len());
    let watch_id = token_map.get("rx").unwrap();
    let mut cycles = Vec::new();
    for part in parts.iter_mut() {
        print_inventory(part, &token_map);
        trace!("\n\n");
        let _ = run_sim(part, &mut watch_list, start, 10000);
        cycles.push(watch_list[*watch_id].2);
    }
    Ok(vec_lcm(&cycles))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_0() -> Result<()> {
        let _ = pretty_env_logger::try_init();
        let input_1 = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(process_part_1(input_1)?, 32000000);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        let _ = pretty_env_logger::try_init();
        let input_1 = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(process_part_1(input_1)?, 11687500);
        Ok(())
    }
}
