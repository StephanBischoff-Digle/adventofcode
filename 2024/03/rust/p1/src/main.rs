use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::opt,
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<(), String> {
    let cli = Args::parse();

    let mut buffer = String::new();
    let Ok(mut file) = File::open(cli.input) else {
        return Err("Failed to open input file".to_string());
    };

    let Ok(_) = file.read_to_string(&mut buffer) else {
        return Err("Failed to read input file".to_string());
    };

    let Ok((_, instructions)) = MulInst::parse_many(&buffer) else {
        return Err("Failed to parse input".to_string());
    };

    let solution: u32 = instructions.iter().map(|inst| inst.execute()).sum();

    println!("{solution}");

    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
struct MulInst {
    a: u32,
    b: u32,
}

impl MulInst {
    fn parse(input: &str) -> IResult<&str, Option<Self>> {
        let (input, _) = take_until("mul")(input)?;
        let (input, _) = tag("mul")(input)?;
        let k = opt(delimited(
            tag("("),
            tuple((digit1::<&str, _>, tag(","), digit1::<&str, _>)),
            tag(")"),
        ))(input);

        let (input, Some((a, _, b))) = k? else {
            return Ok((input, None));
        };

        Ok((
            input,
            Some(Self {
                a: a.parse().expect("Failed to parse a"),
                b: b.parse().expect("Failed to parse b"),
            }),
        ))
    }

    fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        let (input, vo) = many1(Self::parse)(input)?;

        Ok((input, vo.into_iter().flatten().collect()))
    }

    fn execute(&self) -> u32 {
        self.a * self.b
    }
}
