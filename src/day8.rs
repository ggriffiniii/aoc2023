use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let (directions, tree_nodes) = input.split_once("\n\n").unwrap();
    let tree: HashMap<_, _> = tree_nodes
        .lines()
        .map(|nodes| {
            let id = &nodes[..3];
            let left = &nodes[7..][..3];
            let right = &nodes[12..][..3];
            (id, (left, right))
        })
        .collect();

    let mut current = "AAA";
    for (idx, direction) in directions.chars().cycle().enumerate() {
        if current == "ZZZ" {
            return idx;
        }
        let children = tree[current];
        if direction == 'L' {
            current = children.0;
        } else {
            debug_assert!(direction == 'R');
            current = children.1;
        }
    }
    unreachable!()
}

// least common multiple
fn lcm(a: usize, b: usize) -> usize {
    // greatest common divisor
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b // Avoid potential overflow by dividing first
    }
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    // The input data ensures that a path from a starting node to an ending node
    // takes X steps, and then X more steps always gets you back to the ending
    // node again. The input data is very specialized to allow for computing the
    // lcm of all the paths.
    let (directions, tree_nodes) = input.split_once("\n\n").unwrap();
    let tree: HashMap<_, _> = tree_nodes
        .lines()
        .map(|nodes| {
            let id = &nodes[..3];
            let left = &nodes[7..][..3];
            let right = &nodes[12..][..3];
            (id, (left, right))
        })
        .collect();

    tree.keys()
        .copied()
        .filter(|k| k.ends_with("A"))
        .map(|mut current| {
            for (idx, direction) in directions.chars().cycle().enumerate() {
                if current.ends_with("Z") {
                    return idx;
                }
                let children = tree[current];
                if direction == 'L' {
                    current = children.0;
                } else {
                    debug_assert!(direction == 'R');
                    current = children.1;
                }
            }
            unreachable!()
        })
        .fold(1, |a, steps| lcm(a, steps))
}
