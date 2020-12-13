use day01_2::solve;
use std::fs;

fn main() {
    println!("Reading input");

    let input = fs::read_to_string("input").expect("Failed to read input file");
    let input: Vec<&str> = input.trim_end().split("\n").collect();

    if let Some(product) = solve(&input, 2020) {
        println!("Solution: {}", product);
    } else {
        println!("Could not find a match");
    }
}
