use anyhow::Result;
use day_19::Condition;
use std::collections::HashMap;
use thiserror::Error;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

const AOC_DAY: &str = "19";

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

fn parse_flow(input: &str) -> Result<(&str, Vec<Condition>), AoCError> {
    let (key, conditions) = match input.split_once('{') {
        Some(x) => x,
        _ => return Err(AoCError::ParsingError(input.to_string())),
    };
    let mut command_cond: Vec<Condition> = Vec::new();
    for flow in conditions.trim_end_matches('}').split(',') {
        let split_cond = flow.split_once(':');
        let condition = if let Some((left, dest)) = split_cond {
            let delim = if left.contains('>') { '>' } else { '<' };
            let operator = if delim == '>' {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            };
            let (attr, value) = left.split_once(delim).unwrap();

            Condition {
                dest: dest.to_string(),
                attr: attr.chars().next().unwrap(),
                operator,
                value: value.parse::<i32>().unwrap(),
            }
        } else {
            Condition {
                dest: flow.to_string(),
                ..Default::default()
            }
        };
        command_cond.push(condition);
    }
    trace!("{} -> {:?}", key, command_cond);
    Ok((key, command_cond))
}

fn parse_parts(input: &str) -> Result<HashMap<char, i32>, AoCError> {
    let mut part_map = HashMap::new();
    for attr in input
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
    {
        let (label, value) = match attr.split_once('=') {
            Some(x) => x,
            _ => return Err(AoCError::ParsingError(input.to_string())),
        };
        part_map.insert(label.chars().next().unwrap(), value.parse::<i32>().unwrap());
    }
    Ok(part_map)
}

fn get_score(part: &HashMap<char, i32>, flow_map: &HashMap<&str, Vec<Condition>>) -> i64 {
    let mut current = "in".to_string();
    loop {
        if current == "A" {
            return part.values().sum::<i32>() as i64;
        } else if current == "R" {
            break;
        }
        let conditions = match flow_map.get(current.as_str()) {
            Some(x) => x,
            _ => panic!(),
        };
        for condition in conditions {
            if condition.operator == std::cmp::Ordering::Equal {
                current = condition.dest.clone();
                break;
            }
            let value = part.get(&condition.attr).unwrap();
            if value.cmp(&condition.value) == condition.operator {
                current = condition.dest.clone();
                break;
            }
        }
    }
    0
}

fn get_flow_map(input: &str) -> Result<HashMap<&str, Vec<Condition>>, AoCError> {
    let parsed_flows: Vec<_> = input
        .lines()
        .map(parse_flow)
        .collect::<Result<Vec<_>, _>>()?;
    let flow_map = HashMap::from_iter(parsed_flows);
    Ok(flow_map)
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let (flows, parts) = match input.split_once("\n\n") {
        Some(x) => x,
        _ => return Err(AoCError::ParsingError("".to_string())),
    };
    let flow_map = get_flow_map(flows)?;
    let part_map: Vec<_> = parts
        .lines()
        .map(parse_parts)
        .collect::<Result<Vec<_>, _>>()?;
    trace!("{:?}", part_map);
    let result = part_map.iter().map(|part| get_score(part, &flow_map)).sum();
    Ok(result)
}

fn get_range_map() -> HashMap<char, (i32, i32)> {
    let ranges: HashMap<char, (i32, i32)> = HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);
    ranges
}

fn get_part_count(
    flow_map: &HashMap<&str, Vec<Condition>>,
    part_ranges: &HashMap<char, (i32, i32)>,
    current: String,
) -> i64 {
    if current == "A" {
        let count = part_ranges
            .iter()
            .map(|(_, (min, max))| (*max - *min + 1) as i64)
            .filter(|count| *count > 0)
            .product();
        return count;
    }
    if current == "R" {
        return 0;
    }
    let mut count = 0;
    let mut mod_part_ranges = part_ranges.clone();
    let conditions = match flow_map.get(current.as_str()) {
        Some(x) => x,
        _ => panic!(),
    };
    for condition in conditions {
        if condition.operator == std::cmp::Ordering::Equal {
            count += get_part_count(flow_map, &mod_part_ranges, condition.dest.to_string());
            continue;
        }
        let orig = mod_part_ranges.get(&condition.attr).unwrap();
        let (next_a, next_b) = match condition.operator {
            std::cmp::Ordering::Less => ((orig.0, condition.value - 1), (condition.value, orig.1)),
            std::cmp::Ordering::Greater => {
                ((condition.value + 1, orig.1), (orig.0, condition.value))
            }
            _ => panic!(),
        };
        mod_part_ranges.insert(condition.attr, next_a);
        count += get_part_count(flow_map, &mod_part_ranges, condition.dest.to_string());
        mod_part_ranges.insert(condition.attr, next_b);
    }
    count
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    let (flows, _) = match input.split_once("\n\n") {
        Some(x) => x,
        _ => return Err(AoCError::ParsingError("".to_string())),
    };
    let flow_map = get_flow_map(flows)?;
    let part_ranges = get_range_map();
    let result = get_part_count(&flow_map, &part_ranges, "in".to_string());
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() -> Result<()> {
        let _ = pretty_env_logger::try_init();
        let input_1 = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(process_part_1(input_1)?, 19114);
        Ok(())
    }
    #[test]
    fn part_2() -> Result<()> {
        let _ = pretty_env_logger::try_init();
        let input_1 = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(process_part_2(input_1)?, 167409079868000);
        Ok(())
    }
}
