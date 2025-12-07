pub fn solve_puzzle_a(input: &str) -> usize {
    let mut lines = input.lines().rev();
    let mut columns = Vec::new();
    let ops: Vec<_> = lines
        .next()
        .map(|s| {
            s.split_whitespace().map(|o| match o.as_bytes()[0] {
                b'+' => {
                    columns.push(0);
                    |a: &mut usize, b| {
                        *a += b;
                    }
                }
                b'*' => {
                    columns.push(1);
                    |a: &mut usize, b| {
                        *a *= b;
                    }
                }
                _ => panic!("unexpected operation"),
            })
        })
        .unwrap()
        .collect();
    let map = lines.map(|l| {
        l.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .zip(ops.clone())
            .enumerate()
    });
    for (idx, (num, op)) in map.flatten() {
        op(&mut columns[idx], num);
    }

    columns.iter().sum()
}

pub fn solve_puzzle_b(input: &str) -> usize {
    let mut total = 0;
    let mut lines: Vec<_> = input.lines().collect();
    let line_len = lines[0].len();
    let mut ops = lines.pop().unwrap().split_whitespace();
    let mut numbers = Vec::new();
    for idx in 0..line_len {
        let mut current = String::new();
        for line in lines.iter() {
            current.push(line.as_bytes()[idx].into());
        }
        if let Ok(num) = current.trim().parse::<usize>() {
            numbers.push(num);
        } else {
            match ops.next().unwrap().as_bytes()[0] {
                b'+' => total += numbers.iter().sum::<usize>(),
                b'*' => total += numbers.iter().product::<usize>(),
                _ => panic!("unknown operation"),
            }
            numbers.clear();
        }
    }
    match ops.next().unwrap().as_bytes()[0] {
        b'+' => total += numbers.iter().sum::<usize>(),
        b'*' => total += numbers.iter().product::<usize>(),
        _ => panic!("unknown operation"),
    }

    total
}

#[cfg(test)]
mod tests {
    use super::{solve_puzzle_a, solve_puzzle_b};

    #[test]
    fn example() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

        assert_eq!(solve_puzzle_a(input), 4277556);
    }

    #[test]
    fn example2() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

        assert_eq!(solve_puzzle_b(input), 3263827);
    }
}
