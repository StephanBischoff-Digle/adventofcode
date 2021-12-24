use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Read input.txt");

    let lst = input
        .lines()
        .map(|line| line.split(" | ").collect::<Vec<_>>()[1]);

    let solution: i32 = lst
        .map(|line| {
            {
                line.split(' ')
                    .map(|x| match x.len() {
                        2 | 3 | 4 | 7 => 1,
                        _ => 0,
                    })
                    .sum::<i32>()
            }
        })
        .sum();

    println!("{}", solution);
}
