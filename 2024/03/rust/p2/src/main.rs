use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    error::{Error, ErrorKind},
    multi::many1,
    sequence::{delimited, tuple},
    Err, IResult,
};
use tracing::{debug, info};

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<(), String> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let cli = Args::parse();

    let mut buffer = String::new();
    let Ok(mut file) = File::open(cli.input) else {
        return Err("Failed to open input file".to_string());
    };

    let Ok(_) = file.read_to_string(&mut buffer) else {
        return Err("Failed to read input file".to_string());
    };

    let Ok((_, parsed)) = Instruction::parse_many(&buffer) else {
        return Err("Failed to parse input".to_string());
    };

    info!("Instructions: {:?}", parsed);

    let mut exec = Executor::default();
    exec.run(&parsed);

    info!("Final sum: {}", exec.sum);

    Ok(())
}

#[derive(Debug)]
struct Executor {
    sum: u32,
    skip_mul: bool,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            sum: 0,
            skip_mul: false,
        }
    }
}

impl Executor {
    fn run(&mut self, insts: &[Instruction]) {
        for inst in insts.iter() {
            debug!("skip_mul: {}; {:?}", self.skip_mul, inst);
            match inst {
                Instruction::Do => self.skip_mul = false,
                Instruction::Dont => self.skip_mul = true,
                Instruction::Multiply { a, b } if !self.skip_mul => self.sum += a * b,
                _ => (),
            }
            debug!("sum: {}", self.sum);
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Multiply { a: u32, b: u32 },
    Dont,
    Do,
    Garbage,
}

impl Instruction {
    fn parse_mul(input: &str) -> IResult<&str, Self> {
        debug!("parsing mul");
        let (input, _) = tag("mul")(input)?;
        let (input, (a, _, b)) = delimited(
            tag("("),
            tuple((digit1::<&str, _>, tag(","), digit1::<&str, _>)),
            tag(")"),
        )(input)?;

        Ok((
            input,
            Self::Multiply {
                a: a.parse().expect("Failed to parse a"),
                b: b.parse().expect("Failed to parse b"),
            },
        ))
    }

    fn parse_dont(input: &str) -> IResult<&str, Self> {
        debug!("parsing don't()");
        let (input, _) = tag("don't()")(input)?;
        Ok((input, Self::Dont))
    }

    fn parse_do(input: &str) -> IResult<&str, Self> {
        debug!("parsing do()");
        let (input, _) = tag("do()")(input)?;
        Ok((input, Self::Do))
    }

    fn parse_garbage(input: &str) -> IResult<&str, Self> {
        debug!("parsing garbage");

        if input.is_empty() {
            Err(Err::Error(Error::new(&"", ErrorKind::Eof)))
        } else {
            Ok((&input[1..], Self::Garbage))
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        debug!("parsing");
        let (input, inst) = alt((
            Instruction::parse_mul,
            Instruction::parse_do,
            Instruction::parse_dont,
            Instruction::parse_garbage,
        ))(input)?;
        Ok((input, inst))
    }

    fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        info!("Parsing input: {input}");
        let (input, vo) = many1(Self::parse)(input)?;

        info!("Done parsing, filtering garbage");
        Ok((
            input,
            vo.into_iter().filter(|i| *i != Self::Garbage).collect(),
        ))
    }
}
