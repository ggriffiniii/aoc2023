use std::{cmp::Ordering, ops::Range};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone)]
struct Map(Vec<(Range<isize>, isize)>);
impl Map {
    fn new(input: &str) -> Map {
        let mut entries: Vec<_> = input
            .lines()
            .skip(1)
            .map(|line| {
                let (dest_start, rem) = line.split_once(" ").unwrap();
                let (source_start, range_len) = rem.split_once(" ").unwrap();
                let dest_start: isize = dest_start.parse().unwrap();
                let source_start: isize = source_start.parse().unwrap();
                let range_len: isize = range_len.parse().unwrap();

                (
                    (source_start..source_start + range_len),
                    dest_start - source_start,
                )
            })
            .collect();
        entries.sort_by_key(|(range, _offset)| range.start);
        Map(entries)
    }

    fn find_offset(&self, needle: isize) -> Option<isize> {
        self.0
            .binary_search_by(|(range, _)| {
                match (range.start.cmp(&needle), range.end.cmp(&needle)) {
                    (Ordering::Equal | Ordering::Less, Ordering::Greater) => Ordering::Equal,
                    (Ordering::Greater, _) => Ordering::Greater,
                    (_, Ordering::Less | Ordering::Equal) => Ordering::Less,
                }
            })
            .map(|idx| self.0[idx].1)
            .ok()
    }

    fn translate(&self, input: isize) -> isize {
        input + self.find_offset(input).unwrap_or(0)
    }

    fn chain(self, next: Map) -> Map {
        let input_ranges = self.0;
        let translated_ranges: Vec<_> = input_ranges
            .iter()
            .cloned()
            .map(|(range, offset)| range.start + offset..range.end + offset)
            .collect();
        let mut next_ranges = next.0;
        for (input_range, input_offset) in input_ranges {
            eprintln!(
                "chaining ({input_range:?}, {input_offset}) into {:?}",
                next_ranges
            );
            let mut translated_input_range =
                input_range.start + input_offset..input_range.end + input_offset;
            eprintln!("{input_range:?} translates to {translated_input_range:?}");
            /*
            let mut idx = next_ranges
                .binary_search_by(|(range, _)| match translated_input_range.start.cmp(&range.end) {
                    Ordering::Equal | Ordering::Greater => Ordering::Greater,
                    Ordering::Less => match translated_input_range.end.cmp(&range.start) {
                        Ordering::Greater => Ordering::Equal,
                        Ordering::Less | Ordering::Equal => Ordering::Less,
                    },
                })
                .unwrap_or_else(|idx| idx);
            */

            let start_idx = 0;
            let mut idx = start_idx;
            let mut entries_to_replace = Vec::new();
            while !translated_input_range.is_empty() && idx < next_ranges.len() {
                dbg!(idx);
                let (next_range, next_offset) = next_ranges[idx].clone();
                match (
                    next_range.start.cmp(&translated_input_range.start),
                    next_range.end.cmp(&translated_input_range.end),
                ) {
                    (Ordering::Equal, Ordering::Equal) => {
                        entries_to_replace.push((next_range, input_offset + next_offset));
                        translated_input_range.end = translated_input_range.start; // empty range to signal we're done
                    }
                    (Ordering::Equal, Ordering::Greater) => {
                        entries_to_replace
                            .push((translated_input_range.clone(), input_offset + next_offset));
                        entries_to_replace
                            .push((translated_input_range.end..next_range.end, next_offset));
                        translated_input_range.end = translated_input_range.start; // empty range to signal we're done
                    }
                    (Ordering::Equal, Ordering::Less) => {
                        entries_to_replace.push((next_range.clone(), input_offset + next_offset));
                        translated_input_range.start = next_range.end;
                    }
                    (Ordering::Greater, Ordering::Equal) => {
                        entries_to_replace
                            .push((translated_input_range.start..next_range.start, input_offset));
                        entries_to_replace.push((next_range, input_offset + next_offset));
                        translated_input_range.end = translated_input_range.start; // empty range to signal we're done
                    }
                    (Ordering::Greater, Ordering::Greater) => {
                        entries_to_replace
                            .push((translated_input_range.start..next_range.start, input_offset));
                        entries_to_replace.push((
                            next_range.start..translated_input_range.end,
                            input_offset + next_offset,
                        ));
                        entries_to_replace
                            .push((translated_input_range.end..next_range.end, next_offset));
                        translated_input_range.end = translated_input_range.start; // empty range to signal we're done
                    }
                    (Ordering::Greater, Ordering::Less) => {
                        entries_to_replace
                            .push((translated_input_range.start..next_range.start, input_offset));
                        entries_to_replace
                            .push((next_range.start..next_range.end, input_offset + next_offset));
                        translated_input_range.start = next_range.end;
                    }
                    (Ordering::Less, Ordering::Equal) => {
                        eprintln!("less, equal");
                        entries_to_replace
                            .push((translated_input_range.start..next_range.start, input_offset));
                        entries_to_replace.push((next_range.clone(), input_offset + next_offset));
                        translated_input_range.end = translated_input_range.start; // empty range to signal we're done
                    }
                    (Ordering::Less, Ordering::Greater) => {
                        entries_to_replace
                            .push((next_range.start..translated_input_range.start, next_offset));
                        entries_to_replace.push((
                            translated_input_range.start..translated_input_range.end,
                            input_offset,
                        ));
                        entries_to_replace
                            .push((translated_input_range.end..next_range.end, next_offset));
                        translated_input_range.end = translated_input_range.start; // empty range to signal we're done
                    }
                    (Ordering::Less, Ordering::Less) => {
                        entries_to_replace
                            .push((next_range.start..translated_input_range.start, next_offset));
                        entries_to_replace.push((translated_input_range.clone(), input_offset));
                        translated_input_range.end = translated_input_range.start; // empty range to signal we're done
                    }
                }
                idx += 1;
            }
            if !translated_input_range.is_empty() {
                entries_to_replace.push((translated_input_range, input_offset));
            }
            next_ranges.splice(start_idx..idx, entries_to_replace);
            eprintln!("resulting in {:?}", next_ranges);
        }
        Map(next_ranges)
    }
}

const EXAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

#[aoc(day5, part1)]
fn part1(input: &str) -> isize {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<isize> = seeds
        .split(" ")
        .skip(1)
        .map(|seed_id| seed_id.parse().unwrap())
        .collect();
    let map_chain: Vec<_> = maps
        .split("\n\n")
        .map(|map_input| Map::new(map_input))
        .collect();

    seeds
        .into_iter()
        .map(|mut id| {
            for map in map_chain.iter() {
                id = map.translate(id)
            }
            id
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> isize {
    //let input = EXAMPLE;
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<isize> = seeds
        .split(" ")
        .skip(1)
        .map(|seed_id| seed_id.parse().unwrap())
        .collect();
    let map_chain = maps
        .split("\n\n")
        .map(|map_input| Map::new(map_input))
        .reduce(|a, b| a.chain(b));

    seeds
        .as_chunks::<2>()
        .0
        .into_iter()
        .flat_map(|&[start, len]| start..start + len)
        .map(|mut id| {
            for map in map_chain.iter() {
                id = map.translate(id)
            }
            id
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    proptest! {
        #[test]
        fn my_test(a in 0..10isize, b in 0..10isize, c in 0..10isize, d in 0..10isize, e in 0..20isize) {
            //let map_a = Map(vec![(a..a+b, a+b)]);
            //let map_b = Map(vec![(c..c+d, c+d)]);
            let map_a = Map(vec![(1..7, 3)]);
            let map_b = Map(vec![(4..9, 5)]);

            dbg!(&map_a);
            dbg!(&map_b);

            let chained_map = map_a.clone().chain(map_b.clone());

            //let samples = (a+b).max(c+d) + 1;
            let samples = 15;
            let simple: Vec<_> = (0..samples).map(|i| map_b.translate(map_a.translate(i)) - i).collect();
            let chained: Vec<_> = (0..samples).map(|i| chained_map.translate(i) - i).collect();

            assert_eq!(simple, chained);
        }
    }
}
