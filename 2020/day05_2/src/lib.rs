use std::str::FromStr;

enum RowDigit {
    F,
    B,
}

impl RowDigit {
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'F' => Ok(Self::F),
            'B' => Ok(Self::B),
            _ => Err("Could not parse RowDigit"),
        }
    }
}

enum ColumnDigit {
    L,
    R,
}

impl ColumnDigit {
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'L' => Ok(Self::L),
            'R' => Ok(Self::R),
            _ => Err("Could not parse ColumnDigit"),
        }
    }
}

struct BinaryBoarding {
    row: Vec<RowDigit>,
    column: Vec<ColumnDigit>,
}

impl FromStr for BinaryBoarding {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r_digits = Vec::new();
        let mut c_digits = Vec::new();

        for c in s[..7].chars() {
            r_digits.push(RowDigit::from_char(c)?);
        }
        for c in s[7..].chars() {
            c_digits.push(ColumnDigit::from_char(c)?);
        }

        Ok(Self {
            row: r_digits,
            column: c_digits,
        })
    }
}

impl BinaryBoarding {
    pub fn numeric_row(&self) -> u32 {
        let mut l = 0;
        let mut h = 127;
        let mut last = 0;
        for rows in self.row.iter() {
            match rows {
                RowDigit::F => {
                    h = l + (h - l) / 2;
                    last = h;
                }
                RowDigit::B => {
                    l = l + (h - l) / 2 + 1;
                    last = l;
                }
            }
        }
        last
    }

    pub fn numeric_column(&self) -> u32 {
        let mut l = 0;
        let mut h = 7;
        let mut last = 0;
        for column in self.column.iter() {
            match column {
                ColumnDigit::L => {
                    h = l + (h - l) / 2;
                    last = h;
                }
                ColumnDigit::R => {
                    l = l + (h - l) / 2 + 1;
                    last = l;
                }
            }
        }
        last
    }

    pub fn id(&self) -> u32 {
        self.numeric_row() * 8 + self.numeric_column()
    }
}

pub fn solve(input: &str) -> u32 {
    // Convert all the passes to their IDs
    let mut seats: Vec<u32> = input
        .trim_end()
        .split("\n")
        .map(|x| {
            BinaryBoarding::from_str(x)
                .expect("failed to parse pass")
                .id()
        })
        .collect();

    // Sort it
    seats.sort();

    // Filter with windowed iteration
    // I wanted to use BTreeSet, such that I don't have to sort the
    // vector prior to filtering it, but BTreeSet cannot be sliced and
    // windows() is only implemented for slices :(
    let valids: Vec<u32> = seats
        .windows(2)
        .filter_map(|x| {
            match x {
                [l, r] if l + 1 == r - 1 => return Some(l + 1),
                _ => return None,
            };
        })
        .collect();

    // Sanity check and return
    if valids.len() != 1 {
        panic!("No or too many solutions");
    }
    valids[0]
}

#[test]
fn example_row() {
    let input = "FBFBBFFRLR";
    let bb = BinaryBoarding::from_str(input).expect("failure");
    let result = bb.numeric_row();
    let expected = 44;

    assert_eq!(expected, result);
}

#[test]
fn example_column() {
    let input = "FBFBBFFRLR";
    let bb = BinaryBoarding::from_str(input).expect("failure");
    let result = bb.numeric_column();
    let expected = 5;

    assert_eq!(expected, result);
}
