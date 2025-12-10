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

pub fn solve_puzzle_b(input: &str) -> usize {
    let mut points: Vec<Point> = Vec::new();
    for line in input.lines() {
        points.push(line.into());
    }
    let mut pairs = Vec::new();
    let mut vertical_edges = Vec::new();
    let mut horizontal_edges = Vec::new();
    for (idx, point) in points.iter().enumerate() {
        let next = points[(idx + 1) % points.len()];
        if point.x == next.x {
            vertical_edges.push((point, next));
        } else {
            horizontal_edges.push((point, next));
        }
        for other in &points[idx + 1..] {
            pairs.push((point, other));
        }
    }
    pairs.sort_by(|a, b| {
        let area_a = (a.0.x.abs_diff(a.1.x) + 1) * (a.0.y.abs_diff(a.1.y) + 1);
        let area_b = (b.0.x.abs_diff(b.1.x) + 1) * (b.0.y.abs_diff(b.1.y) + 1);
        area_b.cmp(&area_a)
    });
    let mut max_area = 0;
    for (a, b) in pairs {
        let rect_top = a.y.min(b.y);
        let rect_bottom = a.y.max(b.y);
        let rect_left = a.x.min(b.x);
        let rect_right = a.x.max(b.x);
        if vertical_edges
            .iter()
            .filter(|&e| e.0.y.min(e.1.y) < rect_bottom && e.0.y.max(e.1.y) > rect_top)
            .any(|e| {
                !((rect_left <= e.0.x && rect_right <= e.0.x)
                    || (rect_left >= e.0.x && rect_right >= e.0.x))
            })
        {
            continue;
        }
        if horizontal_edges
            .iter()
            .filter(|&e| e.0.x.min(e.1.x) < rect_right && e.0.x.max(e.1.x) > rect_left)
            .any(|e| {
                !((rect_top <= e.0.y && rect_bottom <= e.0.y)
                    || (rect_top >= e.0.y && rect_bottom >= e.0.y))
            })
        {
            continue;
        }
        max_area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
        break;
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::{solve_puzzle_a, solve_puzzle_b};

    #[test]
    fn example() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n";

        assert_eq!(solve_puzzle_a(input), 50);
    }

    #[test]
    fn example2() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n";

        assert_eq!(solve_puzzle_b(input), 24);
    }
}
