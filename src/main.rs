use std::{
    fs::read_to_string,
    io::{Write, stdout},
};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let mut output = stdout().lock();

    // Day 1 - First puzzle
    writeln!(output, "=== Day 1 - First puzzle ===").unwrap();
    let input = read_to_string("./day_1_a_input.txt").unwrap();
    let result = day1::solve_puzzle_a(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 1 - Second puzzle
    writeln!(output, "=== Day 1 - Second puzzle ===").unwrap();
    // Uses the same input as the previous puzzle
    let result = day1::solve_puzzle_b(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 2 - First puzzle
    writeln!(output, "=== Day 2 - First puzzle ===").unwrap();
    let input = read_to_string("./day_2_a_input.txt").unwrap();
    let result = day2::solve_puzzle_a(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 2 - Second puzzle
    writeln!(output, "=== Day 2 - second puzzle ===").unwrap();
    // Uses the same input as the previous puzzle
    let result = day2::solve_puzzle_b(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 3 - First puzzle
    writeln!(output, "=== Day 3 - First puzzle ===").unwrap();
    let input = read_to_string("./day_3_a_input.txt").unwrap();
    let result = day3::solve_puzzle_a(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 3 - Second puzzle
    writeln!(output, "=== Day 3 - second puzzle ===").unwrap();
    // Uses the same input as the previous puzzle
    let result = day3::solve_puzzle_b(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 4 - First puzzle
    writeln!(output, "=== Day 4 - First puzzle ===").unwrap();
    let input = read_to_string("./day_4_a_input.txt").unwrap();
    let result = day4::solve_puzzle_a(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 4 - Second puzzle
    writeln!(output, "=== Day 4 - second puzzle ===").unwrap();
    // Uses the same input as the previous puzzle
    let result = day4::solve_puzzle_b(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 5 - First puzzle
    writeln!(output, "=== Day 5 - First puzzle ===").unwrap();
    let input = read_to_string("./day_5_a_input.txt").unwrap();
    let result = day5::solve_puzzle_a(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 5 - Second puzzle
    writeln!(output, "=== Day 5 - second puzzle ===").unwrap();
    // Uses the same input as the previous puzzle
    let result = day5::solve_puzzle_b(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 6 - First puzzle
    writeln!(output, "=== Day 6 - First puzzle ===").unwrap();
    let input = read_to_string("./day_6_a_input.txt").unwrap();
    let result = day6::solve_puzzle_a(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 6 - Second puzzle
    writeln!(output, "=== Day 6 - second puzzle ===").unwrap();
    // Uses the same input as the previous puzzle
    let result = day6::solve_puzzle_b(&input);
    writeln!(output, "Solution: `{result}`").unwrap();

    // Day 7 - First puzzle
    writeln!(output, "=== Day 7 - First puzzle ===").unwrap();
    let input = read_to_string("./day_7_a_input.txt").unwrap();
    let result = day7::solve_puzzle_a(&input);
    writeln!(output, "Solution: `{result}`").unwrap();
}
