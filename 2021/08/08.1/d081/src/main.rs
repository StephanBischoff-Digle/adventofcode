use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Read input.txt");

    let lst: Vec<&str> = input
        .lines()
        .map(|line| line.split(" | ").collect::<Vec<_>>()[1])
        .collect();

    let easy_map = HashMap::from([(2, 1), (3, 1), (4, 1), (7, 1)]);

    let solution: i32 = lst
        .into_iter()
        .map(|line| {
            {
                line.split(' ')
                    .map(|x| match easy_map.get(&x.len()) {
                        Some(&v) => v,
                        None => 0,
                    })
                    .sum::<i32>()
            }
        })
        .sum();

    println!("{}", solution);
}
