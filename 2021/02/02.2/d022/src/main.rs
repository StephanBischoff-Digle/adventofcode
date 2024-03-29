use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

enum Cmd {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Debug)]
struct CmdParseError {
    details: String,
}

impl CmdParseError {
    fn new(msg: String) -> Self {
        Self { details: msg }
    }
}

impl Display for CmdParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CmdParseError")
    }
}
impl Error for CmdParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl FromStr for Cmd {
    type Err = CmdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>()[..] {
            ["forward", b] => Ok(Self::Forward(
                b.parse::<i32>().expect("Failed to parse Command Value"),
            )),
            ["up", b] => Ok(Self::Up(
                b.parse::<i32>().expect("Failed to parse Command Value"),
            )),
            ["down", b] => Ok(Self::Down(
                b.parse::<i32>().expect("Failed to parse Command Value"),
            )),
            _ => Err(CmdParseError::new(format!("could not parse '{}'", s))),
        }
    }
}

#[derive(Default)]
struct Pos {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Pos {
    fn up(&mut self, val: i32) {
        self.aim -= val;
    }

    fn down(&mut self, val: i32) {
        self.aim += val;
    }

    fn forward(&mut self, val: i32) {
        self.horizontal += val;
        self.depth += self.aim * val;
    }

    fn apply_cmd_str(&mut self, cmd: &str) {
        let p_cmd = cmd.parse::<Cmd>().expect("Failed to parse CmdVal");
        match p_cmd {
            Cmd::Down(val) => self.down(val),
            Cmd::Up(val) => self.up(val),
            Cmd::Forward(val) => self.forward(val),
        }
    }

    fn score(&self) -> i32 {
        self.depth * self.horizontal
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let input: Vec<&str> = input.trim_end().split('\n').collect();

    let mut p = Pos::default();
    for line in input.iter() {
        p.apply_cmd_str(line);
    }

    println!("{}", p.score());
}
