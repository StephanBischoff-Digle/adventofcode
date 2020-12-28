pub fn solve(input: &str) -> u64 {
    let mut adapters: Vec<u64> = input
        .trim_end()
        .lines()
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect();
    adapters.sort_unstable();

    let lowest = adapters[0];
    let l_initial = if lowest == 1 { 1 } else { 0 };
    let h_initial = 1 + if lowest == 3 { 1 } else { 0 };
    let (l, h) = &adapters
        .windows(2)
        .fold((l_initial, h_initial), |acc, d| match d[1] - d[0] {
            1 => (acc.0 + 1, acc.1),
            3 => (acc.0, acc.1 + 1),
            _ => (acc.0, acc.1),
        });

    l * h
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
    let expected = 7 * 5;
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
    let expected = 22 * 10;
    let result = solve(input);
    assert_eq!(expected, result);
}
