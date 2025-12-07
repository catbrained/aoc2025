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
    accessible += evaluate_line(&mut previous, &next, &current, false);
    for line in lines {
        parse_line(line, &mut next);
        accessible += evaluate_line(&mut current, &previous, &next, false);
        std::mem::swap(&mut previous, &mut current);
        std::mem::swap(&mut current, &mut next);
        next.clear();
    }
    accessible += evaluate_line(&mut current, &previous, &next, false);

    accessible
}

fn evaluate_line(current: &mut [Tile], previous: &[Tile], next: &[Tile], remove: bool) -> usize {
    let mut accessible = 0;
    for idx in 0..current.len() {
        if Tile::Empty == current[idx] {
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
            if remove {
                current[idx] = Tile::Empty;
            }
        }
    }

    accessible
}

pub fn solve_puzzle_b(input: &str) -> usize {
    let mut removed = 0;
    let mut lines = Vec::new();
    for line in input.lines() {
        let mut tiles = Vec::with_capacity(line.len());
        parse_line(line, &mut tiles);
        lines.push(tiles);
    }
    let len = lines.len();

    loop {
        let mut removed_this_pass = 0;
        let (current, rest) = lines.split_at_mut(1);
        removed_this_pass += evaluate_line(&mut current[0], &[], &rest[0], true);
        for idx in 1..(len - 1) {
            let (previous, rest) = lines.split_at_mut(idx);
            let (current, rest) = rest.split_at_mut(1);
            removed_this_pass +=
                evaluate_line(&mut current[0], previous.last().unwrap(), &rest[0], true);
        }
        let (previous, current) = lines.split_at_mut(len - 1);
        removed_this_pass += evaluate_line(&mut current[0], previous.last().unwrap(), &[], true);

        removed += removed_this_pass;
        if removed_this_pass == 0 {
            break;
        }
    }

    removed
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
    use super::{solve_puzzle_a, solve_puzzle_b};

    #[test]
    fn example() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n";

        assert_eq!(solve_puzzle_a(input), 13);
    }

    #[test]
    fn example2() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n";

        assert_eq!(solve_puzzle_b(input), 43);
    }
}
