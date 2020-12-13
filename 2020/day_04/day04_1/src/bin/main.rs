use day04_1::solve;
use std::fs;

fn main() {
    println!("Reading input");

    let input = fs::read_to_string("input").expect("Failed to read input file");

    println!("Solution: {}", solve(&input));
}
