use anyhow::Result;
use thiserror::Error;

const AOC_DAY: &str = "11";

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Unable to parse the input `{0}`")]
    ParsingError(String),
    #[error("An unknown error has occurred (super duper helpful error)")]
    Unknown,
}

fn main() {
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

fn parse_line(line: &str) -> Option<(Vec<char>, Vec<i64>)> {
    let (left, right) = line.split_once(' ')?;
    let gears = left.trim().chars().collect();
    let numbers = right
        .trim()
        .split(',')
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    Some((gears, numbers))
}

fn get_count(
    memo: &mut Vec<Vec<Vec<i64>>>,
    line: &[char],
    index: usize,
    prev: bool,
    count: i64,
    numbers: &[i64],
    num_index: usize,
) -> i64 {
    if index == line.len() {
        if num_index == numbers.len()
            || (num_index == numbers.len() - 1 && count == numbers[num_index])
        {
            return 1;
        }
        return 0;
    }
    if num_index < numbers.len() && (count > numbers[num_index]) {
        return 0;
    }
    if memo[index][num_index][count as usize] != -1 {
        return memo[index][num_index][count as usize];
    }
    let mut result = 0;
    if line[index] != '.' && num_index < numbers.len() {
        result = get_count(memo, line, index + 1, true, count + 1, numbers, num_index);
    }
    if line[index] != '#' && !(prev && num_index < numbers.len() && numbers[num_index] != count) {
        let mut next_index = num_index;
        if prev {
            next_index += 1;
        }
        result += get_count(memo, line, index + 1, false, 0, numbers, next_index);
    }
    memo[index][num_index][count as usize] = result;

    result
}

fn unfold(gears: &[char], numbers: &[i64]) -> (Vec<char>, Vec<i64>) {
    let mut ex_gears: Vec<char> = Vec::new();
    let mut ex_numbers: Vec<i64> = Vec::new();
    for _ in 0..5 {
        ex_gears.extend(gears.iter());
        ex_gears.push('?');
        ex_numbers.extend(numbers.iter());
    }
    ex_gears.pop();

    (ex_gears, ex_numbers)
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let mut result = 0;
    for line in input.lines() {
        let (gears, numbers) = match parse_line(line) {
            Some(x) => x,
            _ => return Err(AoCError::ParsingError(line.to_string())),
        };
        let mut memo: Vec<Vec<Vec<i64>>> =
            vec![vec![vec![-1; gears.len() + 1]; numbers.len() + 1]; gears.len() + 1];
        result += get_count(&mut memo, &gears, 0, false, 0, &numbers, 0);
    }
    Ok(result)
}

fn process_part_2(input: &str) -> Result<i64, AoCError> {
    let mut result = 0;
    for line in input.lines() {
        let (gears, numbers) = match parse_line(line) {
            Some(x) => x,
            _ => return Err(AoCError::ParsingError(line.to_string())),
        };
        let (ex_gears, ex_numbers) = unfold(&gears, &numbers);
        let mut memo: Vec<Vec<Vec<i64>>> =
            vec![vec![vec![-1; ex_gears.len() + 1]; ex_numbers.len() + 1]; ex_gears.len() + 1];
        result += get_count(&mut memo, &ex_gears, 0, false, 0, &ex_numbers, 0);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_0() -> Result<()> {
        let input_1 = vec!['?', '#', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?'];
        let nums_1 = vec![3, 2, 1];
        let mut memo: Vec<Vec<Vec<i64>>> =
            vec![vec![vec![-1; input_1.len() + 1]; nums_1.len() + 1]; input_1.len() + 1];
        assert_eq!(get_count(&mut memo, &input_1, 0, false, 0, &nums_1, 0), 10);
        let input_2 = vec![
            '.', '?', '?', '.', '.', '?', '?', '.', '.', '.', '?', '#', '#', '.',
        ];
        let nums_2 = vec![1, 1, 3];
        let mut memo: Vec<Vec<Vec<i64>>> =
            vec![vec![vec![-1; input_2.len() + 1]; nums_2.len() + 1]; input_2.len() + 1];
        assert_eq!(get_count(&mut memo, &input_2, 0, false, 0, &nums_2, 0), 4);
        Ok(())
    }
    #[test]
    fn part_1() -> Result<()> {
        let input_1 = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(process_part_1(input_1)?, 21);
        Ok(())
    }
    #[test]
    fn part_1_5() -> Result<()> {
        let input_1 = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";
        assert_eq!(process_part_1(input_1)?, 1);
        Ok(())
    }
    #[test]
    fn part_2() -> Result<()> {
        let input_1 = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(process_part_2(input_1)?, 525152);
        Ok(())
    }
}
