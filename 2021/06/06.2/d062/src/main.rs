use std::fs;

mod matrix;
use matrix::Matrix;

fn line_parser(lst: &[&str]) -> Matrix<i64, 9, 1> {
    let mut fishes = Matrix::default();
    lst[0]
        .split(',')
        .map(|s| s.parse().expect("Parse input."))
        .for_each(|v| fishes.set(v, 0, fishes.get(v, 0) + 1));

    fishes
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Read from input.txt");
    let input: Vec<&str> = input.trim_end().split('\n').collect();
    let fishes = line_parser(&input);

    let mut transform: Matrix<_, 9, 9> = Matrix::default();
    transform.set(0, 1, 1);
    transform.set(1, 2, 1);
    transform.set(2, 3, 1);
    transform.set(3, 4, 1);
    transform.set(4, 5, 1);
    transform.set(5, 6, 1);
    transform.set(6, 7, 1);
    transform.set(7, 8, 1);
    transform.set(6, 0, 1);
    transform.set(8, 0, 1);

    let f256 = transform.pow(256) * fishes;
    let solution = (0..9).map(|idx| f256.get(idx, 0)).sum::<i64>();
    println!("{}", solution);
}
