use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Unknow,
    Increasing,
    Decreasing,
}

struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn parse_from_line(line: &str) -> Self {
        let levels = line
            .split(' ')
            .map(|x| x.parse().expect("Failed to parse"))
            .collect();
        Self { levels }
    }

    fn is_valid(&self) -> bool {
        if Report::check_report(&self.levels) {
            return true;
        }

        // Try to remove single indices until success
        // PERF: everything is shit
        for idx in 0..self.levels.len() {
            let mut report_under_test = self.levels.clone();
            report_under_test.remove(idx);
            if Report::check_report(&report_under_test) {
                return true;
            }
        }
        return false;
    }

    fn check_report(levels: &[u32]) -> bool {
        let n = levels
            .windows(2)
            .scan(Direction::Unknow, |dir, v| {
                let this_dir = get_direction(v)?;

                if *dir == Direction::Unknow {
                    *dir = this_dir;
                    return Some(*dir);
                }

                if *dir != this_dir {
                    return None;
                }

                Some(*dir)
            })
            .count();

        // Return true if the lenght is as expected
        n == (levels.len() - 1)
    }
}

fn get_direction(pair: &[u32]) -> Option<Direction> {
    let a = pair[0];
    let b = pair[1];
    let diff = a.abs_diff(b);
    let out_of_bounds = !(1..=3).contains(&diff);

    if out_of_bounds {
        return None;
    }

    let dir = if a > b {
        Direction::Decreasing
    } else {
        Direction::Increasing
    };

    Some(dir)
}

fn main() -> Result<(), String> {
    let cli = Args::parse();

    let mut buffer = String::new();
    let Ok(mut file) = File::open(cli.input) else {
        return Err("Failed to open input file".to_string());
    };

    let Ok(_) = file.read_to_string(&mut buffer) else {
        return Err("Failed to read input file".to_string());
    };

    let solution = buffer
        .lines()
        .map(Report::parse_from_line)
        .filter(|rep| rep.is_valid())
        .count();

    println!("{solution}");

    Ok(())
}
