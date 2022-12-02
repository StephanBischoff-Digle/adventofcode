use std::{collections::HashMap, fs};

enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn eval(a: &Self, b: &Self) -> Outcome {
        match (a, b) {
            (Self::Rock, Self::Scissors)
            | (Self::Scissors, Self::Paper)
            | (Self::Paper, Self::Rock) => Outcome::Win,
            (p, q) if p == q => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }
}

struct Match {
    op: RPS,
    me: RPS,
}

impl Match {
    fn score(&self) -> u32 {
        let shape_score = self.me.score();
        let outcome_score = match RPS::eval(&self.me, &self.op) {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        };
        shape_score + outcome_score
    }
}

struct Decryption {
    op_map: HashMap<&'static str, RPS>,
    my_map: HashMap<&'static str, RPS>,
}

impl Decryption {
    fn decrypt(&self, line: &str) -> Match {
        let mut parts_iter = line.split_whitespace().take(2);
        if let (Some(enc_a), Some(enc_b)) = (parts_iter.next(), parts_iter.next()) {
            return Match {
                op: *self
                    .op_map
                    .get(enc_a)
                    .expect(&format!("Op mapping from {}", enc_a)),
                me: *self
                    .my_map
                    .get(enc_b)
                    .expect(&format!("My mapping from {}", enc_b)),
            };
        }
        panic!("Faulty input line: '{}", line);
    }
}

fn main() {
    let dcrypt = Decryption {
        op_map: [("A", RPS::Rock), ("B", RPS::Paper), ("C", RPS::Scissors)]
            .into_iter()
            .collect(),
        my_map: [("X", RPS::Rock), ("Y", RPS::Paper), ("Z", RPS::Scissors)]
            .into_iter()
            .collect(),
    };

    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let score: u32 = input.lines().map(|line| dcrypt.decrypt(line).score()).sum();

    println!("{}", score);
}
