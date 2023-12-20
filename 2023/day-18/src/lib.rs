pub const MOVES: [[i64; 2]; 4] = [
    // N - E - S - W
    [0, -1],
    [1, 0],
    [0, 1],
    [-1, 0],
];

pub const NORTH: usize = 0;
pub const EAST: usize = 1;
pub const SOUTH: usize = 2;
pub const WEST: usize = 3;

fn get_direction(dir: char) -> usize {
    match dir {
        'U' => NORTH,
        'R' => EAST,
        'L' => WEST,
        'D' => SOUTH,
        _ => panic!(),
    }
}

#[derive(Debug)]
pub struct Command {
    pub direction: usize,
    pub steps: usize,
    pub color: u32,
}

impl Command {
    pub fn new(input: &str) -> Option<Self> {
        let mut items = input.split(' ');
        let direction = get_direction(items.next()?.chars().next()?);
        let steps = match items.next()?.parse::<usize>() {
            Ok(x) => x,
            _ => return None,
        };
        let color = match u32::from_str_radix(
            items.next()?.trim_start_matches("(#").trim_end_matches(')'),
            16,
        ) {
            Ok(x) => x,
            _ => return None,
        };
        Some(Command {
            direction,
            steps,
            color,
        })
    }
    pub fn new_part2(input: &str) -> Option<Self> {
        let items = input.split(' ');
        let hex_string = items.last()?.trim_start_matches("(#").trim_end_matches(')');
        let steps = match usize::from_str_radix(&hex_string[..5], 16) {
            Ok(x) => x,
            _ => return None,
        };
        let direction = match hex_string.chars().last() {
            Some(x) => match x {
                '0' => WEST,
                '1' => SOUTH,
                '2' => EAST,
                '3' => NORTH,
                _ => return None,
            },
            _ => return None,
        };
        Some(Command {
            direction,
            steps,
            color: 0,
        })
    }
}
