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

pub fn solve_puzzle_b(input: &str) -> u32 {
    let mut count = 0;
    let mut dial = 50;
    for (dir, clicks) in input.lines().map(|l| l.split_at(1)) {
        let mut clicks = clicks.parse::<i32>().expect("should be valid number");
        match dir {
            "L" => clicks *= -1,
            "R" => {}
            _ => panic!("unknown direction"),
        }
        // Here are some asserts to demonstrate the behaviour of `div_euclid`.
        // Case 1 (turning the dial to the right or left, without arriving at or crossing 0):
        assert_eq!(0, 20_i32.div_euclid(100));
        // Case 2 (turning the dial to the right, and arriving exactly back at 0, without crossing it multiple times):
        assert_eq!(1, 100_i32.div_euclid(100));
        // Case 3 (turning the dial to the right, and crossing 0 once):
        assert_eq!(1, 120_i32.div_euclid(100));
        // Case 4 (turning the dial to the right, and arriving exactly back at 0, and crossing it multiple times):
        assert_eq!(2, 200_i32.div_euclid(100));
        assert_eq!(3, 300_i32.div_euclid(100));
        // Case 5 (turning the dial to the right, and crossing 0 multiple times):
        assert_eq!(2, 220_i32.div_euclid(100));
        assert_eq!(3, 320_i32.div_euclid(100));
        // Case 6 (turning the dial to the left, and arriving exactly at 0, without crossing it):
        assert_eq!(0, 0_i32.div_euclid(100));
        // Case 7 (turning the dial to the left, and crossing 0 once):
        assert_eq!(-1, (-20_i32).div_euclid(100));
        // Case 8 (turning the dial to the left, and arriving exactly back at 0, and crossing it multiple times):
        assert_eq!(-1, (-100_i32).div_euclid(100));
        assert_eq!(-2, (-200_i32).div_euclid(100));
        // Case 9 (turning the dial to the left, and crossing 0 multiple times):
        assert_eq!(-2, (-120_i32).div_euclid(100));
        assert_eq!(-3, (-220_i32).div_euclid(100));
        // So there are some cases where `div_euclid` throws off our answer:
        // - Case 6: should be counted as 1, but isn't
        // - Case 7: incorrect when starting from 0 (dial 0 and L20 yields 1 but should be 0) but correct otherwise (dial 20 and L40 yields 1)
        // - Case 8: correct when starting from 0 (dial 0 and L100 yields 1) but incorrect otherwise (dial 50 and L150 yields 1 but should be 2)
        // - Case 9: similarly to case 7, when starting from 0 `div_euclid` is off by one (dial 0 and L120 yields 2, but should be 1)
        let mut zero_crossings = (dial + clicks).div_euclid(100).unsigned_abs();
        // Handle case 6:
        if clicks < 0 && (dial + clicks) == 0 {
            zero_crossings += 1;
        }
        // Handle case 7 & 9:
        if clicks < 0 && dial == 0 {
            zero_crossings -= 1;
        }
        // Handle case 8:
        if clicks < 0 && dial != 0 && (dial + clicks) != 0 && (dial + clicks) % 100 == 0 {
            zero_crossings += 1;
        }
        dial = (dial + clicks).rem_euclid(100);
        count += zero_crossings;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::{solve_puzzle_a, solve_puzzle_b};

    #[test]
    fn example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        assert_eq!(solve_puzzle_a(input), 3);
    }

    #[test]
    fn example_two() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        assert_eq!(solve_puzzle_b(input), 6);
    }
}
