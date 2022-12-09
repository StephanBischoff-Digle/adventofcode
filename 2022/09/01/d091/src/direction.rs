use nom::{branch::alt, bytes::complete::tag, IResult};

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::p_up, Self::p_down, Self::p_left, Self::p_right))(input)
    }

    fn p_up(input: &str) -> IResult<&str, Self> {
        let (i, _) = tag("U")(input)?;
        Ok((i, Self::Up))
    }

    fn p_down(input: &str) -> IResult<&str, Self> {
        let (i, _) = tag("D")(input)?;
        Ok((i, Self::Down))
    }
    fn p_left(input: &str) -> IResult<&str, Self> {
        let (i, _) = tag("L")(input)?;
        Ok((i, Self::Left))
    }
    fn p_right(input: &str) -> IResult<&str, Self> {
        let (i, _) = tag("R")(input)?;
        Ok((i, Self::Right))
    }
}
