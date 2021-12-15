use std::convert::From;

use super::Field;

#[derive(Debug)]
pub struct Board {
    fields: Vec<Vec<Field>>,
}

impl Board {
    /// Calls the number `val` on every Field of the board and
    /// marks the fields that contain that number.
    pub fn call_nr(&mut self, val: i32) {
        self.fields
            .iter_mut()
            .for_each(|line| line.iter_mut().for_each(|f| f.mark(val)));
    }

    /// Checks if any row is fully marked.
    fn h(&self) -> bool {
        self.fields
            .iter()
            .any(|line| line.iter().all(|f| f.is_marked()))
    }

    /// Checks if any of the columns is fully marked
    fn v(&self) -> bool {
        for (idx, _) in self.fields.iter().enumerate() {
            if self.fields.iter().all(|line| line[idx].is_marked()) {
                return true;
            }
        }
        false
    }

    /// Checks if the Board is in bingo.
    pub fn check_bingo(&self) -> bool {
        self.h() || self.v()
    }

    /// Get all the unmarked values
    pub fn get_unmarked(&self) -> Vec<i32> {
        self.fields
            .iter()
            .flat_map(|line| line.iter().filter(|f| !f.is_marked()).map(|f| f.value()))
            .collect()
    }
}

impl From<Vec<&str>> for Board {
    fn from(v: Vec<&str>) -> Self {
        let fields = v
            .iter()
            .map(|&line| {
                line.split_ascii_whitespace()
                    .filter(|split| !split.is_empty())
                    .map(|split| Field::new(split.parse().expect("Parsing field value")))
                    .collect()
            })
            .collect();
        Self { fields }
    }
}
