// day 4
// puzzle: https://adventofcode.com/2025/day/3
//
// todo: add comments. short on time for day 4!
//
// benchmarks (release mode, amd 3600):
// part 1: ~187µs
// part 2: ~7.5ms

use shared::constants;

type ParsedInput = Vec<Vec<u8>>;
type Answer = usize;

fn parse_input(raw_input: &str) -> ParsedInput {
    raw_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '@' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

fn run_part_one(input: ParsedInput) -> Answer {
    let rows = input.len();
    let cols = input[0].len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if input[r][c] == 0 {
                continue;
            }

            let mut sum = 0;

            for &(dr, dc) in &constants::NB_VECTORS {
                let r_adj = r as i32 + dr as i32;
                let c_adj = c as i32 + dc as i32;

                if r_adj < 0 || r_adj >= rows as i32 || c_adj < 0 || c_adj >= cols as i32 {
                    continue;
                }

                sum += input[r_adj as usize][c_adj as usize];
            }

            if sum < 4 {
                count += 1;
            }
        }
    }

    count
}

fn run_part_two(input: ParsedInput) -> Answer {
    let rows = input.len();
    let cols = input[0].len();

    let mut removed_this_pass = Vec::with_capacity(rows * cols);
    let mut removed = vec![vec![false; cols]; rows];

    let mut r_range = 0..rows;
    let mut c_range = 0..cols;

    loop {
        removed_this_pass.clear();

        let mut min_r = rows;
        let mut max_r = 0;
        let mut min_c = cols;
        let mut max_c = 0;

        for r in r_range.clone() {
            for c in c_range.clone() {
                if removed[r][c] || input[r][c] == 0 {
                    continue;
                }

                let mut sum = 0;

                for &(dr, dc) in &constants::NB_VECTORS {
                    let r_adj = r as i32 + dr as i32;
                    let c_adj = c as i32 + dc as i32;

                    if r_adj < 0 || r_adj >= rows as i32 || c_adj < 0 || c_adj >= cols as i32 {
                        continue;
                    }

                    let r_adj = r_adj as usize;
                    let c_adj = c_adj as usize;

                    if !removed[r_adj][c_adj] {
                        sum += input[r_adj][c_adj];
                    }
                }

                if sum < 4 {
                    removed_this_pass.push((r, c));

                    min_r = min_r.min(r);
                    max_r = max_r.max(r);

                    min_c = min_c.min(c);
                    max_c = max_c.max(c);
                }
            }
        }

        if removed_this_pass.is_empty() {
            break;
        }

        for &(r, c) in &removed_this_pass {
            removed[r][c] = true;
        }

        r_range = (min_r.saturating_sub(1))..(max_r + 2).min(rows);
        c_range = (min_c.saturating_sub(1))..(max_c + 2).min(cols);
    }

    removed.into_iter().flatten().filter(|&b| b).count()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let part = args.get(1).map(String::as_str).unwrap_or("p1");

    let raw_input = include_str!("input.txt");
    let parsed_input = parse_input(raw_input);

    let loose_start = std::time::Instant::now();

    match part {
        "p1" => {
            let result = run_part_one(parsed_input);
            println!("Part 1: {} ({:?})", result, loose_start.elapsed());
        }
        "p2" => {
            let result = run_part_two(parsed_input);
            println!("Part 2: {} ({:?})", result, loose_start.elapsed());
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_one() {
        let result = run_part_one(parse_input(TEST_INPUT));
        assert_eq!(result, 13)
    }

    #[test]
    fn test_part_two() {
        let result = run_part_two(parse_input(TEST_INPUT));
        assert_eq!(result, 43)
    }
}
