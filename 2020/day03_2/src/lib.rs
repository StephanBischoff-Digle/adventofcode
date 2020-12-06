pub mod error;
pub mod field;
pub mod line;

use crate::field::Field;
use std::str::FromStr;

pub fn solve(input: &str, steps: Vec<(usize, usize)>) -> usize {
    let mut result = 1;
    if let Ok(field) = Field::from_str(input) {
        for (dx, dy) in steps.iter() {
            result *= field.iter(*dx, *dy).filter(|x| *x).count();
        }
        return result;
    }
    0
}

#[test]
fn aoc_example() {
    let input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    let steps = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let expected = 336;
    let result = solve(input, steps);
    assert_eq!(expected, result);
}
