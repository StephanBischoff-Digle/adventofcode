use std::collections::HashMap;

pub fn solve(input: &str) -> u64 {
    let mut adapters: Vec<u64> = input
        .trim_end()
        .lines()
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect();
    adapters.sort_unstable();

    let max_jolts = adapters.last().unwrap_or(&0) + 3;
    adapters.push(max_jolts);

    let mut cache = HashMap::new();
    cache.insert(0, 1);

    for i in &adapters {
        let mut sum = 0;
        for j in 1..=3 {
            if i >= &j {
                sum += cache.get(&(i - j)).unwrap_or(&0);
            }
        }
        cache.insert(*i, sum);
    }

    *cache.get(&max_jolts).unwrap()
}


#[test]
fn example_a() {
    let input = "\
16
10
15
5
1
11
7
19
6
12
4";
    let expected = 8;
    let result = solve(input);
    assert_eq!(expected, result);
}

#[test]
fn example_b() {
    let input = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    let expected = 19208;
    let result = solve(input);
    assert_eq!(expected, result);
}
