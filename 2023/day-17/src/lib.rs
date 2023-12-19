use std::char::from_digit;
use std::cmp::Ordering;

pub const NORTH: usize = 0;
pub const EAST: usize = 1;
pub const SOUTH: usize = 2;
pub const WEST: usize = 3;

pub const MOVES: [[i32; 2]; 4] = [
    // N - E - S - W
    [0, -1],
    [1, 0],
    [0, 1],
    [-1, 0],
];

pub fn left_turn(dir: usize) -> usize {
    if dir == 0 {
        return 3;
    }
    dir - 1
}

pub fn right_turn(dir: usize) -> usize {
    if dir == 3 {
        return 0;
    }
    dir + 1
}

pub struct Grid {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
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
    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * (self.width + 1)) + x
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        let index = self.get_index(x, y);
        let number_char = self.data[index] as char;
        number_char.to_digit(10).unwrap()
    }
    pub fn set(&mut self, x: usize, y: usize, value: u32) {
        let index = self.get_index(x, y);
        self.data[index] = from_digit(value, 10).unwrap() as u8
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (u32, usize, usize);
    type IntoIter = GridIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            col: 0,
            row: 0,
        }
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    col: usize,
    row: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (u32, usize, usize); // (block, row, col)
    fn next(&mut self) -> Option<(u32, usize, usize)> {
        if self.col >= self.grid.width || self.row >= self.grid.height {
            return None;
        }
        let block = self.grid.get(self.col, self.row);
        let result = (block, self.row, self.col);
        self.col += 1;
        if self.col == self.grid.width {
            self.col = 0;
            self.row += 1;
        }
        Some(result)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct GridPos {
    pub row: usize,
    pub col: usize,
    pub dir: usize,
    pub cost: usize,
    pub steps: usize,
}

impl Ord for GridPos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for GridPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
