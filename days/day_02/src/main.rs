// day 2 involves finding 'invalid ids', which are numbers made from
// a sequence of repeating digits
//
// the input provides large ranges of numbers to check, and the simple
// approach is to brute force over each number and check its validity
//
// a faster approach is to generate invalid ids for each range of
// the input
//
// this can be done efficiently by mathematically computing
// which 'blocks' of digits can be multiplied into a sequence,
// and the range at which these blocks are valid
//
// benchmarks (release mode, amd 3600):
// part 1: ~36µs
// part 2: ~41µs

use std::collections::HashSet;

type ParsedInput = Vec<(u64, u64)>;
type Answer = u64;

// powers of 10. the number required is the max expected repeats
// in a sequence, which is the length of the largest number in the input.
pub const POWERS: [u64; 10] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
];

fn parse_input(raw_input: &str) -> ParsedInput {
    raw_input
        .split(',')
        .flat_map(|r| r.split_once("-"))
        .map(|(a_str, b_str)| (a_str.parse().unwrap(), b_str.parse().unwrap()))
        .collect()
}

fn gen_invalid_ids((a, b): (u64, u64), max_repeats: Option<usize>) -> HashSet<u64> {
    let mut invalid_ids = HashSet::new();
    let max_digits = b.to_string().len();

    // a block of digits must be at most half the length of the containing string
    // in order to create a repeating sequence
    for block_len in 1..=(max_digits / 2) {
        // the boundary values for blocks at a given length
        // e.g. for blocks of length 3, 100 <= block < 1000
        let min_block_val = POWERS[block_len - 1];
        let max_block_val = POWERS[block_len];

        let mut repeats = 2;

        loop {
            if let Some(limit) = max_repeats
                && repeats > limit
            {
                break;
            }

            // stop if the sequence would be too long
            if block_len * repeats > max_digits {
                break;
            }

            // for each block, we can generate a multiplier that will
            // create the sequence. this is derived from length of the block
            // and number of repeats.
            // e.g. take 54, we can multiply by 54 x 101 = 5454 (2 repeats)
            // or 54 x 10101 = 545454 (3 repeats)
            let mut multiplier = 0;
            for r in 0..repeats {
                multiplier += POWERS[r * block_len];
            }

            // sequences must be in range: a <= sequence <= b
            // and since sequence = block * multipler
            // then (a / multipler) <= block <= (b / multiplier)
            // ceil/floor and clamp to get precise boundaries
            let block_range = ((a + multiplier - 1) / multiplier).max(min_block_val)
                ..=(b / multiplier).min(max_block_val - 1);

            for block in block_range {
                invalid_ids.insert(block * multiplier);
            }

            repeats += 1;
        }
    }

    invalid_ids
}

fn run_part_one(input: ParsedInput) -> Answer {
    input
        .into_iter()
        .flat_map(|range| gen_invalid_ids(range, Some(2)))
        .sum()
}

fn run_part_two(input: ParsedInput) -> Answer {
    input
        .into_iter()
        .flat_map(|range| gen_invalid_ids(range, None))
        .sum()
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

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one() {
        let result = run_part_one(parse_input(TEST_INPUT));
        assert_eq!(result, 1227775554)
    }

    #[test]
    fn test_part_two() {
        let result = run_part_two(parse_input(TEST_INPUT));
        assert_eq!(result, 4174379265)
    }
}
