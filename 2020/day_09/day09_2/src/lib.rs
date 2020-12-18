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

    let failure = *failures[0];

    for i in 0..lines.len() {
        let mut sum = 0;
        let mut smallest = lines[i];
        let mut largest = lines[i];
        for j in i..lines.len() {
            sum += lines[j];
            smallest = std::cmp::min(smallest, lines[j]);
            largest = std::cmp::max(largest, lines[j]);
            if sum > failure {
                break;
            }
            if sum == failure {
                return smallest + largest;
            }
        }
    }
    0
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
    let expected = 62;
    assert_eq!(expected, result);
}
