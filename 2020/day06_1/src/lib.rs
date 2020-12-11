use std::collections::HashSet;

fn extract(group: &str) -> usize {
    let mut all = group.to_owned();
    all.retain(|c| !c.is_whitespace());

    let mut hs = HashSet::new();

    for c in all.chars() {
        hs.insert(c);
    }
    hs.len()
}

pub fn solve(input: &str) -> usize {
    input
        .trim_end()
        .split("\n\n")
        .fold(0, |acc, x| acc + extract(x))
}

#[test]
fn example() {
    let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    let expected = 11;
    let result = solve(input);
    assert_eq!(expected, result);
}
