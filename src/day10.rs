use aoc_runner_derive::aoc;

// Last 4 bits represent the direction. Upper 4 bits represent the opposite
// direction. A rotate_right(4) will switch NORTH <=> SOUTH and EAST <=> WEST.
const NORTH: u8 = 0b0010_0001;
const SOUTH: u8 = 0b0001_0010;
const EAST: u8 = 0b1000_0100;
const WEST: u8 = 0b0100_1000;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Tile(u8);
impl Tile {
    fn new(input: u8) -> Tile {
        match input {
            b'|' => Tile(NORTH | SOUTH),
            b'-' => Tile(EAST | WEST),
            b'L' => Tile(NORTH | EAST),
            b'J' => Tile(NORTH | WEST),
            b'7' => Tile(SOUTH | WEST),
            b'F' => Tile(SOUTH | EAST),
            b'.' => Tile(0),
            // starting point can go in any direction.
            b'S' => Tile(NORTH | EAST | SOUTH | WEST),
            x => panic!("unrecognized tile: {}", x as char),
        }
    }

    fn can_accept(self, incoming_direction: u8) -> bool {
        self.0 & incoming_direction != 0
    }
}

#[derive(Debug)]
struct Grid {
    start: usize,
    row_len: usize,
    data: Vec<Tile>,
}
impl Grid {
    fn new(input: &str) -> Grid {
        let data: Vec<_> = input
            .bytes()
            .filter(|&b| b != b'\n')
            .map(Tile::new)
            .collect();
        let row_len = input.bytes().position(|b| b == b'\n').unwrap();
        let start = data
            .iter()
            .position(|tile| tile.0 == NORTH | EAST | SOUTH | WEST)
            .unwrap();
        Grid {
            start,
            row_len,
            data,
        }
    }

    fn next_tile_pos(&self, start: usize, direction: u8) -> Option<(usize, u8)> {
        let next_pos = match direction {
            NORTH => {
                let start_row = start / self.row_len;
                if start_row > 0 {
                    start - self.row_len
                } else {
                    return None;
                }
            }
            EAST => {
                let start_col = start % self.row_len;
                if start_col + 1 < self.row_len {
                    start + 1
                } else {
                    return None;
                }
            }
            SOUTH => {
                let next_tile = start + self.row_len;
                if next_tile < self.data.len() {
                    next_tile
                } else {
                    return None;
                }
            }
            WEST => {
                let start_col = start % self.row_len;
                if start_col > 0 {
                    start - 1
                } else {
                    return None;
                }
            }
            _ => panic!("unrecognized direction"),
        };
        let next_pipe = self.data[next_pos];
        let opposing_direction = direction.rotate_right(4);
        if next_pipe.can_accept(opposing_direction) {
            let next_direction = next_pipe.0 & !opposing_direction;
            Some((next_pos, next_direction))
        } else {
            None
        }
    }

    fn find_loop_len(&self) -> usize {
        for start_dir in [NORTH, EAST, SOUTH, WEST] {
            let mut current_tile_pos = self.start;
            let mut current_direction = start_dir;
            let mut num_steps = 0usize;
            loop {
                let (pos, dir) = match self.next_tile_pos(current_tile_pos, current_direction) {
                    Some(x) => x,
                    None => break,
                };
                num_steps += 1;
                if pos == self.start {
                    return num_steps;
                }
                current_tile_pos = pos;
                current_direction = dir;
            }
        }
        unreachable!()
    }

    fn find_loop_tiles(&self) -> (Vec<usize>, Tile) {
        for start_dir in [NORTH, EAST, SOUTH, WEST] {
            let mut path = vec![self.start];
            let mut current_direction = start_dir;
            loop {
                let (pos, dir) = match self.next_tile_pos(*path.last().unwrap(), current_direction)
                {
                    Some(x) => x,
                    None => break,
                };
                if pos == self.start {
                    return (path, Tile(start_dir | current_direction.rotate_right(4)));
                }
                path.push(pos);
                current_direction = dir;
            }
        }
        unreachable!()
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.find_loop_len() / 2
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    let (mut loop_tiles, start_tile) = grid.find_loop_tiles();
    loop_tiles.sort();
    // Once the loop is found we know what type of tile the start position is.
    grid.data[grid.start] = start_tile;

    let mut is_outside = true;
    loop_tiles
        .windows(2)
        .map(|pair| {
            let a = pair[0];
            let b = pair[1];
            // If we run into any pipe that goes in the NORTH direction (|, J,
            // L) change whether we're inside or outside. We could have just as
            // easily chosen SOUTH instead.
            if grid.data[a].can_accept(NORTH) {
                is_outside = !is_outside;
            }
            if is_outside {
                return 0;
            }
            b - a - 1
        })
        .sum()
}
