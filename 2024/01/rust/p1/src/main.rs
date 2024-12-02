use std::{fs::File, io::Read, path::PathBuf};

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

    let lines: Vec<_> = buffer.lines().filter(|line| !line.is_empty()).collect();
    let (mut left, mut right): (Vec<&str>, Vec<&str>) =
        lines.iter().map(|&line| split_to_pair(line)).unzip();

    left.sort();
    right.sort();

    let solution: u64 = left
        .iter()
        .zip(right)
        .map(|(a, b)| parse_to_diff(a, b))
        .sum();

    println!("{solution}");

    Ok(())
}

fn split_to_pair(line: &str) -> (&str, &str) {
    let v: Vec<_> = line.split("   ").collect();
    (v[0], v[1])
}

fn parse_to_diff(a: &str, b: &str) -> u64 {
    let a: u64 = a.parse().expect("failed to parse to u64");
    let b: u64 = b.parse().expect("failed to parse to u64");
    a.abs_diff(b)
}
