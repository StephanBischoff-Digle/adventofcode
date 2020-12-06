use day03_2::solve;
use std::fs;

fn main() {
    println!("Reading input");

    let input = fs::read_to_string("input").expect("Failed to read input file");
    let steps = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    println!("Solution: {}", solve(&input, steps));
}
