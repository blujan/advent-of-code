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
    match process_part_1(INPUT, 2) {
        Ok(result) => println!("Part 1 result\n\t{}\n", result),
        Err(e) => println!("Error: {}", e),
    }
    match process_part_1(INPUT, 1000000) {
        Ok(result) => println!("Part 2 result\n\t{}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_zeros(grid: &Grid) -> Result<(Vec<usize>, Vec<usize>), AoCError> {
    let mut row_count = vec![0; grid.height];
    let mut col_count = vec![0; grid.width];
    row_count
        .iter_mut()
        .enumerate()
        .for_each(|(row_index, row)| {
            col_count
                .iter_mut()
                .enumerate()
                .for_each(|(col_index, col)| {
                    let block = match grid.get(col_index as i32, row_index as i32) {
                        Some(x) => x,
                        _ => panic!("invalid char in the grid!"),
                    };
                    if block == '#' {
                        *row += 1;
                        *col += 1;
                    }
                });
        });
    Ok((row_count, col_count))
}

// fn reconstruct_grid(grid: &Grid, row_count: &[usize], col_count: &[usize]) -> Vec<Vec<char>> {
//     let mut expanded_grid: Vec<Vec<char>> = Vec::new();
//     row_count.iter().enumerate().for_each(|(row, &row_num)| {
//         let mut next_row: Vec<char> = Vec::new();
//         col_count.iter().enumerate().for_each(|(col, &col_num)| {
//             next_row.push(grid.get(col as i32, row as i32).unwrap());
//             if col_num == 0 {
//                 next_row.push('.');
//             }
//         });
//         if row_num == 0 {
//             expanded_grid.push(next_row.clone());
//         }
//         expanded_grid.push(next_row);
//     });
//     expanded_grid
// }

fn get_points(grid: Grid) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    for row_index in 0..grid.height {
        for col_index in 0..grid.width {
            if grid.get(col_index as i32, row_index as i32).unwrap() == '#' {
                points.push((col_index, row_index));
            }
        }
    }
    points
}

fn manhattan_dist(a: (usize, usize), b: (usize, usize)) -> i64 {
    (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()
}

fn get_num_empty(row_count: &[usize], col_count: &[usize], a: (usize, usize), b: (usize, usize)) -> i64 {
    let left_col = a.0.min(b.0);
    let right_col = a.0.max(b.0);
    let top_row = a.1.min(b.1);
    let bot_row = a.1.max(b.1);
    let mut count = 0;
    for index in left_col..right_col {
        let cnt = match col_count.get(index) {
            Some(x) => *x,
            _=> panic!("Index out of range"),
        };
        if cnt == 0 {
            count += 1;
        }
    }
    for index in top_row..bot_row {
        let cnt = match row_count.get(index) {
            Some(x) => *x,
            _=> panic!("Index out of range"),
        };
        if cnt == 0 {
            count += 1;
        }
    }
    count
}

fn process_part_1(input: &str, expansion: i64) -> Result<i64, AoCError> {
    let grid = Grid::new(input);
    let (row_count, col_count) = get_zeros(&grid)?;
    let points = get_points(grid);
    let mut result = 0;

    for i in 0..points.len() {
        for x in i + 1..points.len() {
            result += manhattan_dist(points[i], points[x]);
            result += get_num_empty(&row_count, &col_count, points[i], points[x]) * (expansion - 1);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn par1_1() -> Result<()> {
        let input_1 = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(process_part_1(input_1, 2)?, 374);
        assert_eq!(process_part_1(input_1, 10)?, 1030);
        assert_eq!(process_part_1(input_1, 100)?, 8410);
        Ok(())
    }

}
