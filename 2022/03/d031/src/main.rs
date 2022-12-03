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

    let result = input
        .lines()
        .map(|line| {
            // We pray to god that there are only ASCII chars in there
            // TODO(stbf): make save for UTF-8
            let (first_s, second_s) = line.split_at(line.len() / 2);
            let first: HashSet<u32> = first_s.chars().map(char_to_val).collect();
            let second: HashSet<u32> = second_s.chars().map(char_to_val).collect();
            let mut inter = first.intersection(&second);
            if let Some(n) = inter.next() {
                return n.clone();
            } else {
                panic!("First and Second are disjunkt!");
            }
        })
        .sum::<u32>();
    println!("{}", result);
}
