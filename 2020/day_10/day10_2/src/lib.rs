fn matrix_mul(lhs: Vec<Vec<u64>>, rhs: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut result = lhs.clone();

    for i in 0..lhs.len() {
        for j in 0..lhs[i].len() {
            let mut sum = 0;
            for k in 0..lhs.len() {
                sum += lhs[i][k] * rhs[k][j];
            }
            result[i][j] = sum;
        }
    }

    result
}

#[test]
fn matrix_mul_test() {
    let m = vec![vec![0, 1, 1], vec![0, 0, 1], vec![0, 0, 0]];
    let k = m.clone();

    let expected = vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]];
    let result = matrix_mul(m, &k);
    assert_eq!(expected, result);
}

pub fn solve(input: &str) -> u64 {
    let mut adapters: Vec<u64> = input
        .trim_end()
        .lines()
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect();
    adapters.push(0);
    adapters.sort();
    // hack for the windows
    adapters.push(adapters[adapters.len() - 1]);
    adapters.push(adapters[adapters.len() - 1]);
    adapters.push(adapters[adapters.len() - 1]);

    // build adjacency matrix
    let mut adj = Vec::with_capacity(adapters.len());
    &adapters.windows(4).enumerate().for_each(|(i, window)| {
        let mut row = Vec::new();
        row.resize_with(adapters.len(), Default::default);
        for j in 1..=3 {
            if window[j] - window[0] <= 3 {
                row[i + j] = 1;
            } else {
                break;
            }
        }
        adj.push(row);
    });

    // multiply
    // TODO: This is super inefficent. `adj` is a strictly upper triangular matrix, there should be way to make this better
    let original = adj.clone();
    let mut paths = adj[0][adj[0].len() - 1];
    for i in 0..adapters.len() {
        println!(
            "{:3}% done ({} out of {})",
            (i * 100) / adapters.len(),
            i,
            adapters.len()
        );
        adj = matrix_mul(adj, &original);
        paths += adj[0][adj[0].len() - 1];
    }

    paths
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
