use std::collections::HashSet;
use std::str::FromStr;

enum Operation {
    Nop,
    Acc(i32),
    Jmp(i32),
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Nop => f.pad("NOP"),
            Self::Acc(load) => f.pad(&format!("ACC {:+4}", load)),
            Self::Jmp(load) => f.pad(&format!("JMP {:+4}", load)),
        }
    }
}

struct Program {
    instructions: Vec<Operation>,
    acc: i32,
    prog_counter: usize,
}

impl FromStr for Program {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = input.trim_end().split('\n').collect();

        let mut operations = Vec::new();
        for line in lines.iter() {
            let line_split: Vec<&str> = line.split(' ').collect();
            let payload = line_split[1]
                .parse::<i32>()
                .expect("couldn't parse payload");
            operations.push(match line_split[0] {
                "nop" => Operation::Nop,
                "acc" => Operation::Acc(payload),
                "jmp" => Operation::Jmp(payload),
                _ => return Err("Failed to parse program"),
            });
        }
        Ok(Self {
            instructions: operations,
            acc: 0,
            prog_counter: 0,
        })
    }
}

impl Program {
    pub fn step(&mut self) {
        if self.prog_counter >= self.instructions.len() {
            return;
        }

        match self.instructions[self.prog_counter] {
            Operation::Nop => self.prog_counter += 1,
            Operation::Acc(load) => {
                self.prog_counter += 1;
                self.acc += load;
            }
            Operation::Jmp(load) => self.prog_counter = (self.prog_counter as i32 + load) as usize,
        }
    }

    pub fn debug_step(&mut self) {
        println!(
            "{:03} -> {:10} => {}",
            self.prog_counter, self.instructions[self.prog_counter], self.acc
        );

        self.step();
    }

    pub fn finite_run(&mut self, debug: bool) -> i32 {
        let mut seen = HashSet::new();

        loop {
            let acc = self.acc;
            seen.insert(self.prog_counter);
            match debug {
                true => self.debug_step(),
                false => self.step(),
            };
            if seen.contains(&self.prog_counter) {
                return acc;
            }
        }
    }
}

#[test]
fn example() {
    let input = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let mut prog = Program::from_str(input).expect("Failed to parse program");
    let result = prog.finite_run(true);
    let expected = 5;
    assert_eq!(expected, result);
}

pub fn solve(input: &str) -> i32 {
    let mut prog = Program::from_str(input).expect("Failed to parse program");
    prog.finite_run(false)
}
