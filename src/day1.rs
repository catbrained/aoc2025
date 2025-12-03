pub fn solve_puzzle_a(input: &str) -> usize {
    let mut count = 0;
    let mut dial = 50;
    for (dir, clicks) in input.lines().map(|l| l.split_at(1)) {
        let mut clicks = clicks.parse::<i32>().expect("should be valid number");
        match dir {
            "L" => clicks *= -1,
            "R" => {}
            _ => panic!("unknown direction"),
        }
        dial = (dial + clicks).rem_euclid(100);
        if dial == 0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::solve_puzzle_a;

    #[test]
    fn example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        assert_eq!(solve_puzzle_a(input), 3);
    }
}
