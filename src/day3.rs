use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Grid{
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
        Grid{row_len, data}
    }

    fn num_rows(&self) -> usize {
        self.data.len() / self.row_len
    }

    fn rows(&self) -> impl Iterator<Item=&[u8]> {
        self.data.chunks(self.row_len)
    }
}

const EXAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

#[aoc(day3, part1)]
fn part1(input: &str) -> String {
    let input = EXAMPLE;
    let grid = Grid::new(input);
    for (row_idx, row) in grid.rows().enumerate() {
        let mut start_idx = 0;
        while start_idx < row.len() {
            if row[start_idx].is_ascii_digit() {
                let mut end_idx = start_idx+1;
                while row[end_idx].is_ascii_digit() && end_idx < row.len() {
                    end_idx += 1;
                }
                let num: usize = row[start_idx..end_idx].iter().fold(0, |num, digit| num * 10 + (digit - b'0') as usize);
                {
                    let start_idx = start_idx as isize;
                    let end_idx = end_idx as isize;
                    let neighbors = (start_idx-1 .. end_idx+1).flat_map(|y| {
                        (row_idx as isize - 1 ..= row_idx as isize + 1).map(|x| (x, y))
                    }).filter(|&(x,y)| {
                        x >= 0 && y >= 0 && x < row.len() as isize && y < grid.num_rows() as isize
                    }).filter(|&(x,y)| {
                        !(y == row_idx as isize && x >= start_idx && x < end_idx)
                    });
                    
                }
            }
        }
    }
    42
}

#[aoc(day3, part2)]
fn part2(input: &str) -> String {
    todo!()
}