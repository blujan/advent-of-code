#[derive(Copy, Clone)]
pub enum Direction {
    North,
    West,
    South,
    East,
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

    fn get_index(&self, x: usize, y: usize, dir: Direction) -> usize {
        let (n_x, n_y) = match dir {
            Direction::North => (x, y),
            Direction::South => (self.width - x - 1, self.height - y - 1),
            Direction::West => (y, self.height - x - 1),
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

impl<'a> IntoIterator for &'a Grid {
    type Item = (char, usize, usize);
    type IntoIter = GridIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            col: 0,
            row: 0,
            dir: Direction::North,
        }
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    col: usize,
    row: usize,
    dir: Direction,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (char, usize, usize); // (block, row, col)
    fn next(&mut self) -> Option<(char, usize, usize)> {
        let (row_size, col_size) = match self.dir {
            Direction::North | Direction::South => (self.grid.height, self.grid.width),
            Direction::East | Direction::West => (self.grid.width, self.grid.height),
        };
        if self.col >= col_size || self.row >= row_size {
            return None;
        }
        let block = self.grid.get(self.col, self.row, self.dir);
        let result = (block, self.row, self.col);
        self.col += 1;
        if self.col == col_size {
            self.col = 0;
            self.row += 1;
        }
        Some(result)
    }
}
