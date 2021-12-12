use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    fn is_diagonal(&self) -> bool {
        let delta_x = (self.a.x - self.b.x).abs();
        let delta_y = (self.a.y - self.b.y).abs();
        delta_x != 0 && delta_y != 0
    }

    fn get_points(&self) -> Vec<Point> {
        let delta_x = (self.a.x - self.b.x).abs();
        let delta_y = (self.a.y - self.b.y).abs();
        if delta_x == 0 {
            return (self.a.y.min(self.b.y)..=self.a.y.max(self.b.y))
                .into_iter()
                .map(|y| Point::new(self.a.x, y))
                .collect();
        }
        if delta_y == 0 {
            return (self.a.x.min(self.b.x)..=self.a.x.max(self.b.x))
                .into_iter()
                .map(|x| Point::new(x, self.a.y))
                .collect();
        }
        unreachable!("Diagonals were sorted out earlier!");
    }
}

fn line_parser(lst: &Vec<&str>) -> Vec<Line> {
    let mut res = Vec::new();
    for line in lst {
        let points = line.split(" -> ").collect::<Vec<_>>();
        let p_a: Vec<i32> = points[0]
            .split(",")
            .map(|c| c.parse().expect("Parse i32."))
            .collect();
        let p_b: Vec<i32> = points[1]
            .split(",")
            .map(|c| c.parse().expect("Parse i32."))
            .collect();
        let a = Point::new(p_a[0], p_a[1]);
        let b = Point::new(p_b[0], p_b[1]);
        let l = Line::new(a, b);
        if !l.is_diagonal() {
            res.push(l)
        }
    }
    res
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Read from input.txt");
    let input: Vec<&str> = input.trim_end().split("\n").collect();

    let segments = line_parser(&input);

    let mut occupied = HashMap::new();
    segments.iter().for_each(|s| {
        s.get_points().into_iter().for_each(|point| {
            let entry = occupied.entry(point).or_insert(0);
            *entry += 1;
        })
    });

    let solution = occupied.values().filter(|&v| *v >= 2).count();
    println!("{}", solution);
}
