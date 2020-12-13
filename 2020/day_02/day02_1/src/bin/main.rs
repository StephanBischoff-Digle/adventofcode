use day02_1::solve;
use std::fs;

fn main() {
    println!("Reading input");

    let input = fs::read_to_string("input").expect("Failed to read input file");
    let input: Vec<&str> = input.trim_end().split('\n').collect();

    println!("Solution: {}", solve(&input));
}
