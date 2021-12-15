use std::fs;

fn main() {
    println!("Reading input");

    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let input: Vec<u32> = input
        .trim_end()
        .split('\n')
        .map(|l| l.parse::<u32>().expect("Failed to parse"))
        .collect();

    let result = &input.windows(4).filter(|ab| ab[0] < ab[3]).count();

    println!("{}", result);
}
