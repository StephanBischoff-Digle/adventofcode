use std::fmt::Display;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, sequence::delimited, IResult,
};

#[derive(Debug, PartialEq)]
pub enum CrateSpot {
    Empty,
    Occupied(Crate),
}

#[derive(Debug, PartialEq)]
pub struct Crate {
    pub designation: char,
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.designation)
    }
}

impl Crate {
    pub fn new(c: char) -> Self {
        Self { designation: c }
    }
}

pub fn try_parse_crate(input: &str) -> IResult<&str, CrateSpot> {
    alt((parse_crate, parse_empty_space))(input)
}

fn parse_empty_space(input: &str) -> IResult<&str, CrateSpot> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, CrateSpot::Empty))
}

fn parse_crate(input: &str) -> IResult<&str, CrateSpot> {
    let (input, des) = delimited(tag("["), anychar, tag("]"))(input)?;
    Ok((input, CrateSpot::Occupied(Crate { designation: des })))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_crate_spot_occ() {
        let input = "[A]";
        let (_, cs) = parse_crate(input).unwrap();
        if let CrateSpot::Occupied(c) = cs {
            assert_eq!(c.designation, 'A');
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_crate_spot_empty() {
        let input = "    ";
        let (_, cs) = parse_empty_space(input).unwrap();
        match cs {
            CrateSpot::Empty => assert!(true),
            _ => assert!(false),
        };
    }
}
