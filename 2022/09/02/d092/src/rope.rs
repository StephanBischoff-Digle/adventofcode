use std::collections::HashSet;

use crate::{instruction::Instruction, point::Point};

#[derive(Copy, Clone)]
struct Tail {
    pos: Point,
}

impl Default for Tail {
    fn default() -> Self {
        let initial_pos = Point::default();
        let mut initial_visited = HashSet::new();
        initial_visited.insert(initial_pos);
        Self { pos: initial_pos }
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
        self.pos = new_pos;
    }
}

pub struct Head {
    pos: Point,
    tail: [Tail; 9],
    tail_visits: HashSet<Point>,
}

impl Default for Head {
    fn default() -> Self {
        let mut visited = HashSet::new();
        visited.insert(Point::default());
        Self {
            pos: Point::default(),
            tail: [Tail::default(); 9],
            tail_visits: visited,
        }
    }
}

impl Head {
    pub fn apply(&mut self, inst: &Instruction) {
        let delta: Point = inst.direction.into();
        for _ in 0..inst.steps {
            let np = self.pos + delta;
            self.pos = np;
            self.tail[0].follow(np);

            for idx in 1..9 {
                self.tail[idx].follow(self.tail[idx - 1].pos);
            }

            self.tail_visits.insert(self.tail[8].pos);
        }
    }

    pub fn tail_unique_visits(&self) -> usize {
        self.tail_visits.len()
    }
}
