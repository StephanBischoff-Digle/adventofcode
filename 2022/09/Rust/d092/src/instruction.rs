use nom::character::complete::u32 as parse_u32;
use nom::{bytes::complete::tag, sequence::separated_pair, IResult};

use crate::direction::Direction;

pub struct Instruction {
    pub direction: Direction,
    pub steps: u32,
}

impl Instruction {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (direction, steps)) =
            separated_pair(Direction::parse, tag(" "), parse_u32)(input)?;
        Ok((input, Self { direction, steps }))
    }
}
