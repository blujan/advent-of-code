pub const NORTH: usize = 1;
pub const EAST: usize = 2;
pub const SOUTH: usize = 4;
pub const WEST: usize = 8;

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

    pub fn get(&self, x: usize, y: usize) -> char {
        let index = self.get_index(x, y);
        self.data[index] as char
    }
    pub fn set(&mut self, x: usize, y: usize, value: char) {
        let index = self.get_index(x, y);
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
        }
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    col: usize,
    row: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (char, usize, usize); // (block, row, col)
    fn next(&mut self) -> Option<(char, usize, usize)> {
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

#[derive(Debug)]
pub struct GridPos {
    pub row: usize,
    pub col: usize,
    pub dir: usize,
}

impl GridPos {
    pub fn next(self, grid: &Grid) -> Vec<GridPos> {
        let mut result: Vec<GridPos> = Vec::new();
        match grid.get(self.col, self.row) {
            '.' => match self.dir {
                NORTH => {
                    if self.row > 0 {
                        result.push(GridPos {
                            row: self.row - 1,
                            col: self.col,
                            dir: self.dir,
                        })
                    }
                }
                WEST => {
                    if self.col > 0 {
                        result.push(GridPos {
                            row: self.row,
                            col: self.col - 1,
                            dir: self.dir,
                        })
                    }
                }
                SOUTH => {
                    if self.row + 1 < grid.height {
                        result.push(GridPos {
                            row: self.row + 1,
                            col: self.col,
                            dir: self.dir,
                        })
                    }
                }
                EAST => {
                    if self.col + 1 < grid.width {
                        result.push(GridPos {
                            row: self.row,
                            col: self.col + 1,
                            dir: self.dir,
                        })
                    }
                }
                _ => panic!(),
            },
            '/' => match self.dir {
                NORTH => {
                    if self.col + 1 < grid.width {
                        result.push(GridPos {
                            row: self.row,
                            col: self.col + 1,
                            dir: EAST,
                        })
                    }
                }
                WEST => {
                    if self.row + 1 < grid.height {
                        result.push(GridPos {
                            row: self.row + 1,
                            col: self.col,
                            dir: SOUTH,
                        })
                    }
                }
                SOUTH => {
                    if self.col > 0 {
                        result.push(GridPos {
                            row: self.row,
                            col: self.col - 1,
                            dir: WEST,
                        })
                    }
                }
                EAST => {
                    if self.row > 0 {
                        result.push(GridPos {
                            row: self.row - 1,
                            col: self.col,
                            dir: NORTH,
                        })
                    }
                }
                _ => panic!(),
            },
            '\\' => match self.dir {
                NORTH => {
                    if self.col > 0 {
                        result.push(GridPos {
                            row: self.row,
                            col: self.col - 1,
                            dir: WEST,
                        })
                    }
                }
                WEST => {
                    if self.row > 0 {
                        result.push(GridPos {
                            row: self.row - 1,
                            col: self.col,
                            dir: NORTH,
                        })
                    }
                }
                SOUTH => {
                    if self.col + 1 < grid.width {
                        result.push(GridPos {
                            row: self.row,
                            col: self.col + 1,
                            dir: EAST,
                        })
                    }
                }
                EAST => {
                    if self.row + 1 < grid.height {
                        result.push(GridPos {
                            row: self.row + 1,
                            col: self.col,
                            dir: SOUTH,
                        })
                    }
                }
                _ => panic!(),
            },
            '|' => match self.dir {
                NORTH => {
                    if self.row > 0 {
                        result.push(GridPos {
                            row: self.row - 1,
                            col: self.col,
                            dir: self.dir,
                        })
                    }
                }
                EAST | WEST => {
                    if self.row > 0 {
                        result.push(GridPos {
                            row: self.row - 1,
                            col: self.col,
                            dir: NORTH,
                        })
                    };
                    if self.row + 1 < grid.height {
                        result.push(GridPos {
                            row: self.row + 1,
                            col: self.col,
                            dir: SOUTH,
                        })
                    };
                }
                SOUTH => {
                    if self.row + 1 < grid.height {
                        result.push(GridPos {
                            row: self.row + 1,
                            col: self.col,
                            dir: self.dir,
                        })
                    }
                }
                _ => panic!(),
            },
            '-' => match self.dir {
                NORTH | SOUTH => {
                    if self.col > 0 {
                        result.push(GridPos {
                            row: self.row,
                            col: self.col - 1,
                            dir: WEST,
                        })
                    };
                    if self.col + 1 < grid.width {
                        result.push(GridPos {
                            row: self.row,
                            col: self.col + 1,
                            dir: EAST,
                        })
                    };
                }
                WEST => {
                    if self.col > 0 {
                        result.push(GridPos {
                            row: self.row,
                            col: self.col - 1,
                            dir: self.dir,
                        })
                    }
                }
                EAST => {
                    if self.col + 1 < grid.width {
                        result.push(GridPos {
                            row: self.row,
                            col: self.col + 1,
                            dir: self.dir,
                        })
                    }
                }
                _ => panic!(),
            },
            _ => panic!("Unknown block @ {}, {}", self.col, self.row),
        }
        result
    }
}
