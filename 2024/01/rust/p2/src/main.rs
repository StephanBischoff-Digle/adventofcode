use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<(), String> {
    let cli = Args::parse();

    let mut buffer = String::new();
    let Ok(mut file) = File::open(cli.input) else {
        return Err("Failed to open input file".to_string());
    };

    let Ok(_) = file.read_to_string(&mut buffer) else {
        return Err("Failed to read input file".to_string());
    };

    // Read the file, split by newline and remove the last, empty, line.
    let lines: Vec<_> = buffer.lines().filter(|line| !line.is_empty()).collect();

    // Convert the lines into two lists: left and right.
    let (left, right): (Vec<u64>, Vec<u64>) = lines.iter().map(|&line| split_to_pair(line)).unzip();

    // Construct a map of values and their counts from the right list
    let mut counts: HashMap<u64, u64> = HashMap::new();
    for item in right {
        let count = counts.entry(item).or_insert(0);
        *count += 1;
    }

    // Calculate the solution
    let solution: u64 = left
        .iter()
        .map(|x| x * counts.get(x).unwrap_or(&0u64))
        .sum();

    println!("{solution}");

    Ok(())
}

/// Splits the input line and parses it into a tuple of u64
///
/// ## Example
///
/// ```rust
/// assert_eq(split_to_pair("1   2"), (1, 2));
/// ```
fn split_to_pair(line: &str) -> (u64, u64) {
    let v: Vec<u64> = line
        .split("   ")
        .map(|x| x.parse().expect("Failed to parse"))
        .collect();
    (v[0], v[1])
}
