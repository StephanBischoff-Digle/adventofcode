use std::str::FromStr;
use std::error::Error;
use std::fmt::Display;
use std::fs;

enum Cmd {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
struct CmdParseError;

impl Display for CmdParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CmdParseError")
    }
}
impl Error for CmdParseError {
    fn description(&self) -> &str {
        ""
    }
}

impl FromStr for Cmd {
    type Err = CmdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(CmdParseError)
        }
    }
}

struct CmdVal {
    word: Cmd,
    val: i32,
}

impl FromStr for CmdVal {
    type Err = CmdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<_>>()[..] {
            [a, b] => 
                Ok(
                    Self {
                        word: a.parse::<Cmd>().expect("Failed to parse Cmd"),
                        val: b.parse::<i32>().expect("Failed to parse val"),
                    }
                ),
            _ => Err(CmdParseError) 
        }
    }
}


struct Pos {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Pos {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

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
        let p_cmd = cmd.parse::<CmdVal>().expect("Failed to parse CmdVal");
        match p_cmd.word {
            Cmd::Down => self.down(p_cmd.val),
            Cmd::Up => self.up(p_cmd.val),
            Cmd::Forward => self.forward(p_cmd.val),
        }
    }

    fn score(&self) -> i32 {
        self.depth * self.horizontal
    }
}

fn main() {

    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let input: Vec<&str> = input
        .trim_end()
        .split("\n")
        .collect();

    let mut p = Pos::new();
    for line in input.iter() {
        p.apply_cmd_str(line);
    }

    println!("{}", p.score());

}
