use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    input
        .split("\n")
        .map(|line| {
            let mut digits = line
                .as_bytes()
                .iter()
                .copied()
                .filter(|&x| x.is_ascii_digit())
                .map(|x| (x - b'0') as usize);
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

struct Searcher<'a>(&'a str);

impl<'a> Iterator for Searcher<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        while !self.0.is_empty() {
            let first_byte = self.0.as_bytes()[0];
            if first_byte >= b'0' && first_byte <= b'9' {
                self.0 = &self.0[1..];
                return Some((first_byte - b'0') as usize);
            }

            for (value, name) in [
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .into_iter()
            .enumerate()
            {
                if self.0.starts_with(name) {
                    self.0 = &self.0[1..];
                    return Some(value);
                }
            }
            self.0 = &self.0[1..];
        }
        None
    }
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    input
        .split("\n")
        .map(|line| {
            let mut searcher = Searcher(line);
            let first = searcher.next().unwrap();
            let last = searcher.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}
