use std::fs;

fn read_input(filename: &str) -> Vec<Vec<u32>> {
    let input = fs::read_to_string(filename).expect(&format!("Reading {}", filename));
    let mut ret = Vec::new();
    let mut current_vec = Vec::new();
    for line in input.trim_end().split('\n') {
        if line.is_empty() {
            ret.push(current_vec.clone());
            current_vec.clear();
        } else {
            let parsed = line.parse().expect(&format!("Parsing {}", line));
            current_vec.push(parsed);
        }
    }
    ret.push(current_vec);
    ret
}

fn main() {
    let input = read_input("input.txt");

    let mut total_cals: Vec<u32> = input.iter().map(|elf| elf.iter().sum::<u32>()).collect();
    total_cals.sort();

    if total_cals.len() >= 3 {
        let three_elve_total: u32 = total_cals.iter().rev().take(3).sum();
        println!("{}", three_elve_total);
    } else {
        eprintln!("Not enough elves!");
    }
}
