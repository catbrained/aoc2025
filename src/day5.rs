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

pub fn solve_puzzle_b(input: &str) -> usize {
    let mut ranges = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        ranges.push(Some(parse_range(line)));
    }
    ranges.sort_by_key(|r| *r.as_ref().unwrap().start());
    for idx in 0..(ranges.len() - 1) {
        let first = ranges[idx].as_ref();
        let second = ranges[idx + 1].as_ref();
        let first_end = first.as_ref().unwrap().end();
        let second_start = second.as_ref().unwrap().start();
        if second_start <= first_end {
            let first_start = first.as_ref().unwrap().start();
            let second_end = second.as_ref().unwrap().end();
            let end = if first_end > second_end {
                first_end
            } else {
                second_end
            };
            let merged = *first_start..=*end;
            ranges[idx] = None;
            ranges[idx + 1] = Some(merged);
        }
    }
    let mut id_count = 0;
    for range in ranges.iter().filter(|&o| o.is_some()) {
        id_count += range.as_ref().unwrap().end() - range.as_ref().unwrap().start() + 1;
    }

    id_count
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
    use super::{check_id, solve_puzzle_a, solve_puzzle_b};

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

    #[test]
    fn example2() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";

        assert_eq!(solve_puzzle_b(input), 14);
    }
}
