use std::collections::{BTreeSet, HashSet, VecDeque};

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
        let mut matrix: Vec<i32> = Vec::with_capacity(rows * cols);
        // The maximum number of presses for each button
        let mut button_limits = vec![i32::MAX; cols - 1];
        for (target_idx, &target) in self.joltage_target.iter().enumerate() {
            for (button_idx, button) in self.buttons.iter().enumerate() {
                let b = if button.contains(&target_idx) {
                    // This button affects this joltage.
                    // Therefore the maximum number of presses for this button is bounded
                    // by this joltage target.
                    button_limits[button_idx] = button_limits[button_idx].min(target as i32);
                    1
                } else {
                    0
                };
                matrix.push(b);
            }
            matrix.push(target as i32);
        }
        debug_assert_eq!(matrix.len(), rows * cols);

        // Check if we can immediately spot a solution
        for row in 0..rows {
            if (0..(cols - 1))
                .map(|col| matrix[row * cols + col])
                .all(|e| e == 1)
            {
                let result = matrix[row * cols + cols - 1];
                debug_assert!(result >= 0);
                // FIXME: Something very weird is going on with these `println`s. Two of them cause the program
                // to hang if they are uncommented, but one of them is fine.
                // Doesn't matter if the debug assertions cfg is there or not. Doesn't matter if we spawn
                // one thread or 16. Doesn't matter if it's release mode or debug.
                // This one is one that does cause issues.
                // #[cfg(debug_assertions)]
                // println!("Found a fast solution for machine: `{result}`. Yay! :)");
                return result as usize;
            }
        }
        // TODO: do some more early checks on the matrix to see if we can immediately
        // compute a result, instead of doing the gaussian elimination etc.

        // See: `https://en.wikipedia.org/wiki/Gaussian_elimination`
        // Column indexes of free variables (previous iteration and current one)
        let mut free_vars_prev: BTreeSet<_> = (0..cols - 1).collect();
        let mut free_vars_curr = BTreeSet::new();

        // Iterate until free variables no longer change
        while free_vars_prev != free_vars_curr {
            free_vars_prev = free_vars_curr;
            free_vars_curr = (0..cols - 1).collect();

            let mut pivot_row = 0;
            let mut pivot_col = 0;
            while pivot_row < rows && pivot_col < cols - 1 {
                // Find pivot
                let found_pivot = {
                    let mut result = None;
                    for row in pivot_row..rows {
                        let c = matrix[row * cols + pivot_col];
                        // Make sure we keep integers
                        if c != 0
                            && matrix[(row * cols)..(row * cols + cols)]
                                .iter()
                                .all(|e| e % c == 0)
                        {
                            result = Some(row);
                            break;
                        }
                    }

                    result
                };
                let Some(found_pivot) = found_pivot else {
                    // No pivot in this column. Move to next one.
                    pivot_col += 1;
                    continue;
                };

                // Swap rows
                for col in 0..cols {
                    matrix.swap(pivot_row * cols + col, found_pivot * cols + col);
                }

                // Set pivot element to 1
                let factor = matrix[pivot_row * cols + pivot_col];
                for col in 0..cols {
                    matrix[pivot_row * cols + col] /= factor;
                }

                for row in 0..rows {
                    if row == pivot_row {
                        continue;
                    }

                    let factor = matrix[row * cols + pivot_col];
                    // Subtract multiplied pivot row from current row.
                    for col in 0..cols {
                        matrix[row * cols + col] -= matrix[pivot_row * cols + col] * factor;
                    }
                }

                free_vars_curr.remove(&pivot_col);
                pivot_col += 1;
                pivot_row += 1;
            }
        }

        if free_vars_curr.is_empty() {
            // All variables are fully determined. We can return.
            let sum = (0..rows)
                .map(|row| matrix[row * cols + cols - 1])
                .sum::<i32>();
            debug_assert!(sum >= 0);
            // FIXME: Something very weird is going on with these `println`s. Two of them cause the program
            // to hang if they are uncommented, but one of them is fine.
            // Doesn't matter if the debug assertions cfg is there or not. Doesn't matter if we spawn
            // one thread or 16. Doesn't matter if it's release mode or debug.
            // This one is the println that seemingly doesn't cause issues.
            #[cfg(debug_assertions)]
            println!("Found a result for machine without brute force: `{sum}`. Yay! :)");
            return sum as usize;
        }

        // There are some free variables left.
        // We need to search for their values.
        let num_free = free_vars_curr.len();
        let num_fixed = cols - 1 - num_free;
        let presses = (0..num_fixed)
            .map(|row| matrix[row * cols + cols - 1])
            .sum::<i32>();
        // We make some space to keep the parts of our (in)equalities in.
        let mut free_var_coefficients = vec![0; rows * num_free];
        let mut rhs = vec![0; rows * num_free];
        // The maximum number of times the buttons corresponding to the
        // free variables can be pressed.
        let mut free_limits = vec![0; num_free];
        // The costs (i.e., influence on total button presses) of
        // pressing a button corresponding to a free variable.
        let mut free_button_costs = vec![0; num_free];

        for (new_idx, &free_var_idx) in free_vars_curr.iter().enumerate() {
            // Copy the limits that we are interested in.
            free_limits[new_idx] = button_limits[free_var_idx];
            // Copy the cost for each free variable button.
            free_button_costs[new_idx] = 1
                - (0..num_fixed)
                    .map(|row| matrix[row * cols + free_var_idx])
                    .sum::<i32>();

            for row in 0..rows {
                // Copy the coefficients corresponding to each free variable over.
                free_var_coefficients[new_idx * rows + row] = matrix[row * cols + free_var_idx];
            }
        }

        for row in 0..rows {
            // Copy the right hand side over.
            rhs[row] = matrix[row * cols + cols - 1];
        }

        let solution = joltage_helper(
            num_free,
            num_fixed,
            rows,
            presses,
            &free_var_coefficients,
            &mut rhs,
            &free_limits,
            &free_button_costs,
            0,
        )
        .unwrap();
        debug_assert!(solution >= 0);
        // FIXME: Something very weird is going on with these `println`s. Two of them cause the program
        // to hang if they are uncommented, but one of them is fine.
        // Doesn't matter if the debug assertions cfg is there or not. Doesn't matter if we spawn
        // one thread or 16. Doesn't matter if it's release mode or debug.
        // This one is one that does cause issues.
        // #[cfg(debug_assertions)]
        // println!("Found solution for machine: {solution}");
        solution as usize
    }
}

#[expect(clippy::too_many_arguments)]
fn joltage_helper(
    num_free: usize,
    num_fixed: usize,
    rows: usize,
    presses: i32,
    coefficients: &[i32],
    rhs: &mut [i32],
    limits: &[i32],
    costs: &[i32],
    iteration: usize,
) -> Option<i32> {
    // Are we on the last free variable?
    if iteration == num_free - 1 {
        let mut lower_limit = 0;
        let mut upper_limit = limits[iteration];

        // Check if inequalities hold for chosen combination
        for (&c, &r) in coefficients[(iteration * rows)..(iteration * rows + rows)]
            .iter()
            .zip(&rhs[(iteration * rows)..(iteration * rows + rows)])
        {
            if r >= 0 {
                if c > 0 {
                    upper_limit = upper_limit.min(r / c);
                }
            } else if c < 0 {
                lower_limit = lower_limit.max((r + c + 1) / c);
            } else {
                upper_limit = -1;
            }
        }
        // Check equalities
        for row in num_fixed..rows {
            let c = coefficients[iteration * rows + row];
            let r = rhs[iteration * rows + row];

            if c != 0 {
                if r % c == 0 {
                    upper_limit = upper_limit.min(r / c);
                    lower_limit = lower_limit.max(r / c);
                } else {
                    upper_limit = -1;
                }
            }
        }

        let new_presses = if costs[iteration] >= 0 {
            presses + costs[iteration] * lower_limit
        } else {
            presses + costs[iteration] * upper_limit
        };

        if lower_limit <= upper_limit {
            Some(new_presses)
        } else {
            None
        }
    } else {
        let mut min: Option<i32> = None;
        // Try different numbers of presses for this button, up to its limit.
        for x in 0..=limits[iteration] {
            // Calculate influence on number of total button presses.
            let new_presses = presses + costs[iteration] * x;
            // Calculate a new RHS for (in)equalities.
            for col in 0..rows {
                rhs[(iteration + 1) * rows + col] =
                    rhs[iteration * rows + col] - coefficients[iteration * rows + col] * x;
            }

            // Recurse to iterate over values of other free variables.
            if let Some(result) = joltage_helper(
                num_free,
                num_fixed,
                rows,
                new_presses,
                coefficients,
                rhs,
                limits,
                costs,
                iteration + 1,
            ) {
                if min.is_some() {
                    min = min.map(|min| min.min(result));
                } else {
                    min = Some(result);
                }
            }
        }

        min
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
