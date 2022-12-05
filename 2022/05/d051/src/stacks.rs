use std::collections::VecDeque;
use std::fmt::Display;

use nom::bytes::complete::take_until;
use nom::character::complete::newline;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};
use tracing::info;

use crate::command::Command;
use crate::crates::{try_parse_crate, Crate, CrateSpot};

#[derive(Debug, PartialEq)]
pub struct Stacks {
    stacks: Vec<VecDeque<Crate>>,
}

impl Stacks {
    pub fn new() -> Self {
        Self { stacks: Vec::new() }
    }
    pub fn push(&mut self, stack: usize, spot: CrateSpot) {
        if stack >= self.stacks.len() {
            let i = stack - self.stacks.len() + 1;
            for _ in 0..i {
                self.stacks.push(VecDeque::new());
            }
        }

        if let CrateSpot::Occupied(c) = spot {
            self.stacks[stack].push_front(c);
        }
    }

    pub fn apply(&mut self, cmd: &Command) {
        info!("Applying {:?}", cmd);
        for _ in 0..cmd.amount {
            if let Some(c) = self.stacks[(cmd.from - 1) as usize].pop_back() {
                info!("Moving {} from {} to {}", c, cmd.from, cmd.to);
                self.stacks[(cmd.to - 1) as usize].push_back(c);

                info!("{}", self);
            }
        }
    }

    pub fn collect_top(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.back().unwrap_or(&Crate::new(' ')).designation)
            .fold(String::new(), |acc, cr| acc + &cr.to_string())
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hight = self.stacks.iter().map(|s| s.len()).max().unwrap_or(0);
        for h in 0..=hight {
            for s in self.stacks.iter() {
                write!(
                    f,
                    "{} ",
                    match s.get(hight - h) {
                        Some(v) => v.to_string(),
                        None => "   ".to_owned(),
                    }
                )?;
            }
            writeln!(f, "")?;
        }
        self.stacks
            .iter()
            .enumerate()
            .for_each(|(i, _)| write!(f, " {}  ", i + 1).unwrap());
        writeln!(f, "")?;
        Ok(())
    }
}

fn parse_stack_line(input: &str) -> IResult<&str, Vec<CrateSpot>> {
    separated_list1(tag(" "), try_parse_crate)(input)
}

pub fn parse_stacks(input: &str) -> IResult<&str, Stacks> {
    let mut stacks = Stacks::new();

    let (input, config) = take_until(" 1")(input)?;
    let (_, lines) = separated_list1(newline, parse_stack_line)(config)?;
    info!("parsed {:?}", lines);
    for line in lines {
        for (idx, c) in line.into_iter().enumerate() {
            info!("Pushing {:?} onto {}", c, idx);
            stacks.push(idx, c);
        }
    }

    Ok((input, stacks))
}

#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use super::*;

    #[test]
    #[traced_test]
    fn test_parse_stack_line_full() {
        let input = "[Z] [M] [P]";
        let expected = vec![
            CrateSpot::Occupied(Crate::new('Z')),
            CrateSpot::Occupied(Crate::new('M')),
            CrateSpot::Occupied(Crate::new('P')),
        ];

        let (_, line) = parse_stack_line(input).unwrap();
        assert_eq!(line, expected);
    }

    #[test]
    #[traced_test]
    fn test_parse_stack_line_empty() {
        let input = "           ";
        let expected = vec![CrateSpot::Empty, CrateSpot::Empty, CrateSpot::Empty];

        let (_, line) = parse_stack_line(input).unwrap();
        assert_eq!(line, expected);
    }

    #[test]
    #[traced_test]
    fn test_parse_stack_line_mixed() {
        let input = "    [A]    ";
        let expected = vec![
            CrateSpot::Empty,
            CrateSpot::Occupied(Crate::new('A')),
            CrateSpot::Empty,
        ];

        let (_, line) = parse_stack_line(input).unwrap();
        assert_eq!(line, expected);
    }

    #[test]
    #[traced_test]
    fn parse_stacks_full() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3";
        let (_, stacks) = parse_stacks(input).unwrap();
        let a = VecDeque::from_iter(vec![Crate::new('N'), Crate::new('Z')]);
        let b = VecDeque::from_iter(vec![Crate::new('D'), Crate::new('C'), Crate::new('M')]);
        let c = VecDeque::from_iter(vec![Crate::new('P')]);
        let expected = Stacks {
            stacks: vec![a, b, c],
        };

        assert_eq!(stacks, expected);
    }
}
