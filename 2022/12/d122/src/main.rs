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
    let mut node_field = Vec::new();
    for y in 0..field.len() {
        let mut row = Vec::new();
        for x in 0..field[y].len() {
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

        let cx = node.pos.x;
        let cy = node.pos.y;
        let ccost = node.cost;
        let h = field[cy][cx];

        // check neighbors
        if cx > 0 {
            let mut l_node = node_field[cy][cx - 1];
            let l_h = field[l_node.pos.y][l_node.pos.x];
            if l_h <= h + 1 && (l_node.cost > ccost + 1 || l_node.parent.is_none()) {
                l_node.cost = ccost + 1;
                l_node.parent = Some(Point::new(cx, cy));
                node_field[cy][cx - 1].cost = ccost + 1;
                node_field[cy][cx - 1].parent = Some(Point::new(cx, cy));
                heap.push(l_node);
            }
        }
        if cy > 0 {
            let mut l_node = node_field[cy - 1][cx];
            let l_h = field[l_node.pos.y][l_node.pos.x];
            if l_h <= h + 1 && (l_node.cost > ccost + 1 || l_node.parent.is_none()) {
                l_node.cost = ccost + 1;
                l_node.parent = Some(Point::new(cx, cy));
                node_field[cy - 1][cx].cost = ccost + 1;
                node_field[cy - 1][cx].parent = Some(Point::new(cx, cy));
                heap.push(l_node);
            }
        }
        if cx < node_field[0].len() - 1 {
            let mut l_node = node_field[cy][cx + 1];
            let l_h = field[l_node.pos.y][l_node.pos.x];
            if l_h <= h + 1 && (l_node.cost > ccost + 1 || l_node.parent.is_none()) {
                l_node.cost = ccost + 1;
                l_node.parent = Some(Point::new(cx, cy));
                node_field[cy][cx + 1].cost = ccost + 1;
                node_field[cy][cx + 1].parent = Some(Point::new(cx, cy));
                heap.push(l_node);
            }
        }
        if cy < node_field.len() - 1 {
            let mut l_node = node_field[cy + 1][cx];
            let l_h = field[l_node.pos.y][l_node.pos.x];
            if l_h <= h + 1 && (l_node.cost > ccost + 1 || l_node.parent.is_none()) {
                l_node.cost = ccost + 1;
                l_node.parent = Some(Point::new(cx, cy));
                node_field[cy + 1][cx].cost = ccost + 1;
                node_field[cy + 1][cx].parent = Some(Point::new(cx, cy));
                heap.push(l_node);
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
        println!("{:3} of {}", i, startpoints.len());
        best = best.min(find_path(&field, *start, end));
    }
    println!("{}", best);
}
