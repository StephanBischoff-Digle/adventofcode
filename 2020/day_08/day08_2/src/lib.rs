use std::collections::HashSet;
use std::str::FromStr;

#[derive(PartialEq)]
enum Operation {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Nop(load) => f.pad(&format!("NOP {:+4}", load)),
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
                "nop" => Operation::Nop(payload),
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
    pub fn step(&mut self) -> bool {
        if self.prog_counter >= self.instructions.len() {
            return false;
        }

        let mut was_jmp = false;
        match self.instructions[self.prog_counter] {
            Operation::Nop(_) => self.prog_counter += 1,
            Operation::Acc(load) => {
                self.prog_counter += 1;
                self.acc += load;
            }
            Operation::Jmp(load) => {
                self.prog_counter = (self.prog_counter as i32 + load) as usize;
                was_jmp = true;
            }
        }

        if self.prog_counter == self.instructions.len() && !was_jmp {
            return true;
        }

        false
    }

    pub fn debug_step(&mut self) -> bool {
        if self.prog_counter >= self.instructions.len() {
            return false;
        }

        println!(
            "{:03} -> {:10} => {}",
            self.prog_counter, self.instructions[self.prog_counter], self.acc
        );

        self.step()
    }

    pub fn find_termination(&mut self) -> Option<i32> {
        let mut global_seen = HashSet::new();
        let mut prev_branch_idx = 0;
        let mut branch_idx = 0;
        let mut branch_acc = self.acc;
        let mut iterations = 0;

        let mut next_needs_change = false;

        let mut seen = HashSet::new();
        loop {
            seen.insert(self.prog_counter);

            if iterations >= self.instructions.len() || global_seen.len() == self.instructions.len()
            {
                return None;
            }

            if next_needs_change
                && branch_idx != self.prog_counter
                && self.prog_counter > prev_branch_idx
            {
                // change instruction
                match self.instructions[self.prog_counter] {
                    Operation::Jmp(x) => {
                        println!("change {}", self.prog_counter);
                        self.instructions[self.prog_counter] = Operation::Nop(x);
                        next_needs_change = false;
                        prev_branch_idx = branch_idx;
                        branch_idx = self.prog_counter;
                        branch_acc = self.acc;
                        global_seen = seen.clone();
                    }
                    Operation::Nop(x) => {
                        println!("change {}", self.prog_counter);
                        self.instructions[self.prog_counter] = Operation::Jmp(x);
                        next_needs_change = false;
                        prev_branch_idx = branch_idx;
                        branch_idx = self.prog_counter;
                        branch_acc = self.acc;
                        global_seen = seen.clone();
                    }
                    _ => (),
                }
            }

            // do the instruction
            if self.debug_step() {
                return Some(self.acc);
            }

            if seen.contains(&self.prog_counter) {
                // rewind
                println!(" -- {} => REWIND to {} --", self.prog_counter, branch_idx);
                self.prog_counter = branch_idx;
                self.acc = branch_acc;
                seen = global_seen.clone();

                // cleanup
                if !next_needs_change && iterations > 0 {
                    println!("repair {}", branch_idx);
                    match self.instructions[branch_idx] {
                        Operation::Jmp(x) => self.instructions[branch_idx] = Operation::Nop(x),
                        Operation::Nop(x) => self.instructions[branch_idx] = Operation::Jmp(x),
                        _ => (),
                    }
                }

                next_needs_change = true;
                iterations += 1;
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
    let mut program = Program::from_str(input).expect("Failed to parse program");
    let expected = Some(8);
    let result = program.find_termination();
    assert_eq!(expected, result);
}

pub fn solve(input: &str) -> i32 {
    let mut prog = Program::from_str(input).expect("Failed to parse program");
    prog.find_termination()
        .expect("Didn't find shit, maybe I need to change the first instruction")
}
