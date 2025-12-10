use std::collections::{HashSet, VecDeque};

pub fn solve_puzzle_a(input: &str) -> usize {
    let machines = input.lines().map(|l| l.into());

    machines.map(|m: Machine| m.configure()).sum()
}

type Button = HashSet<usize>;

#[derive(Debug)]
struct Machine {
    target: HashSet<usize>,
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
}

#[derive(Debug)]
struct Node {
    presses: usize,
    state: HashSet<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut target = HashSet::new();
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
                b'{' => continue,
                _ => panic!("unknown element"),
            }
        }

        Self { target, buttons }
    }
}

#[cfg(test)]
mod tests {
    use super::{Machine, solve_puzzle_a};

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
}
