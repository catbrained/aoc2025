use std::ops::RangeInclusive;

pub fn solve_puzzle_a(input: &str) -> usize {
    input
        .split(',')
        .map(str::trim)
        .map(|range_str| {
            let (start, end) = range_str
                .split_once('-')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
                .unwrap();
            start..=end
        })
        .fold(0, |acc, e| acc + solve_range(e).iter().sum::<usize>())
}

fn solve_range(range: RangeInclusive<usize>) -> Vec<usize> {
    let mut invalid_ids = Vec::new();
    for id in range {
        let id_str = id.to_string();
        if id_str.len() % 2 != 0 {
            continue;
        }
        let half = id_str.len() / 2;
        let (first, second) = id_str.split_at(half);
        if first == second {
            invalid_ids.push(id);
        }
    }

    invalid_ids
}

#[cfg(test)]
mod tests {
    use super::{solve_puzzle_a, solve_range};

    #[test]
    fn example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(1227775554, solve_puzzle_a(input));
    }

    #[test]
    fn example_ranges() {
        let inputs = [
            (11..=22),
            (95..=115),
            (998..=1012),
            (1188511880..=1188511890),
            (222220..=222224),
            (1698522..=1698528),
            (446443..=446449),
            (38593856..=38593862),
            (565653..=565659),
            (824824821..=824824827),
            (2121212118..=2121212124),
        ];
        let expected = [
            vec![11, 22],
            vec![99],
            vec![1010],
            vec![1188511885],
            vec![222222],
            vec![],
            vec![446446],
            vec![38593859],
            vec![],
            vec![],
            vec![],
        ];

        for (input, expected) in inputs.into_iter().zip(expected) {
            assert_eq!(solve_range(input), expected);
        }
    }
}
