use anyhow::Result;
use day_16::{Grid, GridPos, EAST, NORTH, SOUTH, WEST};
use std::collections::VecDeque;
use thiserror::Error;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

const AOC_DAY: &str = "16";

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

fn is_visited(pos: &GridPos, visited: &[Vec<usize>]) -> bool {
    (visited[pos.row][pos.col] & pos.dir) == pos.dir
}

fn find_energized_tiles(grid: &Grid, initial_pos: GridPos) -> Result<usize, AoCError> {
    let mut visited: Vec<Vec<usize>> = vec![vec![0; grid.width]; grid.height];
    let mut queue: VecDeque<GridPos> = VecDeque::new();
    visited[initial_pos.row][initial_pos.col] = initial_pos.dir;
    queue.push_back(initial_pos);
    while !queue.is_empty() {
        let current_pos = queue.pop_front().unwrap();
        trace!(
            "At {}, {} ({})",
            current_pos.row,
            current_pos.col,
            grid.get(current_pos.col, current_pos.row)
        );
        for next_pos in current_pos.next(grid) {
            if !is_visited(&next_pos, &visited) {
                visited[next_pos.row][next_pos.col] |= next_pos.dir;
                queue.push_back(next_pos);
            }
        }
    }
    let result = visited.iter().flatten().filter(|val| **val != 0).count();
    Ok(result)
}

fn process_part_1(input: &str) -> Result<usize, AoCError> {
    let grid = Grid::new(input);
    let result = find_energized_tiles(
        &grid,
        GridPos {
            row: 0,
            col: 0,
            dir: EAST,
        },
    )?;
    Ok(result)
}

fn process_part_2(input: &str) -> Result<usize, AoCError> {
    let grid = Grid::new(input);
    let mut result = 0;

    // Top & bottom
    for pos in 0..grid.width {
        let top_result = find_energized_tiles(
            &grid,
            GridPos {
                row: 0,
                col: pos,
                dir: SOUTH,
            },
        )?;
        let bot_result = find_energized_tiles(
            &grid,
            GridPos {
                row: grid.height - 1,
                col: pos,
                dir: NORTH,
            },
        )?;
        result = result.max(top_result);
        result = result.max(bot_result);
    }
    // Left & Right
    for pos in 0..grid.height {
        let left_result = find_energized_tiles(
            &grid,
            GridPos {
                row: pos,
                col: 0,
                dir: EAST,
            },
        )?;
        let right_result = find_energized_tiles(
            &grid,
            GridPos {
                row: pos,
                col: grid.width - 1,
                dir: WEST,
            },
        )?;
        result = result.max(left_result);
        result = result.max(right_result);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() -> Result<()> {
        let input_1 = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(process_part_1(input_1)?, 46);
        Ok(())
    }
    #[test]
    fn part_2() -> Result<()> {
        let input_1 = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(process_part_2(input_1)?, 51);
        Ok(())
    }
}
