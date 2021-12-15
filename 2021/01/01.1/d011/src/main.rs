use std::fs;

fn main() {
    println!("Reading input");

    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let input: Vec<u32> = input
        .trim_end()
        .split('\n')
        .map(|l| l.parse::<u32>().expect("Failed to parse"))
        .collect();

    let result = &input
        .windows(2)
        .filter_map(|ab| (ab[0] < ab[1]).then(|| {}))
        .count();

    println!("{}", result);
}
