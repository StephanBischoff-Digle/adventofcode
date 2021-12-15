use std::fs;

mod structs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Read from input.txt");
    let input: Vec<&str> = input.trim_end().split('\n').collect();

    let seq: Vec<i32> = input[0]
        .split(',')
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
    let mut looser = None;
    for n in seq {
        boards.iter_mut().for_each(|board| board.call_nr(n));
        if boards.len() > 1 {
            boards = boards
                .into_iter()
                .filter(|board| !board.check_bingo())
                .collect();
        } else if boards[0].check_bingo() {
            last = n;
            looser = Some(&boards[0]);
            break;
        }
    }

    if let Some(l) = looser {
        let solution = last * l.get_unmarked().iter().sum::<i32>();
        println!("{}", solution);
    } else {
        println!("Failed");
    }
}
