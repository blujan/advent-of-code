use anyhow::Result;
use thiserror::Error;

const AOC_DAY: &str = "13";

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Unable to parse the input `{0}`")]
    ParsingError(String),
    #[error("An unknown error has occurred (super duper helpful error)")]
    Unknown,
}

#[derive(Copy, Clone)]
struct Grid<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
}

impl Grid<'_> {
    pub fn new(data: &str) -> Grid {
        let height = data.lines().count();
        let width = data.lines().next().unwrap().chars().count();
        Grid {
            data: data.as_bytes(),
            width,
            height,
        }
    }
    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        if x >= (self.width as i32) || y >= (self.height as i32) || x < 0 || y < 0 {
            return None;
        }
        let index = ((y as usize) * (self.width + 1)) + (x as usize);
        Some(self.data[index] as char)
    }
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

fn cols_eq(grid: &Grid, a: i32, b: i32) -> (bool, i32) {
    let mut diff = 0;
    for index in 0..grid.height {
        let row = index as i32;
        let left = match grid.get(a, row) {
            Some(x) => x,
            _ => panic!(),
        };
        let right = match grid.get(b, row) {
            Some(x) => x,
            _ => panic!(),
        };
        if left != right {
            diff += 1;
        }
    }
    if diff > 0 {
        return (false, diff);
    }
    (true, diff)
}

fn check_cols_mirrored(grid: &Grid, a: i32, b: i32) -> bool {
    let mut left = a;
    let mut right = b;
    while left >= 0 && right < grid.width as i32 && cols_eq(grid, left, right).0 {
        left -= 1;
        right += 1;
    }
    if left == -1 || right == grid.width as i32 {
        return true;
    }
    false
}

fn rows_eq(grid: &Grid, a: i32, b: i32) -> (bool, i32) {
    let mut diff = 0;
    for index in 0..grid.width {
        let col = index as i32;
        let left = match grid.get(col, a) {
            Some(x) => x,
            _ => panic!(),
        };
        let right = match grid.get(col, b) {
            Some(x) => x,
            _ => panic!(),
        };
        if left != right {
            diff += 1;
        }
    }
    if diff > 0 {
        return (false, diff);
    }
    (true, diff)
}

fn check_rows_mirrored(grid: &Grid, a: i32, b: i32) -> bool {
    let mut top = a;
    let mut bot = b;
    while top >= 0 && bot < grid.height as i32 && rows_eq(grid, top, bot).0 {
        top -= 1;
        bot += 1;
    }
    if top == -1 || bot == grid.height as i32 {
        return true;
    }
    false
}

fn check_rows_mirrored_2(grid: &Grid, a: i32, b: i32) -> bool {
    let mut top = a;
    let mut bot = b;
    let mut smudge = 0;
    while top >= 0 && bot < grid.height as i32 && smudge < 2 {
        let result = rows_eq(grid, top, bot);
        if !result.0 {
            smudge += result.1;
        }
        top -= 1;
        bot += 1;
    }
    if (top == -1 || bot == grid.height as i32) && smudge == 1 {
        return true;
    }
    false
}

fn check_cols_mirrored_2(grid: &Grid, a: i32, b: i32) -> bool {
    let mut left = a;
    let mut right = b;
    let mut smudge = 0;
    while left >= 0 && right < grid.width as i32 && smudge < 2 {
        let result = cols_eq(grid, left, right);
        if !result.0 {
            smudge += result.1;
        }
        left -= 1;
        right += 1;
    }
    if (left == -1 || right == grid.width as i32) && smudge == 1 {
        return true;
    }
    false
}

fn check_mirrored(grid: &Grid, check: fn(&Grid, i32, i32) -> bool, limit: i32) -> Option<i32> {
    let mut top = 0;
    let mut bot = 1;
    while bot < limit {
        if check(grid, top, bot) {
            return Some(top + 1);
        }
        top += 1;
        bot += 1;
    }
    None
}

fn process_part_1(input: &str) -> Result<i32, AoCError> {
    let grids: Vec<Grid> = input.split("\n\n").map(|grid| Grid::new(grid)).collect();
    let mut result = 0;
    for grid in grids.iter() {
        // let col_mirror = match check_cols(grid) {
        let col_mirror = match check_mirrored(grid, check_cols_mirrored, grid.width as i32) {
            Some(x) => x,
            _ => 0,
        };
        let row_mirror = match check_mirrored(grid, check_rows_mirrored, grid.height as i32) {
            Some(x) => x,
            _ => 0,
        };
        result += col_mirror + (row_mirror * 100);
    }
    Ok(result)
}

fn process_part_2(input: &str) -> Result<i32, AoCError> {
    let grids: Vec<Grid> = input.split("\n\n").map(|grid| Grid::new(grid)).collect();
    let mut result = 0;
    for grid in grids.iter() {
        let col_mirror = match check_mirrored(grid, check_cols_mirrored_2, grid.width as i32) {
            Some(x) => x,
            _ => 0,
        };
        let row_mirror = match check_mirrored(grid, check_rows_mirrored_2, grid.height as i32) {
            Some(x) => x,
            _ => 0,
        };
        result += col_mirror + (row_mirror * 100);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() -> Result<()> {
        let input_1 = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(process_part_1(input_1)?, 405);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let input_1 = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(process_part_2(input_1)?, 400);
        Ok(())
    }
}
