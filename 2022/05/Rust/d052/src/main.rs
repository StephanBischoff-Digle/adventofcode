mod command;
mod crates;
mod stacks;

use std::fs;

use command::{parse_command, Command};
use nom::{
    bytes::complete::take_until1, character::complete::newline, multi::separated_list1, IResult,
};
use stacks::{parse_stacks, Stacks};
use tracing::error;

struct Input {
    stacks: Stacks,
    commands: Vec<Command>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, stacks) = parse_stacks(input)?;
    let (input, _) = take_until1("move")(input)?;
    let (input, commands) = separated_list1(newline, parse_command)(input)?;

    Ok((input, Input { stacks, commands }))
}

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let Ok(input_str) = fs::read_to_string("input.txt") else {
        error!("Failed to open input file!");
        return;
    };

    let Ok((_, mut input)) = parse_input(&input_str) else {
        error!("Failed to parse input string");
        return;
    };

    input
        .commands
        .iter()
        .for_each(|cmd| input.stacks.apply(cmd));

    println!("{}", input.stacks.collect_top());
}
