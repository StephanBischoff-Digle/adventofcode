use std::fs;

fn all_different(lst: &[char]) -> bool {
    let mut work = Vec::from(lst);
    work.sort();
    work.windows(2).all(|ab| ab[0] != ab[1])
}

fn main() {
    let Ok(input) = fs::read_to_string("input.txt") else {
        eprintln!("Failed to read input file");
        return;
    };

    let windowsize = 14;
    let line: Vec<char> = input.trim_end().chars().collect();
    let markers = &line
        .windows(windowsize)
        .enumerate()
        .filter_map(|(idx, window)| match window {
            win if all_different(win) => Some(idx + windowsize),
            _ => None,
        })
        .collect::<Vec<_>>();
    if let Some(marker) = markers.first() {
        println!("{}", marker);
    } else {
        eprintln!("Couldn't find a marker!");
    }
}
