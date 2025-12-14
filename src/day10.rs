use std::collections::{HashSet, VecDeque};

pub fn solve_puzzle_a(input: &str) -> usize {
    let machines = input.lines().map(|l| l.into());

    machines.map(|m: Machine| m.configure()).sum()
}

pub fn solve_puzzle_b(input: &str) -> usize {
    let machines = input.lines().map(|l| l.into());

    machines.map(|m: Machine| m.configure_joltage()).sum()
}

type Button = HashSet<usize>;

#[derive(Debug)]
struct Machine {
    target: HashSet<usize>,
    joltage_target: Vec<usize>,
    buttons: Vec<Button>,
}

impl Machine {
    fn configure(&self) -> usize {
        let mut queue = VecDeque::new();
        let mut seen_states = Vec::new();
        let root = HashSet::new();
        seen_states.push(root.clone());
        queue.push_back(Node {
            presses: 0,
            state: root,
        });
        while let Some(node) = queue.pop_front() {
            if node.state == self.target {
                return node.presses;
            }
            for button in &self.buttons {
                let next = &node.state ^ button;
                if !seen_states.contains(&next) {
                    seen_states.push(next.clone());
                    queue.push_back(Node {
                        presses: node.presses + 1,
                        state: next,
                    });
                }
            }
        }

        unreachable!("Machine should be configurable");
    }

    fn configure_joltage(&self) -> usize {
        let rows = self.buttons.len() + 1;
        let cols = self.joltage_target.len() + self.buttons.len() + 2;
        let mut matrix = Vec::with_capacity(rows * cols);
        for (button_idx, button) in self.buttons.iter().enumerate() {
            for (target_idx, _) in self.joltage_target.iter().enumerate() {
                let b = if button.contains(&target_idx) {
                    1.0
                } else {
                    0.0
                };
                matrix.push(b);
            }
            for n in 0..self.buttons.len() {
                if n == button_idx {
                    matrix.push(1.0);
                } else {
                    matrix.push(0.0);
                }
            }
            matrix.push(0.0);
            matrix.push(1.0);
        }
        for &target in self.joltage_target.iter() {
            matrix.push(-(target as f64));
        }
        matrix.extend(std::iter::repeat_n(0.0, self.buttons.len()));
        matrix.push(1.0);
        matrix.push(0.0);
        debug_assert_eq!(matrix.len(), rows * cols);

        // Apply simplex algorithm
        loop {
            // Find pivot column
            let mut pivot_col = 0;
            let mut largest_negative = f64::MAX;
            for col in 0..(cols - 1) {
                let c = matrix[(rows - 1) * cols + col];
                if c < 0.0 && c < largest_negative {
                    pivot_col = col;
                    largest_negative = c;
                }
            }
            if largest_negative == f64::MAX {
                // We are done!
                break;
            }
            // Find pivot row
            let mut pivot_row = 0;
            let mut smallest_fraction = f64::MAX;
            for row in 0..(rows - 1) {
                let b = matrix[row * cols + cols - 1];
                let a = matrix[row * cols + pivot_col];
                if a <= 0.0 {
                    continue;
                }
                let f = b / a;
                if f < smallest_fraction {
                    pivot_row = row;
                    smallest_fraction = f;
                }
            }
            let pivot = matrix[pivot_row * cols + pivot_col];
            debug_assert!(pivot != 0.0);
            // Divide pivot row by pivot
            matrix[pivot_row * cols + pivot_col] = 1.0;
            for col in 0..cols {
                if col == pivot_col {
                    continue;
                }
                matrix[pivot_row * cols + col] /= pivot;
            }
            // Set other rows in pivot column to zero.
            for row in 0..rows {
                if row == pivot_row {
                    continue;
                }
                let factor = matrix[row * cols + pivot_col];
                matrix[row * cols + pivot_col] = 0.0;
                for col in 0..cols {
                    if col == pivot_col {
                        continue;
                    }
                    matrix[row * cols + col] -= matrix[pivot_row * cols + col] * factor;
                }
            }
        }
        // println!("{:+.3?}", &matrix[((rows - 1) * cols)..]);

        // Buttons can only be pressed an integer number of times and also presses >= 0.
        let range_start = (rows - 1) * cols + self.joltage_target.len();
        let range_end = range_start + self.buttons.len();
        let coefficients: Vec<_> = matrix[range_start..range_end]
            .iter()
            .map(|&f| {
                let f = f.trunc();
                if f < 0.0 { 0_usize } else { f as usize }
            })
            .collect();
        // The simplex algorithm above did not operate on integers.
        // Therefore, we might not yet have the optimal _integer_ solution,
        // only the optimal non-integer one, turned into integers by truncating and clamping negative numbers to zero.
        // The truncated and clamped solution might not even be a valid solution anymore.
        // So now we need to start searching the area around the current "solution" for valid
        // solutions and then take the best one.
        let minus_one = coefficients.iter().map(|&c| c.saturating_sub(1)).collect();
        let plus_one = coefficients.iter().map(|&c| c + 1).collect();
        let mut best_valid = usize::MAX;
        for list in [&coefficients, &minus_one, &plus_one] {
            let mut current_presses = 0;
            for (joltage_idx, &target) in self.joltage_target.iter().enumerate() {
                let sum: usize = list
                    .iter()
                    .zip(&self.buttons)
                    .map(|(&c, btn)| if btn.contains(&joltage_idx) { c } else { 0 })
                    .sum();
                if sum == target && current_presses + sum < best_valid {
                    current_presses += sum;
                } else {
                    current_presses = 0;
                    break;
                }
            }
            if current_presses != 0 && current_presses < best_valid {
                println!("Found new best! Previous: {best_valid}, New: {current_presses}");
                best_valid = current_presses;
                current_presses = 0;
            }
        }

        let solution = *matrix.last().unwrap();

        debug_assert!(solution > 0.0);
        debug_assert!(solution.is_finite());

        solution as usize
    }
}

#[derive(Debug)]
struct Node {
    presses: usize,
    state: HashSet<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut target = HashSet::new();
        let mut joltage_target = Vec::new();
        let mut buttons = Vec::new();
        let elements = value.split_whitespace();
        for elem in elements {
            match elem.as_bytes()[0] {
                b'[' => {
                    for (idx, c) in elem.as_bytes()[1..elem.len() - 1].iter().enumerate() {
                        match c {
                            b'.' => continue,
                            b'#' => {
                                let _ = target.insert(idx);
                            }
                            _ => panic!("unknown indicator"),
                        }
                    }
                }
                b'(' => {
                    let nums = elem[1..elem.len() - 1].split(',');
                    let mut button = Button::new();
                    for n in nums.map(|n| n.parse::<usize>().unwrap()) {
                        let _ = button.insert(n);
                    }
                    buttons.push(button);
                }
                b'{' => {
                    let nums = elem[1..elem.len() - 1].split(',');
                    for n in nums.map(|n| n.parse::<usize>().unwrap()) {
                        joltage_target.push(n);
                    }
                }
                _ => panic!("unknown element"),
            }
        }

        Self {
            target,
            joltage_target,
            buttons,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Machine, solve_puzzle_a, solve_puzzle_b};

    #[test]
    fn example() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n";

        assert_eq!(solve_puzzle_a(input), 7);
    }

    #[test]
    fn example_machines() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n";
        let mut machines = input.lines().map(Machine::from);

        assert_eq!(machines.next().unwrap().configure(), 2);
        assert_eq!(machines.next().unwrap().configure(), 3);
        assert_eq!(machines.next().unwrap().configure(), 2);
    }

    #[test]
    fn example2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n";

        assert_eq!(solve_puzzle_b(input), 33);
    }

    #[test]
    fn example_machines2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n";
        let mut machines = input.lines().map(Machine::from);

        assert_eq!(machines.next().unwrap().configure_joltage(), 10);
        assert_eq!(machines.next().unwrap().configure_joltage(), 12);
        assert_eq!(machines.next().unwrap().configure_joltage(), 11);
    }
}
