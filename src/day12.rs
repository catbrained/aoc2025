pub fn solve_puzzle_a(input: &str) -> usize {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    parse(input, &mut shapes, &mut regions);

    regions
        .iter()
        .filter_map(|r| r.can_fit_presents(&shapes).then_some(()))
        .count()
}

fn parse(input: &str, shapes: &mut Vec<Present>, regions: &mut Vec<Region>) {
    let lines: Vec<_> = input.lines().collect();

    for i in 0..lines.len() {
        let line = lines[i];

        let Some((first, rest)) = line.split_once(':') else {
            continue;
        };
        if let Some((x, y)) = first.split_once('x') {
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            let mut presents = Vec::new();
            presents.extend(rest.split_whitespace().map(|c| c.parse::<usize>().unwrap()));
            let region = Region { x, y, presents };
            regions.push(region);
        } else {
            let id: usize = first.parse().unwrap();
            let mut data = Vec::new();
            let mut width = 0;
            let mut height = 0;
            let mut fields_covered = 0;
            for line in lines[i + 1..].iter().take_while(|l| !l.is_empty()) {
                height += 1;
                data.extend(line.chars().map(|c| {
                    width += 1;
                    if c == '.' {
                        0
                    } else {
                        fields_covered += 1;
                        1
                    }
                }));
            }
            let present = Present {
                _id: id,
                width,
                height,
                fields_covered,
                _data: data,
            };
            shapes.push(present);
        }
    }
}

#[derive(Debug)]
struct Present {
    _id: usize,
    width: usize,
    height: usize,
    fields_covered: usize,
    _data: Vec<usize>,
}

#[derive(Debug)]
struct Region {
    x: usize,
    y: usize,
    presents: Vec<usize>,
}

impl Region {
    fn can_fit_presents(&self, present_shapes: &[Present]) -> bool {
        let (min_area_needed, max_area_needed): (usize, usize) = self
            .presents
            .iter()
            .zip(present_shapes)
            .map(|(count, shape)| {
                (
                    count * shape.fields_covered,
                    count * shape.width * shape.height,
                )
            })
            .reduce(|acc, elem| (acc.0 + elem.0, acc.1 + elem.1))
            .unwrap();

        if min_area_needed > self.x * self.y {
            // Can't fit all presents because their total area is too large!
            return false;
        }
        if max_area_needed <= self.x * self.y {
            // All presents trivially fit!
            return true;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::solve_puzzle_a;

    #[test]
    fn example() {
        let input = "0:\n###\n##.\n##.\n\n1:\n###\n##.\n.##\n\n2:\n.##\n###\n##.\n\n3:\n##.\n###\n##.\n\n4:\n###\n#..\n###\n\n5:\n###\n.#.\n###\n\n4x4: 0 0 0 0 2 0\n12x5: 1 0 1 0 2 2\n12x5: 1 0 1 0 3 2\n";

        assert_eq!(solve_puzzle_a(input), 2);
    }
}
