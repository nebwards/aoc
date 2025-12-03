// day 3
// puzzle: https://adventofcode.com/2025/day/3
//
// benchmarks (release mode, amd 3600):
// part 1: ~24µs
// part 2: ~60µs

type ParsedInput = Vec<Vec<u8>>;
type Answer = u64;

fn parse_input(raw_input: &str) -> ParsedInput {
    raw_input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn joltage(bank: &[u8], out_len: usize) -> u64 {
    let mut sum = 0;
    let mut start = 0;

    for j in 0..out_len {
        // the end of each window is adjusted to ensure there will
        // be enough digits in the bank to complete the output
        let end = bank.len() - (out_len - (j + 1));

        // find the first max number in the window. we can short circuit early
        // on 9 since its always the largest digit
        let mut window_max_idx = 0;
        let mut max = 0;
        for (i, &n) in bank[start..end].iter().enumerate() {
            if n > max {
                max = n;
                window_max_idx = i;

                if n == 9 {
                    break;
                }
            }
        }

        // a neat way of joining digits on the fly
        // e.g. 5 * 10 + 4 = 54
        sum = sum * 10 + max as u64;

        // the next window is positioned just after the current window max
        start += window_max_idx + 1;
    }

    sum
}

fn run_part_one(input: ParsedInput) -> Answer {
    input.iter().map(|bank| joltage(bank, 2)).sum()
}

fn run_part_two(input: ParsedInput) -> Answer {
    input.iter().map(|bank| joltage(bank, 12)).sum()
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
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_one() {
        let result = run_part_one(parse_input(TEST_INPUT));
        assert_eq!(result, 357)
    }

    #[test]
    fn test_part_two() {
        let result = run_part_two(parse_input(TEST_INPUT));
        assert_eq!(result, 3121910778619)
    }
}
