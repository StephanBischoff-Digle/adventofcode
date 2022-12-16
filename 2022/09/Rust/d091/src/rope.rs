use std::collections::HashSet;

use crate::{instruction::Instruction, point::Point};

struct Tail {
    pos: Point,
    visited: HashSet<Point>,
}

impl Default for Tail {
    fn default() -> Self {
        let initial_pos = Point::default();
        let mut initial_visited = HashSet::new();
        initial_visited.insert(initial_pos);
        Self {
            pos: initial_pos,
            visited: initial_visited,
        }
    }
}

impl Tail {
    fn is_touching(&self, head: &Point) -> bool {
        let x_touching = (head.x - self.pos.x).abs() <= 1;
        let y_touching = (head.y - self.pos.y).abs() <= 1;
        x_touching && y_touching
    }

    fn follow(&mut self, head: Point) {
        if self.is_touching(&head) {
            return;
        }
        let delta = (head - self.pos).clamped(1, 1);
        let new_pos = self.pos + delta;
        self.visited.insert(new_pos);
        self.pos = new_pos;
    }
}

pub struct Head {
    pos: Point,
    tail: Tail,
}

impl Default for Head {
    fn default() -> Self {
        Self {
            pos: Point::default(),
            tail: Tail::default(),
        }
    }
}

impl Head {
    pub fn apply(&mut self, inst: &Instruction) {
        let delta: Point = inst.direction.into();
        for _ in 0..inst.steps {
            let np = self.pos + delta;
            self.pos = np;
            self.tail.follow(np);
        }
    }

    pub fn tail_unique_visits(&self) -> usize {
        self.tail.visited.len()
    }
}
