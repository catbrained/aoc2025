pub fn solve_puzzle_a(input: &str) -> usize {
    input.lines().map(solve_bank).sum()
}

fn solve_bank(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut result = [0, 0];
    for idx in 0..(bytes.len() - 1) {
        if bytes[idx] > result[0] {
            result[0] = bytes[idx];
            result[1] = find_largest(&bytes[idx + 1..]);
        }
    }

    str::from_utf8(&result).unwrap().parse().unwrap()
}

fn find_largest(input: &[u8]) -> u8 {
    let mut largest = 0;
    for &b in input {
        if b > largest {
            largest = b;
        }
        if b == 9 {
            break;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::{solve_bank, solve_puzzle_a};

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
}
