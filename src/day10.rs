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
        let rows = self.joltage_target.len();
        let cols = self.buttons.len() + 1;
        let mut matrix = Vec::with_capacity(rows * cols);
        for (target_idx, &target) in self.joltage_target.iter().enumerate() {
            for button in self.buttons.iter() {
                let b = if button.contains(&target_idx) {
                    1.0
                } else {
                    0.0
                };
                matrix.push(b);
            }
            matrix.push(target as f64);
        }
        debug_assert_eq!(matrix.len(), rows * cols);
        for row in 0..rows {
            println!("{:+.2?}", &matrix[(row * cols)..(row * cols + cols)]);
        }
        println!("_______________");

        // See: `https://en.wikipedia.org/wiki/Gaussian_elimination`
        const EPSILON: f64 = 1.0E-10;
        let mut pivot_row = 0;
        let mut pivot_col = 0;
        while pivot_row < rows && pivot_col < cols {
            // Find pivot
            let mut max = f64::MIN;
            let mut max_idx = pivot_row;
            for row in pivot_row..rows {
                let c = matrix[row * cols + pivot_col].abs();
                if c > max {
                    max = c;
                    max_idx = row;
                }
            }
            // Check if found pivot is zero.
            // (With tolerance to account for floating point precision limits)
            if matrix[max_idx * cols + pivot_col].abs() <= EPSILON {
                // No pivot in this column. Move to next one.
                pivot_col += 1;
            } else {
                // Swap rows
                for col in 0..cols {
                    matrix.swap(pivot_row * cols + col, max_idx * cols + col);
                }
                // For all rows below pivot
                for row in (pivot_row + 1)..rows {
                    let factor =
                        matrix[row * cols + pivot_col] / matrix[pivot_row * cols + pivot_col];
                    // Set lower part of pivot column to zero.
                    matrix[row * cols + pivot_col] = 0.0;
                    // Subtract multiplied pivot row from remaining elements of current row.
                    for col in (pivot_col + 1)..cols {
                        matrix[row * cols + col] -= matrix[pivot_row * cols + col] * factor;
                    }
                }
                pivot_col += 1;
                pivot_row += 1;
            }
        }
        for row in 0..rows {
            println!("{:+.2?}", &matrix[(row * cols)..(row * cols + cols)]);
        }
        todo!()
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
