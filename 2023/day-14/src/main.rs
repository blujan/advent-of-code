use anyhow::Result;
use thiserror::Error;
//use fxhash::FxBuildHasher;
use std::collections::HashMap;

const AOC_DAY: &str = "14";

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Unable to parse the input `{0}`")]
    ParsingError(String),
    #[error("An unknown error has occurred (super duper helpful error)")]
    Unknown,
}

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Grid {
    pub fn new(data: &str) -> Grid {
        let height = data.lines().count();
        let width = data.lines().next().unwrap().chars().count();
        Grid {
            data: data.as_bytes().to_vec(),
            width,
            height,
        }
    }

    fn get_index(&self, x: usize, y: usize, dir: Direction) -> usize {
        let (n_x, n_y) = match dir {
            Direction::North => (x, y),
            Direction::South => (self.width - x - 1, self.height - y - 1),
            Direction::West => (y ,self.height - x - 1),
            Direction::East => (self.width - y - 1, x),
        };
        (n_y * (self.width + 1)) + n_x
    }

    pub fn get(&self, x: usize, y: usize, dir: Direction) -> char {
        let index = self.get_index(x, y, dir);
        self.data[index] as char
    }
    pub fn set(&mut self, x: usize, y: usize, dir: Direction, value: char) {
        let index = self.get_index(x, y, dir);
        self.data[index] = value as u8;
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
    match process_part_2(INPUT, 1000000000) {
        Ok(result) => println!("Part 2 result\n\t{}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn process_line(line: &str, row: usize, rock_count: &mut [i64], pos: &mut [usize]) {
    for (col, block) in line.chars().enumerate() {
        if block == '#' {
            pos[col] = row + 1;
        } else if block == 'O' {
            rock_count[pos[col]] += 1;
            pos[col] += 1;
        }
    }
}

fn process_part_1(input: &str) -> Result<i64, AoCError> {
    let size = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut rock_count: Vec<i64> = vec![0; size];
    let mut pos: Vec<usize> = vec![0; width];
    input.lines().enumerate().for_each(|(row, line)| {
        process_line(line, row, &mut rock_count, &mut pos);
    });
    let result: i64 = rock_count
        .iter()
        .enumerate()
        .map(|(row, &count)| count * (size - row) as i64)
        .sum();
    Ok(result)
}

fn tilt (grid: &mut Grid, dir: Direction) {
    let (y_size, x_size) = match dir {
        Direction::North | Direction::South => (grid.height, grid.width),
        Direction::East | Direction::West => (grid.width, grid.height),
    };
    let mut pos: Vec<usize> = vec![0; x_size];
    for row in 0..y_size {
        for col in 0..x_size {
            let block = grid.get(col, row, dir);
            if block == '#' {
                pos[col] = row + 1;
            } else if block == 'O' {
                grid.set(col, row, dir, '.');
                grid.set(col, pos[col], dir, 'O');
                pos[col] += 1;
            }
        }
    }
}

fn spin(grid: &mut Grid) {
    tilt(grid, Direction::North);
    tilt(grid, Direction::West);
    tilt(grid, Direction::South);
    tilt(grid, Direction::East);
}

fn get_cycle_size(grid: &mut Grid, spins: usize, state_cache: &mut HashMap<Vec<u8>, usize>) -> (usize, usize) {
    let mut lead_up = 0;
    let mut cycle_size = 0;
    for spin_num in 0..spins {
        spin(grid);
        if let Some(&x) = state_cache.get(&grid.data) {
            println!("At {}. State seen previously at iteration {}", spin_num, x);
            cycle_size = spin_num - x;
            lead_up = x;
            break;
        }
        state_cache.insert(grid.data.clone(), spin_num);
    }
    (cycle_size, lead_up)
}

fn get_score(grid: &Grid) -> i64 {
    let mut count = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.get(col, row, Direction::North) == 'O' {
                count += (grid.height - row) as i64;
            }
        }
    }
    count
}

fn process_part_2(input: &str, spins: usize) -> Result<i64, AoCError> {
    // Need to find the fundamental frequency of the shifting cycles
    // i.e., the number of cycles it takes until it "resets" and repeats.
    // Then, take that value and use it to modulo the 1B number
    // & restore the target grid from the cache
    let mut grid = Grid::new(input);
    let mut state_cache: HashMap<Vec<u8>, usize> = HashMap::new();
    let (cycle_size, lead_up) = get_cycle_size(&mut grid, spins, &mut state_cache);
    let target_cycle = (((spins - lead_up) % cycle_size) + lead_up) - 1;
    // println!("Cycle size: {}, lead up: {}", cycle_size, lead_up);
    // println!("Target cycle is: {}", target_cycle);
    for (key, _) in state_cache.drain().filter(|(_, value)| *value == target_cycle) {
        println!("Restoring from state cache..");
        grid.data = key;
    }
    // for y in 0..grid.height {
    //     for x in 0..grid.width {
    //         print!("{}", grid.get(x, y, Direction::North));
    //     }
    //     println!();
    // }
    // println!();
    let score = get_score(&grid);
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn par1_1() -> Result<()> {
        let input_1 = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(process_part_1(input_1)?, 136);
        Ok(())
    }
    #[test]
    fn part2() -> Result<()> {
        let input_1 =
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(process_part_2(input_1, 1000000000)?, 64);
        Ok(())
    }
}
