#[derive(Debug, Copy, Clone, PartialEq, Default)]
enum Tile {
    #[default]
    Empty = 0,
    Paper = 1,
}

pub fn solve_puzzle_a(input: &str) -> usize {
    let mut accessible = 0;
    let mut current = Vec::new();
    let mut previous = Vec::new();
    let mut next = Vec::new();
    let mut lines = input.lines();
    parse_line(lines.next().unwrap(), &mut previous);
    parse_line(lines.next().unwrap(), &mut current);
    accessible += evaluate_line(&previous, &next, &current);
    for line in lines {
        parse_line(line, &mut next);
        accessible += evaluate_line(&current, &previous, &next);
        std::mem::swap(&mut previous, &mut current);
        std::mem::swap(&mut current, &mut next);
        next.clear();
    }
    accessible += evaluate_line(&current, &previous, &next);

    accessible
}

fn evaluate_line(current: &[Tile], previous: &[Tile], next: &[Tile]) -> usize {
    let mut accessible = 0;
    for (idx, &tile) in current.iter().enumerate() {
        if Tile::Empty == tile {
            continue;
        }

        let mut neighbors = 0;
        if let Some(before) = idx.checked_sub(1) {
            neighbors += current.get(before).map(|t| *t as u8).unwrap_or_default()
                + previous.get(before).map(|t| *t as u8).unwrap_or_default()
                + next.get(before).map(|t| *t as u8).unwrap_or_default();
        }
        neighbors += current.get(idx + 1).map(|t| *t as u8).unwrap_or_default()
            + previous.get(idx).map(|t| *t as u8).unwrap_or_default()
            + previous.get(idx + 1).map(|t| *t as u8).unwrap_or_default()
            + next.get(idx).map(|t| *t as u8).unwrap_or_default()
            + next.get(idx + 1).map(|t| *t as u8).unwrap_or_default();

        if neighbors < 4 {
            accessible += 1;
        }
    }

    accessible
}

fn parse_line(line: &str, out: &mut Vec<Tile>) {
    for c in line.chars() {
        match c {
            '.' => out.push(Tile::Empty),
            '@' => out.push(Tile::Paper),
            _ => panic!("unknown tile"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve_puzzle_a;

    #[test]
    fn example() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n";

        assert_eq!(solve_puzzle_a(input), 13);
    }
}
