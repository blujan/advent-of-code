use anyhow::Result;
use day_17::{left_turn, right_turn, Grid, GridPos, EAST, MOVES, SOUTH};
use std::collections::BinaryHeap;
use thiserror::Error;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

const AOC_DAY: &str = "17";

//
// A* might perform better here on average, but I think Dijkstra's might just fine in this scenario/scale
//

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

fn get_next_pos(grid: &Grid, pos: &GridPos, dir: usize) -> Option<GridPos> {
    let next_col = pos.col as i32 + MOVES[dir][0];
    let next_row = pos.row as i32 + MOVES[dir][1];
    if next_row >= 0
        && next_col >= 0
        && next_row < grid.height as i32
        && next_col < grid.width as i32
    {
        let next_step = if dir == pos.dir { pos.steps + 1 } else { 1 };
        let next_pos = GridPos {
            row: next_row as usize,
            col: next_col as usize,
            dir,
            cost: pos.cost + grid.get(next_col as usize, next_row as usize) as usize,
            steps: next_step,
        };
        return Some(next_pos);
    }
    None
}

fn get_min_loss(
    grid: &Grid,
    start: (usize, usize),
    dest: (usize, usize),
    min_steps: usize,
    max_steps: usize,
) -> Option<usize> {
    let mut dists: Vec<Vec<Vec<Vec<usize>>>> =
        vec![vec![vec![vec![std::usize::MAX; grid.width]; grid.height]; max_steps]; 4];
    let mut heap = BinaryHeap::new();
    for dist in dists.iter_mut().take(4) {
        for step in dist.iter_mut().take(max_steps) {
            step[start.0][start.1] = 0;
        }
    }
    heap.push(GridPos {
        row: start.0,
        col: start.1,
        dir: EAST,
        cost: 0,
        steps: 0,
    });
    heap.push(GridPos {
        row: start.0,
        col: start.1,
        dir: SOUTH,
        cost: 0,
        steps: 0,
    });
    while let Some(current_pos) = heap.pop() {
        if current_pos.row == dest.0 && current_pos.col == dest.1 && current_pos.steps >= min_steps
        {
            trace!(
                "FOUND {}, {} - {} with cost {}",
                current_pos.row,
                current_pos.col,
                current_pos.steps,
                current_pos.cost
            );
            return Some(current_pos.cost);
        }
        if current_pos.steps > 0
            && dists[current_pos.dir][current_pos.steps - 1][current_pos.row][current_pos.col]
                < current_pos.cost
        {
            continue;
        }
        trace!(
            "@ {}, {} - {} with cost {}",
            current_pos.row,
            current_pos.col,
            current_pos.steps,
            current_pos.cost
        );
        if current_pos.steps < max_steps {
            // forward
            let next_pos = get_next_pos(grid, &current_pos, current_pos.dir);
            if let Some(pos) = next_pos {
                if pos.cost < dists[pos.dir][pos.steps - 1][pos.row][pos.col] {
                    trace!(
                        "  Adding {}, {} - {} with cost {}",
                        pos.row,
                        pos.col,
                        pos.steps,
                        pos.cost
                    );
                    dists[pos.dir][pos.steps - 1][pos.row][pos.col] = pos.cost;
                    heap.push(pos);
                }
            }
        }
        if current_pos.steps >= min_steps {
            if let Some(pos) = get_next_pos(grid, &current_pos, left_turn(current_pos.dir)) {
                if pos.cost < dists[pos.dir][pos.steps - 1][pos.row][pos.col] {
                    trace!(
                        "  Adding {}, {} - {} with cost {}",
                        pos.row,
                        pos.col,
                        pos.steps,
                        pos.cost
                    );
                    dists[pos.dir][pos.steps - 1][pos.row][pos.col] = pos.cost;
                    heap.push(pos);
                }
            }
            if let Some(pos) = get_next_pos(grid, &current_pos, right_turn(current_pos.dir)) {
                if pos.cost < dists[pos.dir][pos.steps - 1][pos.row][pos.col] {
                    trace!(
                        "  Adding {}, {} - {} with cost {}",
                        pos.row,
                        pos.col,
                        pos.steps,
                        pos.cost
                    );
                    dists[pos.dir][pos.steps - 1][pos.row][pos.col] = pos.cost;
                    heap.push(pos);
                }
            }
        }
    }
    None
}

fn process_part_1(input: &str) -> Result<usize, AoCError> {
    let grid = Grid::new(input);
    let dists = match get_min_loss(&grid, (0, 0), (grid.height - 1, grid.width - 1), 1, 3) {
        Some(x) => x,
        _ => return Err(AoCError::Unknown),
    };
    Ok(dists)
}

fn process_part_2(input: &str) -> Result<usize, AoCError> {
    let grid = Grid::new(input);
    let dists = match get_min_loss(&grid, (0, 0), (grid.height - 1, grid.width - 1), 4, 10) {
        Some(x) => x,
        _ => return Err(AoCError::Unknown),
    };
    Ok(dists)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() -> Result<()> {
        let _ = pretty_env_logger::try_init();
        let input_1 = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(process_part_1(input_1)?, 102);
        Ok(())
    }
    #[test]
    fn part_2_1() -> Result<()> {
        let _ = pretty_env_logger::try_init();
        let input_1 = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(process_part_2(input_1)?, 94);
        Ok(())
    }
    #[test]
    fn part_2_2() -> Result<()> {
        let _ = pretty_env_logger::try_init();
        let input_1 = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(process_part_2(input_1)?, 71);
        Ok(())
    }
}

// 831, 845 = too low
// 880 = too high
