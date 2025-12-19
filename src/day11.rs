use aoc_runner_derive::aoc;

fn solve(input: &str, expansion_rate: usize) -> usize {
    let mut row_idx = 0;
    let mut col_idx = 0;
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    let mut galaxies = Vec::new();
    for b in input.bytes() {
        match b {
            b'#' => {
                galaxies.push((col_idx, row_idx));
                *get_idx(&mut empty_rows, row_idx, true) = false;
                *get_idx(&mut empty_cols, col_idx, true) = false;
            }
            b'.' => {}
            b'\n' => {
                row_idx += 1;
                col_idx = 0;
                continue;
            }
            x => panic!("unexpected input: {}", x as char),
        }
        col_idx += 1;
    }
    let mut offset = 0usize;
    let row_expansion_offsets: Vec<_> = empty_rows
        .into_iter()
        .map(|is_empty| {
            if is_empty {
                offset += expansion_rate - 1;
            }
            offset
        })
        .collect();
    offset = 0;
    let col_expansion_offsets: Vec<_> = empty_cols
        .into_iter()
        .map(|is_empty| {
            if is_empty {
                offset += expansion_rate - 1;
            }
            offset
        })
        .collect();

    for (x, y) in galaxies.iter_mut() {
        *x += col_expansion_offsets[*x];
        *y += row_expansion_offsets[*y];
    }

    all_pairs(&galaxies)
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum()
}

fn all_pairs(s: &[(usize, usize)]) -> impl Iterator<Item = ((usize, usize), (usize, usize))> {
    (0..s.len() - 1).flat_map(move |ai| (ai + 1..s.len()).map(move |bi| (s[ai], s[bi])))
}

fn get_idx(v: &mut Vec<bool>, idx: usize, default: bool) -> &mut bool {
    if v.len() <= idx {
        v.resize(idx + 1, default);
    }
    &mut v[idx]
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    solve(input, 1_000_000)
}
