use std::collections::HashSet;

pub fn solve_puzzle_a(input: &str) -> usize {
    let mut manifold: Manifold = input.into();

    manifold.propagate(manifold.start)
}

#[derive(Debug)]
struct Manifold {
    start: (usize, usize),
    splitters: HashSet<(usize, usize)>,
    beams: HashSet<(usize, usize)>,
    max_x: usize,
    max_y: usize,
}

impl Manifold {
    fn propagate(&mut self, from: (usize, usize)) -> usize {
        if from.1 + 1 >= self.max_y {
            return 0;
        }

        let next = (from.0, from.1 + 1);
        if !self.splitters.contains(&next) {
            if self.beams.insert(next) {
                self.propagate(next)
            } else {
                0
            }
        } else {
            let mut sum = 1;
            if let Some(x) = next.0.checked_sub(1) {
                let left = (x, next.1);
                if self.beams.insert(left) {
                    sum += self.propagate(left);
                }
            }
            if next.0 + 1 < self.max_x {
                let right = (next.0 + 1, next.1);
                if self.beams.insert(right) {
                    sum += self.propagate(right);
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve_puzzle_a;

    #[test]
    fn example() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............\n";

        assert_eq!(solve_puzzle_a(input), 21);
    }
}
