use std::{
    fs::read_to_string,
    io::{Write, stdout},
};

mod day1;

fn main() {
    let mut output = stdout().lock();

    // Day 1 - First puzzle
    writeln!(output, "=== Day 1 - First puzzle ===").unwrap();
    let input = read_to_string("./day_1_a_input.txt").unwrap();
    let result = day1::solve_puzzle_a(&input);
    writeln!(output, "Solution: `{result}`").unwrap();
}
