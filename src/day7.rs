use std::{
    cmp::{self, Ordering},
    fmt,
};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Hand(Vec<u8>, [u8; 5]);
impl Hand {
    fn new(hand_input: &str) -> Self {
        let mut hand = [0; 5];
        let mut hand_matches = vec![0; 13];
        for (idx, b) in hand_input.bytes().enumerate() {
            let card_value = match b {
                b'2'..=b'9' => b - b'2',
                b'T' => 8,
                b'J' => 9,
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                b => panic!("unrecognized card: {}", b as char),
            };
            hand_matches[card_value as usize] += 1;
            hand[idx] = card_value;
        }
        let mut hand_type: Vec<_> = hand_matches
            .into_iter()
            .filter(|&count| count > 0)
            .collect();
        hand_type.sort_by_key(|&b| cmp::Reverse(b));
        Hand(hand_type, hand)
    }

    fn new_with_jokers(hand_input: &str) -> Self {
        let mut hand = [0; 5];
        let mut hand_matches = vec![0; 13];
        for (idx, b) in hand_input.bytes().enumerate() {
            let card_value = match b {
                b'J' => 0,
                b'2'..=b'9' => b - b'1',
                b'T' => 9,
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                b => panic!("unrecognized card: {}", b as char),
            };
            hand_matches[card_value as usize] += 1;
            hand[idx] = card_value;
        }
        let mut hand_type: Vec<_> = hand_matches[1..]
            .into_iter()
            .copied()
            .filter(|&count| count > 0)
            .collect();
        hand_type.sort_by_key(|&b| cmp::Reverse(b));
        if hand_type.is_empty() {
            // must be all jokers
            hand_type.push(5);
        } else {
            hand_type[0] += hand_matches[0];
        }
        Hand(hand_type, hand)
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand_input, bid)| {
            let bid: usize = bid.parse().unwrap();
            let hand = Hand::new(hand_input);
            (hand, bid)
        })
        .collect();
    hands.sort_by_key(|(hand, _bid)| hand.clone());
    hands
        .into_iter()
        .enumerate()
        .map(|(idx, (_hand, bid))| (idx + 1) * bid)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand_input, bid)| {
            let bid: usize = bid.parse().unwrap();
            let hand = Hand::new_with_jokers(hand_input);
            (hand, bid)
        })
        .collect();
    hands.sort_by_key(|(hand, _bid)| hand.clone());
    hands
        .into_iter()
        .enumerate()
        .map(|(idx, (_hand, bid))| (idx + 1) * bid)
        .sum()
}
