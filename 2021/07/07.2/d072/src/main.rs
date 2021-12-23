use std::fs;

fn find_min(lst: &[i32]) -> i32 {
    let mut start = *lst.iter().min().expect("Find Minimum in lst.");
    let mut prev = lst
            .iter()
            .map(|l| ((start - l).abs() * ((start - l).abs() + 1)) / 2)
            .sum();
    let mut current = prev;

    while prev >= current {
        start += 1;
        prev = current;
        current = lst
            .iter()
            .map(|l| ((start - l).abs() * ((start - l).abs() + 1)) / 2)
            .sum();
    }

    prev
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Read input.");
    let input: Vec<&str> = input.trim_end().split('\n').collect();
    let input: Vec<_> = input[0]
        .split(',')
        .map(|v| v.parse().expect("Parse input."))
        .collect();

    let solution = find_min(&input);
    println!("{}", solution);
}
