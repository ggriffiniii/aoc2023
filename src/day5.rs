use std::{cmp::Ordering, ops::Range};

use aoc_runner_derive::aoc;

struct Map(Vec<(Range<isize>, isize)>);
impl Map {
    fn new(input: &str) -> Map {
        let mut entries: Vec<_> = input.lines().skip(1).map(|line| {
            let (dest_start, rem) = line.split_once(" ").unwrap();
            let (source_start, range_len) = rem.split_once(" ").unwrap();
            let dest_start: isize = dest_start.parse().unwrap();
            let source_start: isize = source_start.parse().unwrap();
            let range_len: isize = range_len.parse().unwrap();

            ((source_start..source_start+range_len), dest_start-source_start)
        }).collect();
        entries.sort_by_key(|(range, _offset)| range.start);
        Map(entries)
    }

    fn find_offset(&self, needle: isize) -> Option<isize> {
        self.0.binary_search_by(|(range, _)| {
            match (range.start.cmp(&needle), range.end.cmp(&needle)) {
                (Ordering::Equal | Ordering::Less, Ordering::Greater) => Ordering::Equal,
                (Ordering::Greater, _) => Ordering::Greater,
                (_, Ordering::Less | Ordering::Equal) => Ordering::Less,
            }
        }).map(|idx| self.0[idx].1).ok()
    }

    fn translate(&self, input: isize) -> isize {
        input + self.find_offset(input).unwrap_or(0)
    }

    fn chain(&mut self, next: Map) {
        for (mut range_to_chain, mut offset)in next.0 {
            eprintln!("chaining ({range_to_chain:?}, {offset}) into {:?}", self.0);
            let mut idx = self.0.partition_point(|(range, _)| {
                range.end < range_to_chain.start
            });
            let start_idx = idx;
            let mut entries_to_replace = Vec::new();
            while !range_to_chain.is_empty() && idx < self.0.len() {
                let (existing_range, existing_offset) = self.0[idx].clone();
                match (existing_range.start.cmp(&range_to_chain.start), existing_range.end.cmp(&range_to_chain.end)) {
                    (Ordering::Equal, Ordering::Equal) => {
                        entries_to_replace.push((existing_range, offset));
                        range_to_chain.end = range_to_chain.start;  // empty range to signal we're done.
                    },
                    (Ordering::Equal, Ordering::Greater) => {
                        entries_to_replace.push((range_to_chain.clone(), existing_offset+offset));
                        entries_to_replace.push((range_to_chain.end..existing_range.end, existing_offset));
                        range_to_chain.end = range_to_chain.start; // empty range to signal we're done
                    },
                    (Ordering::Equal, Ordering::Less) => {
                        entries_to_replace.push((existing_range.clone(), existing_offset+offset));
                        range_to_chain.start = existing_range.end;
                    },
                    (Ordering::Greater, Ordering::Equal) => {
                        entries_to_replace.push((range_to_chain.start..existing_range.start, offset));
                        entries_to_replace.push((existing_range, existing_offset+offset));
                        range_to_chain.end = range_to_chain.start;  // empty range to signal we're done
                    },
                    (Ordering::Greater, Ordering::Greater) => {
                        entries_to_replace.push((range_to_chain.start..existing_range.start, offset));
                        entries_to_replace.push((existing_range.start..range_to_chain.end, existing_offset+offset));
                        entries_to_replace.push((range_to_chain.end..existing_range.end, existing_offset));
                        range_to_chain.end = range_to_chain.start;  // empty range to signal we're done
                    },
                    (Ordering::Greater, Ordering::Less) => {
                        entries_to_replace.push((range_to_chain.start..existing_range.start, offset));
                        entries_to_replace.push((existing_range.start..existing_range.end, existing_offset+offset));
                        range_to_chain.start = existing_range.end;
                    }
                    (Ordering::Less, Ordering::Equal) => {
                        entries_to_replace.push((existing_range.start..range_to_chain.start, existing_offset));
                        entries_to_replace.push((range_to_chain.clone(), existing_offset+offset));
                        range_to_chain.end = range_to_chain.start;  // empty range to signal we're done

                    },
                    (Ordering::Less, Ordering::Greater) => {
                        entries_to_replace.push((existing_range.start..range_to_chain.start, existing_offset));
                        entries_to_replace.push((range_to_chain.start..range_to_chain.end, existing_offset+offset));
                        entries_to_replace.push((range_to_chain.end..existing_range.end, existing_offset));
                        range_to_chain.end = range_to_chain.start;  // empty range to signal we're done
                    },
                    (Ordering::Less, Ordering::Less) => {
                        entries_to_replace.push((existing_range.start..range_to_chain.start, existing_offset));
                        entries_to_replace.push((range_to_chain.start..existing_range.end, existing_offset+offset));
                        range_to_chain.start = existing_range.end;
                    }
                }
                idx += 1;
            }
            if !range_to_chain.is_empty() {
                entries_to_replace.push((range_to_chain, offset));
            }
            self.0.splice(start_idx..idx, entries_to_replace);
            eprintln!("resulting in {:?}", self.0);
        }
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
    let seeds: Vec<isize> = seeds.split(" ").skip(1).map(|seed_id| seed_id.parse().unwrap()).collect();
    let map_chain: Vec<_> = maps.split("\n\n").map(|map_input| {
        Map::new(map_input)
    }).collect();

    seeds.into_iter().map(|mut id| {
        for map in map_chain.iter() {
            id = map.translate(id)
        }
        id
    }).min().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> isize {
    //let input = EXAMPLE;
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<isize> = seeds.split(" ").skip(1).map(|seed_id| seed_id.parse().unwrap()).collect();
    let map_chain = maps.split("\n\n").map(|map_input| {
        Map::new(map_input)
    }).reduce(|mut a, b| { a.chain(b); a});

    seeds.as_chunks::<2>().0.into_iter().flat_map(|&[start, len]| start..start+len).map(|mut id| {
        for map in map_chain.iter() {
            id = map.translate(id)
        }
        id
    }).min().unwrap()
}