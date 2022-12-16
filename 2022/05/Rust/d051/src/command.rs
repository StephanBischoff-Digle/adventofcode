use nom::bytes::complete::tag;
use nom::character::complete::u32 as parse_u32;
use nom::{sequence::preceded, IResult};
use tracing::info;

#[derive(Debug)]
pub struct Command {
    pub amount: u32,
    pub from: u32,
    pub to: u32,
}

pub fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, amount) = preceded(tag("move "), parse_u32)(input)?;
    let (input, from) = preceded(tag(" from "), parse_u32)(input)?;
    let (input, to) = preceded(tag(" to "), parse_u32)(input)?;

    let com = Command { amount, from, to };
    info!("parsed {:?}", com);
    Ok((input, com))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_command() {
        let input = "move 1 from 2 to 3";
        let (_, c) = parse_command(input).unwrap();
        assert_eq!(c.amount, 1);
        assert_eq!(c.from, 2);
        assert_eq!(c.to, 3);
    }
}
