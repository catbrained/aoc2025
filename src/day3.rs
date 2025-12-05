pub fn solve_puzzle_a(input: &str) -> usize {
    input.lines().map(solve_bank).sum()
}

fn solve_bank(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut result = [0, 0];
    for idx in 0..(bytes.len() - 1) {
        if bytes[idx] > result[0] {
            result[0] = bytes[idx];
            result[1] = find_largest(&bytes[idx + 1..]).1;
        }
    }

    str::from_utf8(&result).unwrap().parse().unwrap()
}

fn find_largest(input: &[u8]) -> (usize, u8) {
    let mut largest = 0;
    let mut idx = 0;
    for (n, &b) in input.iter().enumerate() {
        if b > largest {
            largest = b;
            idx = n;
        }
        if b == 9 {
            break;
        }
    }

    (idx, largest)
}

pub fn solve_puzzle_b(input: &str) -> usize {
    input.lines().map(solve_bank_two).sum()
}

fn solve_bank_two(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut result = [0; 12];
    let mut start = 0;
    let mut end = bytes.len() - 11;
    for res in &mut result {
        let (pos, num) = find_largest(&bytes[start..end]);
        *res = num;
        start += pos + 1;
        end += 1;
    }

    str::from_utf8(&result).unwrap().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{solve_bank, solve_bank_two, solve_puzzle_a, solve_puzzle_b};

    #[test]
    fn example() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";

        assert_eq!(solve_puzzle_a(input), 357);
    }

    #[test]
    fn example_banks() {
        let inputs = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let expected = [98, 89, 78, 92];
        for (input, expected) in inputs.into_iter().zip(expected) {
            assert_eq!(solve_bank(input), expected);
        }
    }

    #[test]
    fn example2() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";

        assert_eq!(solve_puzzle_b(input), 3121910778619);
    }

    #[test]
    fn example_banks2() {
        let inputs = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let expected = [987654321111, 811111111119, 434234234278, 888911112111];
        for (input, expected) in inputs.into_iter().zip(expected) {
            assert_eq!(solve_bank_two(input), expected);
        }
    }
}
