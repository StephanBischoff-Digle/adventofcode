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

fn print_vis(vis: &[Vec<bool>]) {
    let s = vis
        .iter()
        .map(|l| {
            l.iter()
                .map(|v| if *v { "A" } else { "." })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", s);
}

fn solve(field: &Field) -> usize {
    let len = field.len();

    let mut visible = Vec::with_capacity(len);
    field.iter().for_each(|_| visible.push(vec![false; len]));
    let mut kernel = [vec![-1; len], vec![-1; len], vec![-1; len], vec![-1; len]];

    for i in 0..len {
        let ir = len - 1 - i;
        for o in 0..len {
            if kernel[0][o] < field[i][o] as i32 {
                visible[i][o] |= true;
                kernel[0][o] = field[i][o] as i32
            }
            if kernel[1][o] < field[ir][o] as i32 {
                visible[ir][o] |= true;
                kernel[1][o] = field[ir][o] as i32
            }
            if kernel[2][o] < field[o][i] as i32 {
                visible[o][i] |= true;
                kernel[2][o] = field[o][i] as i32
            }
            if kernel[3][o] < field[o][ir] as i32 {
                visible[o][ir] |= true;
                kernel[3][o] = field[o][ir] as i32
            }
        }
    }
    print_vis(&visible);

    visible
        .iter()
        .map(|l| l.iter().filter(|&v| *v).count())
        .sum()
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
