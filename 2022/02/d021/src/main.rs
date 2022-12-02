use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

struct Match {
    op: RPS,
    me: RPS,
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
                op: self
                    .op_map
                    .get(enc_a)
                    .expect(&format!("Op mapping from {}", enc_a)),
                me: self
                    .my_map
                    .get(enc_b)
                    .expect(&format!("My mapping from {}", enc_b)),
            };
        }
    }
}

fn main() {}
