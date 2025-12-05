// day 5
// puzzle: https://adventofcode.com/2025/day/5
//
// fairly straightforward puzzle today
//
// benchmarks (release mode, amd 3600):
// part 1: ~13µs
// part 2: ~5µs

type ParsedInput = (Vec<(u64, u64)>, Vec<u64>);
type Answer = u64;

fn parse_input(raw_input: &str) -> ParsedInput {
    raw_input
        .split_once("\n\n")
        .or_else(|| raw_input.split_once("\r\n\r\n"))
        .map(|(r, id)| {
            (
                r.lines()
                    .filter_map(|r| r.split_once("-"))
                    .map(|(a_str, b_str)| (a_str.parse().unwrap(), b_str.parse().unwrap()))
                    .collect(),
                id.lines().filter_map(|l| l.parse().ok()).collect(),
            )
        })
        .unwrap()
}

fn merge_ranges(ranges: &mut Vec<(u64, u64)>) {
    ranges.sort_unstable_by_key(|r| r.0);

    ranges.dedup_by(|a, b| {
        if b.1 >= a.0 {
            b.1 = b.1.max(a.1);
            return true;
        }
        false
    });
}

fn run_part_one(input: ParsedInput) -> Answer {
    let (mut ranges, ids) = input;
    merge_ranges(&mut ranges);

    let mut count = 0;

    for id in ids {
        // the index of the first range where id is less than the start of the range
        // essentially binary search
        let i = ranges.partition_point(|range| range.0 <= id);

        // we then check the previous range where id would be greater than the start of the
        // range and check it falls within
        if i > 0 && id <= ranges[i - 1].1 {
            count += 1;
        }
    }

    count
}

fn run_part_two(input: ParsedInput) -> Answer {
    let mut ranges = input.0;
    merge_ranges(&mut ranges);

    ranges.iter().fold(0, |acc, (a, b)| acc + (b - a) + 1)
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
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_one() {
        let result = run_part_one(parse_input(TEST_INPUT));
        assert_eq!(result, 3)
    }

    #[test]
    fn test_part_two() {
        let result = run_part_two(parse_input(TEST_INPUT));
        assert_eq!(result, 14)
    }
}
