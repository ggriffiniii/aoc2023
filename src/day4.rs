use aoc_runner_derive::aoc;

#[derive(Debug)]
struct CardGame {
    id: usize,
    winning_numbers: u128,
    numbers_you_have: u128,
}

impl CardGame {
    fn new(input: &str) -> Self {
        let (mut id, input) = input.split_once(": ").unwrap();
        let (winning, have) = input.split_once(" | ").unwrap();
        while !id.as_bytes()[0].is_ascii_digit() {
            id = &id[1..];
        }
        let id = id.parse().unwrap();

        fn num_to_bitset(input: &str) -> u128 {
            input
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .fold(0u128, |bs, n| bs | (1 << n))
        }
        let winning_numbers = num_to_bitset(winning);
        let numbers_you_have = num_to_bitset(have);
        CardGame {
            id,
            winning_numbers,
            numbers_you_have,
        }
    }

    fn num_winning_numbers_you_have(&self) -> u32 {
        (self.winning_numbers & self.numbers_you_have).count_ones()
    }

    fn points(&self) -> usize {
        let count = self.num_winning_numbers_you_have();
        if count == 0 { 0 } else { 2usize.pow(count - 1) }
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| CardGame::new(line))
        .map(|game| game.points())
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let games: Vec<_> = input.lines().map(|line| CardGame::new(line)).collect();
    let mut num_instances = vec![1; games.len()];

    for idx in 0..games.len() {
        let game = &games[idx];
        let num_cards_won = game.num_winning_numbers_you_have() as usize;
        for copy_idx in idx + 1..idx + 1 + num_cards_won {
            num_instances[copy_idx] += num_instances[idx];
        }
    }

    num_instances.into_iter().sum()
}
