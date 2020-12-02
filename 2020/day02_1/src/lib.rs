pub mod interval;
pub mod policy;

use std::str::FromStr;

pub fn solve(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|i| {
            let split: Vec<&str> = i.split(":").collect();
            let policy = policy::Policy::from_str(split[0]).unwrap();
            let data = split[1];
            (policy, data)
        })
        .filter(|(policy, data)| policy.apply(data))
        .count()
}

#[test]
fn aoc_example() {
    let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    assert_eq!(2, solve(&input));
}
