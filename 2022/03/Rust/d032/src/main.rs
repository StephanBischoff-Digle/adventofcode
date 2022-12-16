use std::{collections::HashSet, fs};

fn char_to_val(c: char) -> u32 {
    let la_code = 'a' as u32;
    let ua_code = 'A' as u32;
    let c_code = c as u32;

    if c.is_uppercase() {
        return c_code - ua_code + 27;
    } else {
        return c_code - la_code + 1;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Reading file!");

    let conv: Vec<HashSet<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let result: u32 = conv
        .chunks(3)
        .map(|triple| {
            let a = &triple[0];
            let b = &triple[1];
            let c = &triple[2];
            let intersection = &(a & b) & c;
            if let Some(n) = intersection.into_iter().next() {
                return char_to_val(n);
            } else {
                panic!("Couldn't find badge! {:?}", triple);
            }
        })
        .sum();
    println!("{}", result);
}
