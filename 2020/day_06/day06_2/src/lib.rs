use std::collections::HashMap;

fn extract(group: &str) -> usize {
    let participants: Vec<&str> = group.trim_end().split('\n').collect();
    let mut hm = HashMap::new();

    for p in participants.iter() {
        for c in p.chars() {
            let v = hm.entry(c).or_insert(0);
            *v += 1;
        }
    }

    hm.iter()
        .filter(|(_, &v)| v == participants.len())
        .collect::<HashMap<_, _>>()
        .len()
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

    let expected = 6;
    let result = solve(input);
    assert_eq!(expected, result);
}
