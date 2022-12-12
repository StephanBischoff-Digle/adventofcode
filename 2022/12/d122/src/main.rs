use std::{cmp::Ordering, collections::BinaryHeap, fs};

type Field = Vec<Vec<usize>>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbors(&self, bottom_right: &Point) -> Vec<Self> {
        let mut neighbors = Vec::new();
        if self.x > 0 {
            neighbors.push(Self::new(self.x - 1, self.y));
        }

        if self.x < bottom_right.x {
            neighbors.push(Self::new(self.x + 1, self.y));
        }

        if self.y > 0 {
            neighbors.push(Self::new(self.x, self.y - 1));
        }

        if self.y < bottom_right.y {
            neighbors.push(Self::new(self.x, self.y + 1));
        }

        neighbors
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: Point,
    cost: usize,
    parent: Option<Point>,
}

impl Node {
    fn new(pos: Point, cost: usize, parent: Option<Point>) -> Self {
        Self { pos, cost, parent }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn find_path(field: &Field, start: Point, end: Point) -> usize {
    let max_y = field.len();
    let max_x = field[0].len();
    let bottom_right = Point::new(max_x - 1, max_y - 1);
    let mut node_field = Vec::new();
    for (y, line) in field.iter().enumerate() {
        let mut row = Vec::new();
        for (x, _) in line.iter().enumerate() {
            row.push(Node::new(Point::new(x, y), 0, None));
        }
        node_field.push(row);
    }

    let mut heap = BinaryHeap::new();
    heap.push(Node::new(start, 0, None));

    while let Some(node) = heap.pop() {
        if node.pos == end {
            return node.cost;
        }

        let h = field[node.pos.y][node.pos.x];

        // check neighbors
        for neighbor in node.pos.neighbors(&bottom_right) {
            let n_node = node_field[neighbor.y][neighbor.x];
            let n_height = field[neighbor.y][neighbor.x];

            if n_height <= h + 1 && (n_node.cost > node.cost + 1 || n_node.parent.is_none()) {
                node_field[neighbor.y][neighbor.x].cost = node.cost + 1;
                node_field[neighbor.y][neighbor.x].parent = Some(node.pos);
                heap.push(node_field[neighbor.y][neighbor.x]);
            }
        }
    }

    usize::MAX
}

fn parse_field(input: &str) -> (Field, Vec<Point>, Option<Point>) {
    let mut startpoints = Vec::new();
    let mut end = None;
    let mut field = Vec::new();
    for (y, line) in input.lines().enumerate() {
        field.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        startpoints.push(Point::new(x, y));
                        'a' as usize
                    }
                    'E' => {
                        end = Some(Point::new(x, y));
                        'z' as usize
                    }
                    'a' => {
                        startpoints.push(Point::new(x, y));
                        'a' as usize
                    }
                    v => v as usize,
                })
                .collect(),
        );
    }

    (field, startpoints, end)
}

fn main() {
    let Ok(input) = fs::read_to_string("input.txt") else {
        eprintln!("Failed to read input");
        return;
    };

    let (field, startpoints, Some(end)) = parse_field(&input) else {
        eprintln!("Failed to parse the field");
        return;
    };

    let mut best = usize::MAX;
    for (i, start) in startpoints.iter().enumerate() {
        if i % 50 == 0 {
            println!("{} of {}", i, startpoints.len());
        }
        best = best.min(find_path(&field, *start, end));
    }
    println!("{}", best);
}
