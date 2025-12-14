use aoc_runner_derive::aoc;

const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

// For a race of time, determine the min and max speeds that will reach at least
// distance.
fn winning_range(time: usize, distance: usize) -> (usize, usize) {
    let time = time as isize;
    let distance = distance as isize;

    // The distance (d) covered over time (t) depending on the speed (s) is
    // d = s * (t - s)
    // or
    // d = -s² + ts
    // Now in a form suitable for the quadratic equation
    // 0 = -s² + ts - d (A = -1, B = t, C = -d)
    let discriminant = (time * time) - (4 * -1 * -distance);
    assert!(discriminant > 0);
    let min = (-time as f64 + (discriminant as f64).sqrt()) / -2.0;
    let min = (min + 1.0).floor() as usize;
    let max = (-time as f64 - (discriminant as f64).sqrt()) / -2.0;
    let max = (max - 1.0).ceil() as usize;
    (min, max)
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let (time, distance) = input.split_once("\n").unwrap();
    time.split_whitespace().zip(distance.split_whitespace()).skip(1).map(|(time, distance)| {
        let time: usize = time.parse().unwrap();
        let distance: usize = distance.parse().unwrap();
        let (min, max) = winning_range(time, distance);
        max - min + 1
    }).product()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    //let input = EXAMPLE;
    let (time, distance) = input.split_once("\n").unwrap();
    let time = time.as_bytes().iter().filter(|b| b.is_ascii_digit()).fold(0usize, |accum, b| accum * 10 + (b - b'0') as usize);
    let distance = distance.as_bytes().iter().filter(|b| b.is_ascii_digit()).fold(0usize, |accum, b| accum * 10 + (b - b'0') as usize);
    let (min, max) = winning_range(time, distance);

    max - min + 1
}
