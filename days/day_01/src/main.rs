type ParsedInput = Vec<i32>;
type Answer = usize;

const MOD: i32 = 100;
const START_POS: i32 = 50;

fn parse_input(raw_input: &str) -> ParsedInput {
    raw_input
        .lines()
        .map(|l| {
            let (direction, num_str) = l.split_at(1);

            let sign = match direction {
                "L" => -1,
                "R" | _ => 1,
            };

            sign * num_str.parse::<i32>().unwrap()
        })
        .collect()
}

fn run_part_one(input: ParsedInput) -> Answer {
    let mut pos = START_POS;
    let mut count = 0;

    for n in input {
        let next = (pos + n).rem_euclid(MOD);

        if next == 0 {
            count += 1;
        }

        pos = next;
    }

    count as usize
}

fn run_part_two(input: ParsedInput) -> Answer {
    let mut pos = START_POS;
    let mut count = 0;

    for n in input {
        let sum = pos + n;

        // full rotation
        count += (sum / MOD).abs();

        // account for partial rotation that crosses 0
        if pos != 0 && sum <= 0 {
            count += 1;
        }

        pos = sum.rem_euclid(MOD);
    }

    count as usize
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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_one() {
        let result = run_part_one(parse_input(TEST_INPUT));
        assert_eq!(result, 3)
    }

    #[test]
    fn test_part_two() {
        let result = run_part_two(parse_input(TEST_INPUT));
        assert_eq!(result, 6)
    }
}
