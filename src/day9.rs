#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value
            .split_once(',')
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .unwrap();

        Self { x, y }
    }
}

pub fn solve_puzzle_a(input: &str) -> usize {
    let mut points: Vec<Point> = Vec::new();
    for line in input.lines() {
        points.push(line.into());
    }
    let mut pairs = Vec::new();
    for (idx, point) in points.iter().enumerate() {
        for other in &points[idx + 1..] {
            pairs.push((point, other));
        }
    }

    pairs
        .iter()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::solve_puzzle_a;

    #[test]
    fn example() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n";

        assert_eq!(solve_puzzle_a(input), 50);
    }
}
