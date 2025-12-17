use aoc_runner_derive::aoc;

fn next_value(input: &mut [isize]) -> isize {
    let first = input[0];
    if input.iter().all(|&e| first == e) {
        return first;
    }
    let mut prev = first;
    for idx in 1..input.len() {
        input[idx - 1] = input[idx] - prev;
        prev = input[idx];
    }
    let next_level_len = input.len() - 1;
    input[input.len() - 1] + next_value(&mut input[..next_level_len])
}

#[aoc(day9, part1)]
fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let mut seq: Vec<isize> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            next_value(&mut seq)
        })
        .sum()
}

fn prev_value(input: &mut [isize]) -> isize {
    let first = input[0];
    if input.iter().all(|&e| first == e) {
        return first;
    }
    let mut prev = first;
    for idx in 1..input.len() {
        let curr = input[idx];
        input[idx] = curr - prev;
        prev = curr;
    }
    input[0] - prev_value(&mut input[1..])
}

#[aoc(day9, part2)]
fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let mut seq: Vec<isize> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            prev_value(&mut seq)
        })
        .sum()
}
