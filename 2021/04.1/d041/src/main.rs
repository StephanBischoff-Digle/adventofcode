use std::fs;

mod structs;

fn check_winner(boards: &Vec<structs::Board>) -> Option<usize> {
    for (idx, v) in boards.iter().map(|board| board.check_bingo()).enumerate() {
        if v {
            return Some(idx);
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Read from input.txt");
    let input: Vec<&str> = input.trim_end().split("\n").collect();

    let seq: Vec<i32> = input[0]
        .split(",")
        .map(|s| s.parse().expect("Parse input sequence."))
        .collect();

    let mut boards = Vec::new();
    let mut collector = Vec::new();
    for line in input.iter().skip(2) {
        if line.is_empty() {
            boards.push(structs::Board::from(collector));
            collector = Vec::new();
        } else {
            collector.push(line);
        }
    }
    boards.push(structs::Board::from(collector));

    let mut last = 0;
    let mut winner = None;
    for n in seq {
        boards.iter_mut().for_each(|board| board.call_nr(n));
        if let Some(w) = check_winner(&boards) {
            last = n;
            winner = Some(w);
            break;
        }
    }

    if let Some(w) = winner {
        let solution = last * boards[w].get_unmarked().iter().sum::<i32>();
        println!("{}", solution);
    } else {
        println!("Failed");
    }
}
