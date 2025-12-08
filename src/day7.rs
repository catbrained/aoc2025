use std::collections::{HashMap, HashSet};

pub fn solve_puzzle_a(input: &str) -> usize {
    let mut manifold: Manifold = input.into();

    manifold.propagate(manifold.start, false)
}

pub fn solve_puzzle_b(input: &str) -> usize {
    let mut manifold: Manifold = input.into();

    manifold.propagate(manifold.start, true)
}

#[derive(Debug)]
struct Manifold {
    start: (usize, usize),
    splitters: HashSet<(usize, usize)>,
    beams: HashSet<(usize, usize)>,
    max_x: usize,
    max_y: usize,
    memo: HashMap<(usize, usize), usize>,
}

impl Manifold {
    fn propagate(&mut self, from: (usize, usize), many_worlds: bool) -> usize {
        if let Some(result) = self.memo.get(&from) {
            return *result;
        }
        if from.1 + 1 >= self.max_y {
            if many_worlds {
                return 1;
            } else {
                return 0;
            }
        }

        let next = (from.0, from.1 + 1);
        if !self.splitters.contains(&next) {
            if self.beams.insert(next) || many_worlds {
                let result = self.propagate(next, many_worlds);
                let _ = self.memo.insert(next, result);
                result
            } else {
                0
            }
        } else {
            let mut sum = if many_worlds { 0 } else { 1 };
            if let Some(x) = next.0.checked_sub(1) {
                let left = (x, next.1);
                if self.beams.insert(left) || many_worlds {
                    let result = self.propagate(left, many_worlds);
                    let _ = self.memo.insert(left, result);
                    sum += result;
                }
            }
            if next.0 + 1 < self.max_x {
                let right = (next.0 + 1, next.1);
                if self.beams.insert(right) || many_worlds {
                    let result = self.propagate(right, many_worlds);
                    let _ = self.memo.insert(right, result);
                    sum += result;
                }
            }

            sum
        }
    }
}

impl From<&str> for Manifold {
    fn from(value: &str) -> Self {
        let mut splitters = HashSet::new();
        let mut start = (0, 0);
        let mut max_y = 0;
        let mut max_x = 0;
        for (y, line) in value.lines().enumerate() {
            max_y += 1;
            max_x = 0;
            for (x, b) in line.bytes().enumerate() {
                max_x += 1;
                match b {
                    b'.' => continue,
                    b'S' => start = (x, y),
                    b'^' => {
                        let _ = splitters.insert((x, y));
                    }
                    _ => panic!("unexpected tile"),
                }
            }
        }

        Self {
            start,
            splitters,
            beams: HashSet::new(),
            max_x,
            max_y,
            memo: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_puzzle_a, solve_puzzle_b};

    #[test]
    fn example() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............\n";

        assert_eq!(solve_puzzle_a(input), 21);
    }

    #[test]
    fn example2() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............\n";

        assert_eq!(solve_puzzle_b(input), 40);
    }
}
