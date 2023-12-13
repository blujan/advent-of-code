use anyhow::Result;
use queue::Queue;
use thiserror::Error;

const AOC_DAY: &str = "10";

const MOVES: [[i32; 2]; 4] = [
    // N - E - S - W
    [0, -1],
    [1, 0],
    [0, 1],
    [-1, 0],
];

const CONNECT: [[char; 4]; 4] = [
    ['|', 'F', '7', 'S'],
    ['-', '7', 'J', 'S'],
    ['|', 'L', 'J', 'S'],
    ['-', 'L', 'F', 'S'],
];

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

#[derive(Copy, Clone, PartialEq, Eq)]
enum MoveDirection {
    Forward,
    Backward,
}

fn next_direction(current: char, prev: usize) -> Option<usize> {
    match current {
        '|' => match prev {
            NORTH => Some(SOUTH),
            SOUTH => Some(NORTH),
            _ => None,
        },
        '-' => match prev {
            WEST => Some(EAST),
            EAST => Some(WEST),
            _ => None,
        },
        'L' => match prev {
            NORTH => Some(EAST),
            EAST => Some(NORTH),
            _ => None,
        },
        'J' => match prev {
            NORTH => Some(WEST),
            WEST => Some(NORTH),
            _ => None,
        },
        '7' => match prev {
            WEST => Some(SOUTH),
            SOUTH => Some(WEST),
            _ => None,
        },
        'F' => match prev {
            EAST => Some(SOUTH),
            SOUTH => Some(EAST),
            _ => None,
        },
        _ => None,
    }
}

fn opposite_direction(dir: usize) -> usize {
    (dir + 2) % 4
}

#[derive(Copy, Clone)]
struct Maze<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
}

impl Maze<'_> {
    pub fn new(data: &str) -> Maze {
        let height = data.lines().count();
        let width = data.lines().next().unwrap().chars().count();
        Maze {
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

#[derive(Copy, Clone)]
struct MazePos<'a> {
    maze: &'a Maze<'a>,
    pos: (i32, i32),
    prev_pos: (i32, i32),
    start: (i32, i32),
    prev: usize,
    steps: usize,
    move_direction: MoveDirection,
}

impl MazePos<'_> {
    pub fn new<'a>(maze: &'a Maze<'a>, direction: MoveDirection) -> MazePos<'a> {
        let index = match maze.data.iter().position(|&x| x == b'S') {
            Some(x) => x as i32,
            _ => panic!("Unable to find start position!"),
        };
        let h = index / ((maze.width as i32) + 1);
        let w = index % ((maze.width as i32) + 1);
        MazePos {
            maze,
            steps: 0,
            pos: (w, h),
            prev_pos: (-1, -1),
            start: (w, h),
            prev: 0,
            move_direction: direction,
        }
    }

    fn connect_start(&mut self, i: usize) -> Option<usize> {
        let next_x = self.pos.0 + MOVES[i][0];
        let next_y = self.pos.1 + MOVES[i][1];
        let pipe = match self.maze.get(next_x, next_y) {
            Some(x) => x,
            _ => return None,
        };
        if CONNECT[i].contains(&pipe) {
            self.prev = opposite_direction(i);
            self.pos = (next_x, next_y);
            self.steps += 1;
            return Some(self.steps);
        }
        None
    }

    fn next_move(&mut self) {
        let pipe = match self.maze.get(self.pos.0, self.pos.1) {
            Some(x) => x,
            _ => panic!("Current pos pipe not found??"),
        };
        let next_dir = match next_direction(pipe, self.prev) {
            Some(x) => x,
            _ => panic!("Somehow ended up on invalid pipe"),
        };
        let next_x = self.pos.0 + MOVES[next_dir][0];
        let next_y = self.pos.1 + MOVES[next_dir][1];
        let next_pipe = match self.maze.get(next_x, next_y) {
            Some(x) => x,
            _ => panic!("Next pipe at {}, {} is not valid", next_x, next_y),
        };
        if !CONNECT[next_dir].contains(&next_pipe) {
            panic!("Next pipe {} at {}, {} is not a valid connection from ({}) @ {}, {} towards {}. prev: {}", next_pipe, next_x, next_y, pipe, self.pos.0, self.pos.1, next_dir, self.prev);
        }
        self.prev_pos = self.pos;
        self.prev = opposite_direction(next_dir);
        self.pos = (next_x, next_y);
        self.steps += 1;
    }

    pub fn next(&mut self) -> usize {
        if self.pos == self.start {
            if self.move_direction == MoveDirection::Backward {
                for i in (0..4).rev() {
                    match self.connect_start(i) {
                        Some(x) => return x,
                        _ => continue,
                    }
                }
            } else {
                for i in 0..4 {
                    match self.connect_start(i) {
                        Some(x) => return x,
                        _ => continue,
                    }
                }
            }
            panic!("Nothing connected to start position!");
        }
        self.next_move();
        self.steps
    }

    pub fn get(&self) -> Option<char> {
        self.maze.get(self.pos.0, self.pos.1)
    }
}

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

fn process_part_1(input: &str) -> Result<usize, AoCError> {
    let maze_data = Maze::new(input);
    let mut maze_pos_fwd = MazePos::new(&maze_data, MoveDirection::Forward);
    let mut maze_pos_bwd = maze_pos_fwd;
    maze_pos_bwd.move_direction = MoveDirection::Backward;
    let mut result = 0;
    maze_pos_fwd.next();
    maze_pos_bwd.next();
    while !(maze_pos_fwd.pos.0 == maze_pos_bwd.pos.0 && maze_pos_fwd.pos.1 == maze_pos_bwd.pos.1) {
        maze_pos_fwd.next();
        if maze_pos_fwd.pos.0 == maze_pos_bwd.pos.0 && maze_pos_fwd.pos.1 == maze_pos_bwd.pos.1 {
            break;
        }
        result = maze_pos_bwd.next();
    }
    Ok(result)
}

fn get_right_direction(dir: usize) -> usize {
    if dir == NORTH {
        return WEST;
    }
    dir - 1
}

fn get_left_direction(dir: usize) -> usize {
    if dir == WEST {
        return NORTH;
    }
    dir + 1
}

fn set_mark(maze: &mut [Vec<char>], dir: usize, mark: char, pos: (i32, i32)) {
    let next_x = pos.0 + MOVES[dir][0];
    let next_y = pos.1 + MOVES[dir][1];
    if next_x >= 0
        && next_y >= 0
        && next_x < (maze[0].len() as i32)
        && next_y < (maze.len() as i32)
        && maze[next_y as usize][next_x as usize] == '.'
    {
        maze[next_y as usize][next_x as usize] = mark;
    }
}

fn reconstruct_maze(input: &str) -> Result<Vec<Vec<char>>, AoCError> {
    let maze = Maze::new(input);
    let mut cleared_maze = vec![vec!['.'; maze.width]; maze.height];
    let mut mark = MazePos::new(&maze, MoveDirection::Forward);
    cleared_maze[mark.start.1 as usize][mark.start.0 as usize] = 'S';
    mark.next();
    // Idea:
    // Traveling CW, the inside is always to the right of the direction we're heading
    // However, we don't know which direction we're heading when we go around the pipe, even though we do know they are distinct
    // Let's mark it, and then decide later based on whichever touches the edges
    while !(mark.pos.0 == mark.start.0 && mark.pos.1 == mark.start.1) {
        let block = match mark.get() {
            Some(x) => x,
            _ => panic!(),
        };
        cleared_maze[mark.pos.1 as usize][mark.pos.0 as usize] = block;
        let left = get_left_direction(mark.prev);
        let right = get_right_direction(mark.prev);

        // Set side & outside corners
        set_mark(&mut cleared_maze, left, 'X', mark.pos);
        if mark.prev == WEST && block == '7'
            || mark.prev == NORTH && block == 'J'
            || mark.prev == EAST && block == 'L'
            || mark.prev == SOUTH && block == 'F'
        {
            set_mark(
                &mut cleared_maze,
                opposite_direction(mark.prev),
                'X',
                mark.pos,
            );
        }

        set_mark(&mut cleared_maze, right, 'Y', mark.pos);
        if mark.prev == SOUTH && block == '7'
            || mark.prev == WEST && block == 'J'
            || mark.prev == NORTH && block == 'L'
            || mark.prev == EAST && block == 'F'
        {
            set_mark(
                &mut cleared_maze,
                opposite_direction(mark.prev),
                'Y',
                mark.pos,
            );
        }
        mark.next();
    }
    Ok(cleared_maze)
}

fn determine_outside(maze: &[Vec<char>]) -> Option<char> {
    // This is getting long, so I'm going to do a little shortcut because I can at the cost of rigor.
    // Normally, you'd scan the entire perimeter and search the open space until you found which
    // char is the outside, but we can just scan the first line in this case since all of them have
    // an "outside" that touches the first line.
    // I'm also not considering the worst case: that the pipe expands the perimeter of the entire grid.
    // In that case, you'd want the newly constructed grid to be +2 x and y bigger than the original so
    // that there's empty space to be found around it.
    let line = maze.iter().next().unwrap();
    for block in line.iter() {
        if *block == 'X' || *block == 'Y' {
            return Some(*block);
        }
    }
    None
}

fn count_inside(maze: &[Vec<char>], inside: char) -> usize {
    maze.iter()
        .map(|line| line.iter().filter(|&&block| block == inside).count())
        .sum()
}

fn fill_inside(maze: &mut Vec<Vec<char>>, inside: char) {
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut q = Queue::new();
    maze.iter().enumerate().for_each(|(row, line)| {
        line.iter()
            .enumerate()
            .filter(|(_, &block)| block == inside)
            .for_each(|(col, _)| {
                q.queue((col, row)).unwrap();
                visited[row][col] = true;
            });
    });
    while !q.is_empty() {
        let (col, row) = q.dequeue().unwrap();
        for next in MOVES.iter() {
            let next_row = (row as i32) + next[0];
            let next_col = (col as i32) + next[1];
            if next_row >= 0
                && next_col >= 0
                && next_row < (maze.len() as i32)
                && next_col < (maze[0].len() as i32)
                && maze[next_row as usize][next_col as usize] == '.'
                && !visited[next_row as usize][next_col as usize]
            {
                maze[next_row as usize][next_col as usize] = inside;
                visited[next_row as usize][next_col as usize] = true;
                q.queue(((next_col as usize), (next_row as usize))).unwrap();
            }
        }
    }
}

fn process_part_2(input: &str) -> Result<usize, AoCError> {
    let mut maze = reconstruct_maze(input)?;

    let outside = determine_outside(&maze).unwrap();
    let inside = match outside {
        'X' => 'Y',
        'Y' => 'X',
        _ => panic!(),
    };
    println!("Outside is: {}", outside);
    println!("Inside is: {}", inside);
    fill_inside(&mut maze, inside);
    for line in maze.iter() {
        for point in line.iter() {
            print!("{}", point);
        }
        println!();
    }
    let count = count_inside(&maze, inside);
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() -> Result<()> {
        let input_1 = ".....
.S-7.
.|.|.
.L-J.
.....";
        let input_2 = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(process_part_1(input_1)?, 4);
        assert_eq!(process_part_1(input_2)?, 8);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let input_1 = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";
        let input_2 = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let input_3 = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(process_part_2(input_1)?, 4);
        assert_eq!(process_part_2(input_2)?, 8);
        assert_eq!(process_part_2(input_3)?, 10);

        Ok(())
    }
}
