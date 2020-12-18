pub fn solve(input: &str, preamble: usize) -> u64 {
    let lines: Vec<_> = input
        .trim_end()
        .split('\n')
        .map(|x| x.parse::<u64>().expect("Failed to parse input"))
        .collect();

    let failures = &lines
        .windows(preamble + 1)
        .filter_map(|xs| {
            let target_sum = xs.last().expect("Something went wrong ... ");
            let mut match_found = false;
            'outer: for i in 0..xs.len() - 1 {
                for j in i + 1..xs.len() - 1 {
                    if xs[i] + xs[j] == *target_sum {
                        match_found = true;
                        break 'outer;
                    }
                }
            }
            if match_found {
                return None;
            }
            Some(target_sum)
        })
        .collect::<Vec<_>>();

    *failures[0]
}

#[test]
fn example() {
    let input = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let result = solve(input, 5);
    let expected = 127;
    assert_eq!(expected, result);
}
