use std::collections::VecDeque;
use std::fmt::Display;

use nom::bytes::complete::take_until;
use nom::character::complete::newline;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};
use tracing::info;

use crate::command::Command;
use crate::crates::{try_parse_crate, Crate, CrateSpot};

#[derive(Debug, PartialEq, Eq)]
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
        let len = self.stacks[(cmd.from - 1) as usize].len();

        let crane: Vec<Crate> = self.stacks[(cmd.from - 1) as usize]
            .drain((len - cmd.amount as usize)..)
            .collect();
        info!(
            "Crane moves | {} |",
            crane
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        self.stacks[(cmd.to - 1) as usize].extend(crane);

        info!("{}", self);
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
            writeln!(f)?;
        }
        self.stacks
            .iter()
            .enumerate()
            .for_each(|(i, _)| write!(f, " {}  ", i + 1).unwrap());
        writeln!(f)?;
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
        let a = VecDeque::from_iter(vec![Crate::new('Z'), Crate::new('N')]);
        let b = VecDeque::from_iter(vec![Crate::new('M'), Crate::new('C'), Crate::new('D')]);
        let c = VecDeque::from_iter(vec![Crate::new('P')]);
        let expected = Stacks {
            stacks: vec![a, b, c],
        };

        assert_eq!(stacks, expected);
    }
}
