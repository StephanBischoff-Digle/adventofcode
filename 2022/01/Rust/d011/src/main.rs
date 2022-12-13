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

    let max_cal = input.iter().map(|elf| elf.iter().sum::<u32>()).max();
    if let Some(answer) = max_cal {
        println!("{}", answer);
    } else {
        eprintln!("Failed to find max!");
    }
}
