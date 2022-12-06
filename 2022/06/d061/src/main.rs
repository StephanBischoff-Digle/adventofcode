use std::{collections::BTreeSet, fs};

fn all_different(lst: &[char]) -> bool {
    BTreeSet::<&char>::from_iter(lst).len() == lst.len()
}

fn main() {
    let Ok(input) = fs::read_to_string("input.txt") else {
        eprintln!("Failed to read input file");
        return;
    };

    let line: Vec<char> = input.trim_end().chars().collect();
    let marker = &line.windows(4).position(all_different);
    if let Some(marker) = marker {
        println!("{}", marker + 4);
    } else {
        eprintln!("Couldn't find a marker!");
    }
}
