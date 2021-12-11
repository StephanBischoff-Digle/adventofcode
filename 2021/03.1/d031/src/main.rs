use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Read from input.txt");
    let input: Vec<&str> = input.trim_end().split("\n").collect();

    let mut acc: HashMap<usize, u32> = HashMap::new();

    // Count bits
    for line in &input {
        line.chars().enumerate().for_each(|(idx, c)| {
            let entry = acc.entry(idx).or_default();
            *entry += match c {
                '1' => 1,
                _ => 0,
            }
        });
    }

    let l = input
        .len()
        .try_into()
        .expect("Convert number of input line unto u32");
    let mut gamma = 0;
    let mut mask = 0;
    for (k, v) in acc.iter() {
        let d = if 2 * v < l { 0 } else { 1 };
        gamma += d << (acc.len() - k - 1);
        mask += 1 << (acc.len() - k - 1);
    }

    let epsilon = gamma ^ mask;
    println!("{}", epsilon * gamma);
}
