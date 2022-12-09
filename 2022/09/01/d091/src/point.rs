use std::ops::{Add, Sub};

use crate::direction::Direction;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Point {
    pub fn clamped(self, vx: i32, vy: i32) -> Self {
        Self {
            x: self.x.min(vx).max(-vx),
            y: self.y.min(vy).max(-vy),
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<Direction> for Point {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Self { x: 0, y: 1 },
            Direction::Down => Self { x: 0, y: -1 },
            Direction::Left => Self { x: -1, y: 0 },
            Direction::Right => Self { x: 1, y: 0 },
        }
    }
}
