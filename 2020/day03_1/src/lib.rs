pub mod error;
pub mod field;
pub mod line;

use crate::field::Field;
use std::str::FromStr;

pub fn solve(input: &str, dx: usize, dy: usize) -> usize {
    if let Ok(field) = Field::from_str(input) {
        return field.iter(dx, dy).filter(|x| *x).count();
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

    let field = Field::from_str(input).unwrap();
    let result = field.iter(3, 1).filter(|x| *x).count();

    let expected = 7;
    assert_eq!(expected, result);
}
