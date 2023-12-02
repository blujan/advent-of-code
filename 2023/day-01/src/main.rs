use anyhow::Result;
use day_01::Trie;
use std::fmt::Error;

struct Word {
    n: &'static str,
    v: char,
}

const NUMBERS: [Word; 9] = [
    Word { n: "one", v: '1' },
    Word { n: "two", v: '2' },
    Word { n: "three", v: '3' },
    Word { n: "four", v: '4' },
    Word { n: "five", v: '5' },
    Word { n: "six", v: '6' },
    Word { n: "seven", v: '7' },
    Word { n: "eight", v: '8' },
    Word { n: "nine", v: '9' },
];

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: i32 = input
        .split('\n')
        .map(|line| get_digits_part_1(line).unwrap())
        .map(|line| line.parse::<i32>().unwrap())
        .sum();
    println!("Part 1 result: {}", result);

    // Part 2
    let mut trie = setup_trie();
    let result_2: i32 = input
        .split('\n')
        .map(|line| get_digits_part_2(line, &mut trie).unwrap())
        .map(|line| line.parse::<i32>().unwrap())
        .sum();
    println!("Part 2 result: {}", result_2);
}

fn get_digits_part_1(input: &str) -> Result<String, Error> {
    let mut ans: String = Default::default();
    for c in input.chars() {
        if c.is_numeric() {
            ans.push(c);
            break;
        }
    }
    for c in input.chars().rev() {
        if c.is_numeric() {
            ans.push(c);
            break;
        }
    }
    if ans.len() != 2 {
        return Err(Error);
    }
    Ok(ans)
}

fn setup_trie() -> Trie {
    let mut trie = Trie::default();
    for num in NUMBERS.iter() {
        trie.insert(num.n, num.v);
    }
    trie
}

fn get_digits_part_2(input: &str, trie: &mut Trie) -> Result<String, Error> {
    let mut ans: String = Default::default();
    for (index, _c) in input.char_indices() {
        let s = &input[index..];
        let (contains, c) = trie.contains(s);
        if contains {
            if ans.is_empty() {
                ans.push(c);
                ans.push(c);
            } else {
                ans.pop();
                ans.push(c);
            }
        }
    }
    if ans.len() != 2 {
        return Err(Error);
    }
    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_digits_pt_1() -> Result<()> {
        assert_eq!(get_digits_part_1("1abc2").unwrap(), "12".to_string());
        assert_eq!(get_digits_part_1("pqr3stu8vwx").unwrap(), "38".to_string());
        assert_eq!(get_digits_part_1("a1b2c3d4e5f").unwrap(), "15".to_string());
        assert_eq!(get_digits_part_1("treb7uchet").unwrap(), "77".to_string());
        Ok(())
    }
    #[test]
    fn test_get_digits_pt_2() -> Result<()> {
        let mut trie = setup_trie();
        assert_eq!(
            get_digits_part_2("two1nine", &mut trie).unwrap(),
            "29".to_string()
        );
        assert_eq!(
            get_digits_part_2("eightwothree", &mut trie).unwrap(),
            "83".to_string()
        );
        assert_eq!(
            get_digits_part_2("abcone2threexyz", &mut trie).unwrap(),
            "13".to_string()
        );
        assert_eq!(
            get_digits_part_2("xtwone3four", &mut trie).unwrap(),
            "24".to_string()
        );
        assert_eq!(
            get_digits_part_2("4nineeightseven2", &mut trie).unwrap(),
            "42".to_string()
        );
        assert_eq!(
            get_digits_part_2("zoneight234", &mut trie).unwrap(),
            "14".to_string()
        );
        assert_eq!(
            get_digits_part_2("7pqrstsixteen", &mut trie).unwrap(),
            "76".to_string()
        );
        Ok(())
    }
}
