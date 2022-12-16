use std::fs;

use instruction::Instruction;
use nom::{character::complete::newline, multi::separated_list1, IResult};
use rope::Head;

mod direction;
mod instruction;
mod point;
mod rope;

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, Instruction::parse)(input)
}

fn main() {
    let Ok(input) = fs::read_to_string("input.txt") else {
        eprintln!("Failed to read input file");
        return;
    };

    let Ok((_, instructions)) = parse_input(&input) else {
        eprintln!("Failed to parse input");
        return;
    };

    let mut head = Head::default();
    instructions
        .iter()
        .for_each(|instruction| head.apply(instruction));

    println!("{}", head.tail_unique_visits());
}
