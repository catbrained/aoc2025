use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct Pair<'a> {
    a: &'a JunctionBox,
    b: &'a JunctionBox,
    dist: f64,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&str> for JunctionBox {
    fn from(value: &str) -> Self {
        let mut coords = value.split(',').map(|num| num.parse::<usize>().unwrap());

        Self {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        }
    }
}

impl JunctionBox {
    fn dist(&self, other: &Self) -> f64 {
        let x = self.x.abs_diff(other.x).pow(2);
        let y = self.y.abs_diff(other.y).pow(2);
        let z = self.z.abs_diff(other.z).pow(2);

        ((x + y + z) as f64).sqrt()
    }
}

pub fn solve_puzzle_a(input: &str) -> usize {
    solve_puzzle(input, 1000, 3, false)
}

pub fn solve_puzzle_b(input: &str) -> usize {
    solve_puzzle(input, usize::MAX, usize::MAX, true)
}

fn solve_puzzle(input: &str, num_pairs: usize, num_circuits: usize, part_two: bool) -> usize {
    let mut junctions = Vec::new();
    for line in input.lines() {
        junctions.push(line.into());
    }
    let num_junctions = junctions.len();
    let mut pairs = build_pairs(&junctions);
    pairs.sort_unstable_by(|a, b| a.dist.total_cmp(&b.dist));
    let circuits = build_circuits(&pairs, num_pairs, num_junctions);

    if part_two {
        let Either::Part2(result) = circuits else {
            unreachable!();
        };
        result
    } else {
        let Either::Part1(mut circuits) = circuits else {
            unreachable!();
        };
        circuits.sort_unstable_by_key(|circ| circ.len());

        circuits
            .iter()
            .rev()
            .take(num_circuits)
            .map(|circ| circ.len())
            .product()
    }
}

fn build_pairs(junctions: &[JunctionBox]) -> Vec<Pair<'_>> {
    let mut pairs = Vec::new();
    for (idx, junction) in junctions.iter().enumerate() {
        for other in &junctions[idx + 1..] {
            let pair = Pair {
                a: junction,
                b: other,
                dist: junction.dist(other),
            };
            pairs.push(pair);
        }
    }

    pairs
}

fn build_circuits(pairs: &[Pair], num_pairs: usize, num_junctions: usize) -> Either {
    let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();
    let mut added_junctions = HashSet::new();
    for (a, b) in pairs.iter().take(num_pairs).map(|p| (p.a, p.b)) {
        let mut connected = circuits
            .iter_mut()
            .filter(|circ| circ.contains(a) || circ.contains(b));
        if let Some(circ) = connected.next() {
            let _ = circ.insert(*a);
            let _ = circ.insert(*b);
            let _ = added_junctions.insert(*a);
            let _ = added_junctions.insert(*b);
            for other in connected {
                circ.extend(other.drain());
            }
            if added_junctions.len() == num_junctions {
                debug_assert_eq!(circuits.iter().filter(|circ| !circ.is_empty()).count(), 1);
                return Either::Part2(a.x * b.x);
            }
        } else {
            let mut new_circ = HashSet::new();
            let _ = new_circ.insert(*a);
            let _ = new_circ.insert(*b);
            let _ = added_junctions.insert(*a);
            let _ = added_junctions.insert(*b);
            circuits.push(new_circ);
        }
    }

    Either::Part1(circuits)
}

#[derive(Debug)]
enum Either {
    Part1(Vec<HashSet<JunctionBox>>),
    Part2(usize),
}

#[cfg(test)]
mod tests {
    use super::{solve_puzzle, solve_puzzle_b};

    #[test]
    fn example() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689\n";

        assert_eq!(solve_puzzle(input, 10, 3, false), 40);
    }

    #[test]
    fn example2() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689\n";

        assert_eq!(solve_puzzle_b(input), 25272);
    }
}
