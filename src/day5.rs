use std::ops::RangeInclusive;

pub fn solve_puzzle_a(input: &str) -> usize {
    let mut ranges = Vec::new();
    let mut fresh = 0;
    let mut lines = input.lines();
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        ranges.push(parse_range(line));
    }
    for line in lines {
        let id = line.parse().unwrap();
        if check_id(id, &ranges) {
            fresh += 1;
        }
    }

    fresh
}

fn check_id(id: usize, ranges: &[RangeInclusive<usize>]) -> bool {
    ranges.iter().any(|r| r.contains(&id))
}

fn parse_range(line: &str) -> RangeInclusive<usize> {
    let (start, end) = line
        .split_once('-')
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        .unwrap();

    start..=end
}

#[cfg(test)]
mod tests {
    use super::{check_id, solve_puzzle_a};

    #[test]
    fn example() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";

        assert_eq!(solve_puzzle_a(input), 3);
    }

    #[test]
    fn check_ids() {
        let ranges = [3..=5, 10..=14, 16..=20, 12..=18];
        let inputs = [1, 5, 8, 11, 17, 32];
        let expected = [false, true, false, true, true, false];

        for (&input, expected) in inputs.iter().zip(expected) {
            assert_eq!(check_id(input, &ranges), expected);
        }
    }
}
