use std::{cmp::Reverse, collections::VecDeque, fmt::Debug, fs};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till},
    character::complete::newline,
    character::complete::u64 as p_u64,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

type Transform = Box<dyn Fn(u64) -> u64>;

struct TargetItem {
    target: usize,
    item: u64,
}

struct Monkey {
    items: VecDeque<u64>,
    op: Transform,
    test: u64,
    success_target: usize,
    failure_target: usize,
    inspected: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("test", &self.test)
            .field("success_target", &self.success_target)
            .field("failure_target", &self.failure_target)
            .field("inspected", &self.inspected)
            .finish()
    }
}

impl Monkey {
    fn new(
        items: VecDeque<u64>,
        op: Transform,
        test: u64,
        s_target: usize,
        f_target: usize,
    ) -> Self {
        Self {
            items,
            op,
            test,
            success_target: s_target,
            failure_target: f_target,
            inspected: 0,
        }
    }

    fn parse_operation(input: &str) -> IResult<&str, Box<dyn Fn(u64) -> u64>> {
        let (input, op_str) = preceded(tag(" new = old "), is_not("\n"))(input)?;
        let (operant, sign) = alt((tag("+"), tag("*")))(op_str)?;
        match sign {
            "+" => match operant {
                " old" => Ok((input, Box::new(|x| x + x))),
                x => {
                    let (_, v) = preceded(tag(" "), p_u64)(x)?;
                    Ok((input, Box::new(move |x| x + v)))
                }
            },
            "*" => match operant {
                " old" => Ok((input, Box::new(|x| x * x))),
                x => {
                    let (_, v) = preceded(tag(" "), p_u64)(x)?;
                    Ok((input, Box::new(move |x| x * v)))
                }
            },
            _ => unreachable!(),
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = take_till(|c| c == '\n')(input)?;
        let (input, item_str) =
            delimited(tag("\n  Starting items: "), is_not("\n"), newline)(input)?;
        let (_, items) = separated_list1(tag(", "), p_u64)(item_str)?;
        let (input, op) = preceded(tag("  Operation:"), Self::parse_operation)(input)?;
        let (input, test) = delimited(is_not("0123456789"), p_u64, newline)(input)?;
        let (input, succ) = delimited(is_not("0123456789"), p_u64, newline)(input)?;
        let (input, fail) = preceded(is_not("0123456789"), p_u64)(input)?;
        Ok((
            input,
            Self::new(
                VecDeque::from(items),
                op,
                test,
                succ as usize,
                fail as usize,
            ),
        ))
    }

    fn turn(&mut self, mod_space: &u64) -> Vec<TargetItem> {
        let mut ret = Vec::new();
        while let Some(item) = self.items.pop_front() {
            self.inspected += 1;
            let item = (self.op)(item) % mod_space;
            match item % self.test {
                0 => ret.push(TargetItem {
                    target: self.success_target,
                    item,
                }),
                _ => ret.push(TargetItem {
                    target: self.failure_target,
                    item,
                }),
            }
        }

        ret
    }
}

fn round(monkeys: &mut Vec<Monkey>, mod_space: &u64) {
    for i in 0..monkeys.len() {
        let tis = monkeys[i].turn(mod_space);
        for ti in tis {
            monkeys[ti.target].items.push_back(ti.item);
        }
    }
}

fn main() {
    let Ok(input) = fs::read_to_string("input.txt") else {
        eprintln!("Failed to read input.txt");
        return;
    };
    let Ok((_, mut monkeys)) = separated_list1(tag("\n\n"), Monkey::parse)(&input) else {
        eprintln!("Failed to parse monkeys");
        return;
    };

    let mod_space: u64 = monkeys.iter().map(|monkey| monkey.test).product();
    for _ in 0..10_000 {
        round(&mut monkeys, &mod_space);
    }

    let mut inspected: Vec<_> = monkeys.iter().map(|monkey| monkey.inspected).collect();
    inspected.sort_by_key(|w| Reverse(*w));
    let val: usize = inspected.iter().take(2).product();
    println!("{}", val);
}
