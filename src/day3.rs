use aoc_runner_derive::aoc;

use std::ops::Range;

#[derive(Debug)]
struct Grid {
    row_len: usize,
    data: Vec<u8>,
}
impl Grid {
    fn new(input: &str) -> Grid {
        let mut row_len = 0;
        let mut data = Vec::with_capacity(input.len());
        for line in input.lines() {
            row_len = line.len();
            data.extend_from_slice(line.as_bytes());
        }
        Grid { row_len, data }
    }

    fn num_rows(&self) -> usize {
        self.data.len() / self.row_len
    }

    fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.data.chunks(self.row_len)
    }
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut part_sum = 0;
    for (row_idx, row) in grid.rows().enumerate() {
        let mut start_idx = 0;
        while start_idx < row.len() {
            if row[start_idx].is_ascii_digit() {
                let mut end_idx = start_idx + 1;
                while end_idx < row.len() && row[end_idx].is_ascii_digit() {
                    end_idx += 1;
                }
                let part_num: usize = row[start_idx..end_idx]
                    .iter()
                    .fold(0, |num, digit| num * 10 + (digit - b'0') as usize);
                {
                    let start_idx = start_idx as isize;
                    let end_idx = end_idx as isize;
                    let mut neighbors = (start_idx - 1..end_idx + 1)
                        .flat_map(|x| {
                            (row_idx as isize - 1..=row_idx as isize + 1).map(move |y| (x, y))
                        })
                        .filter(|&(x, y)| {
                            // remove any locations outside the grid
                            x >= 0
                                && y >= 0
                                && x < row.len() as isize
                                && y < grid.num_rows() as isize
                        })
                        .filter(|&(x, y)| {
                            // remove locations where the part_num is
                            !(y == row_idx as isize && x >= start_idx && x < end_idx)
                        })
                        .map(|(x, y)| (x as usize, y as usize));
                    let touching_symbol =
                        neighbors.any(|(x, y)| grid.data[y * row.len() + x] != b'.');
                    if touching_symbol {
                        part_sum += part_num;
                    }
                }
                start_idx = end_idx;
            } else {
                start_idx += 1;
            }
        }
    }
    part_sum
}

fn find_adjacent_numbers(grid: &Grid, idx: usize) -> Vec<usize> {
    let to_xy = |idx: usize| ((idx % grid.row_len) as isize, (idx / grid.row_len) as isize);
    let to_idx = |x, y| y as usize * grid.row_len + x as usize;

    let (x, y) = to_xy(idx);

    let adjacent_cells = (x - 1..=x + 1)
        .flat_map(|x| (y - 1..=y + 1).map(move |y| (x, y)))
        .filter(|&(x, y)| {
            // remove any locations outside the grid
            x >= 0 && y >= 0 && x < grid.row_len as isize && y < grid.num_rows() as isize
        })
        .map(|(x, y)| to_idx(x, y));

    let mut num_ranges: Vec<Range<usize>> = adjacent_cells
        .filter_map(|idx| {
            if !grid.data[idx].is_ascii_digit() {
                return None;
            }
            let (mut x, y) = to_xy(idx);
            while x > 0 && grid.data[to_idx(x - 1, y)].is_ascii_digit() {
                x -= 1;
            }
            let start_idx = to_idx(x, y);
            let (mut x, y) = to_xy(idx);
            while x < grid.row_len as isize && grid.data[to_idx(x, y)].is_ascii_digit() {
                x += 1;
            }
            let end_idx = to_idx(x, y);
            Some(start_idx..end_idx)
        })
        .collect();
    num_ranges.sort_by_key(|range| (range.start, range.end));
    num_ranges.dedup();
    num_ranges
        .into_iter()
        .map(|range| {
            let mut num = 0;
            for idx in range {
                num = num * 10 + (grid.data[idx] - b'0') as usize;
            }
            num
        })
        .collect()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let grid = Grid::new(input);

    grid.data
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, symbol)| symbol == b'*')
        .map(|(idx, _)| idx)
        .filter_map(|idx| {
            // determine if 2 adjacent numbers and return the product of them, else skip
            let adjacent_numbers = find_adjacent_numbers(&grid, idx);
            if adjacent_numbers.len() == 2 {
                return Some(adjacent_numbers.into_iter().product::<usize>());
            }
            None
        })
        .sum()
}
