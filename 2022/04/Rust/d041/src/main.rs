use std::{fs, str::FromStr};

enum Containment {
    Disjunct,
    Intersecting,
    Enclosed,
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn containment(&self, other: &Self) -> Containment {
        if (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end)
        {
            return Containment::Enclosed;
        }

        if self.start > other.end || other.start > self.end {
            return Containment::Disjunct;
        }

        Containment::Intersecting
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // This might panic
        let split: Vec<u32> = s
            .split('-')
            .map(|c| c.parse().expect(&format!("Parsing {}", c)))
            .collect();
        if split.len() == 2 {
            return Ok(Self {
                start: u32::min(split[0], split[1]),
                end: u32::max(split[0], split[1]),
            });
        }
        Err(())
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Reading input file");
    let count = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|sr| Range::from_str(sr).expect(&format!("Parsing Range from {}", sr)))
                .collect::<Vec<_>>()
        })
        .filter_map(|range_vec| match range_vec[0].containment(&range_vec[1]) {
            Containment::Enclosed => Some(()),
            _ => None,
        })
        .count();
    println!("{}", count);
}
