use std::fs;

type Field = Vec<Vec<u32>>;

fn parse_field(input: &str) -> Result<Field, &str> {
    let f: Field = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
        .ok_or("Failed to parse")?;
    Ok(f)
}

fn solve(field: &Field) -> usize {
    let len = field.len();
    let mut max_score = 0;

    for x in 0..len {
        for y in 0..len {
            let mut range = [None; 4];
            let mut obstructed = [false; 4];
            let plot = field[y][x];
            for u in 1..len {
                if x == 0 {
                    range[0] = Some(0);
                    obstructed[0] = true;
                } else if !obstructed[0] && x >= u && (field[y][x - u] >= plot || x == u) {
                    range[0] = Some(u);
                    obstructed[0] = true;
                }

                if x == len - 1 {
                    range[1] = Some(0);
                    obstructed[1] = true;
                } else if !obstructed[1]
                    && x + u < len
                    && (field[y][x + u] >= plot || x + u == len - 1)
                {
                    range[1] = Some(u);
                    obstructed[1] = true;
                }

                if y == 0 {
                    range[2] = Some(0);
                    obstructed[2] = true;
                } else if !obstructed[2] && y >= u && (field[y - u][x] >= plot || y == u) {
                    range[2] = Some(u);
                    obstructed[2] = true;
                }

                if y == len - 1 {
                    range[3] = Some(0);
                    obstructed[3] = true;
                } else if !obstructed[3]
                    && y + u < len
                    && (field[y + u][x] >= plot || y + u == len - 1)
                {
                    range[3] = Some(u);
                    obstructed[3] = true;
                }
                if obstructed.iter().all(|v| *v) {
                    break;
                }
            }

            if let [Some(a), Some(b), Some(c), Some(d)] = range {
                max_score = max_score.max(a * b * c * d);
            } else {
                dbg!(range);
                panic!("Logic error at x: {} y: {}", x, y);
            }
        }
    }

    max_score
}

fn main() {
    let Ok(input) = fs::read_to_string("input.txt") else {
        eprintln!("Failed to read input.txt");
        return;
    };

    let Ok(field) = parse_field(&input) else {
        eprintln!("Failed to parse field");
        return;
    };

    println!("{}", solve(&field));
}
