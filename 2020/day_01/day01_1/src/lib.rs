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
    let mut low_idx = 0;
    let mut high_idx = parsed.len() - 1;
    loop {
        match (parsed[low_idx] + parsed[high_idx]).cmp(&target) {
            Ordering::Less => low_idx += 1,
            Ordering::Greater => high_idx -= 1,
            Ordering::Equal => break,
        };

        if high_idx <= low_idx {
            return None;
        }
    }

    Some(parsed[low_idx] * parsed[high_idx])
}

#[test]
fn aoc_example() {
    let input = vec!["1721", "979", "366", "299", "675", "1456"];
    let output = solve(&input, 2020);

    assert_eq!(Some(514579), output);
}
