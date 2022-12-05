use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32 as parse_u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

enum Containment {
    Disjunct,
    Intersecting,
    Enclosed,
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn containment(&self, other: &Self) -> Containment {
        if (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end)
        {
            return Containment::Enclosed;
        }

        if self.start > other.end || other.start > self.end {
            return Containment::Disjunct;
        }

        Containment::Intersecting
    }
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (start, end)) = separated_pair(parse_u32, tag("-"), parse_u32)(input)?;
    Ok((input, Range { start, end }))
}

fn parse_line(input: &str) -> IResult<&str, (Range, Range)> {
    let (input, (a, b)) = separated_pair(parse_range, tag(","), parse_range)(input)?;
    Ok((input, (a, b)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Range, Range)>> {
    let (input, ranges) = separated_list1(newline, parse_line)(input)?;
    Ok((input, ranges))
}

fn compute(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter_map(|(a, b)| match a.containment(b) {
            Containment::Enclosed => Some(()),
            Containment::Intersecting => Some(()),
            Containment::Disjunct => None,
        })
        .count()
}

fn main() {
    match &fs::read_to_string("input.txt") {
        Ok(data) => match parse_input(data) {
            Ok((_, input)) => println!("{}", compute(&input)),
            Err(err) => eprintln!("Error parsing: {}", err),
        },
        Err(err) => eprintln!("Failed to read input file: {}", err),
    }
}
