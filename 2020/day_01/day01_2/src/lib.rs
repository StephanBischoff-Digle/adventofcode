use std::cmp::Ordering;

/// Solves the 2020 Day01 Problem 1.
///
/// The `input` is expected to be the lines of the input file without
/// the last empty line. The `target` argument is the number the solver
/// tries to find a match for. The result is the product of the maching
/// entries that add up to the `target` value, if it can be found, otherwise
/// `None` is returned.
///
/// How it is solved:
/// 1. Parse the input lines to `u64`
/// 2. Sort the resulting `Vec<u64>`
/// 3. Use divide-and-conquer to find the matching entries
/// 4. Return `None` if we the indices pass each other or break and return
///    the product of the two elements the indices point at.
pub fn solve(input: &Vec<&str>, target: u64) -> Option<u64> {
    let mut parsed: Vec<u64> = input
        .iter()
        .map(|v| v.parse::<u64>().expect("Unexpected Input"))
        .collect();

    parsed.sort();
    let mut high_idx = parsed.len() - 1;
    loop {
        while (parsed[high_idx] + parsed[0]) >= target {
            high_idx -= 1;
        }

        for i in 0..high_idx {
            let mut l = i + 1;
            let mut r = high_idx;

            while l < r {
                match (parsed[i] + parsed[l] + parsed[r]).cmp(&target) {
                    Ordering::Equal => return Some(parsed[i] * parsed[l] * parsed[r]),
                    Ordering::Greater => r -= 1,
                    Ordering::Less => l += 1,
                }
            }
        }
        return None;
    }
}

#[test]
fn aoc_example() {
    let input = vec!["1721", "979", "366", "299", "675", "1456"];
    let output = solve(&input, 2020);

    assert_eq!(Some(241861950), output);
}
