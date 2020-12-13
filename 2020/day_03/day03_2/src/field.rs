use crate::error::ParseFieldError;
use crate::line::Line;
use std::str::FromStr;

pub struct FieldIterator<'a> {
    dx: usize,
    dy: usize,
    field_ref: &'a Field,
    x: usize,
    y: usize,
}

impl<'a> std::iter::Iterator for FieldIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.field_ref.check(self.x, self.y);
        self.x += self.dx;
        self.y += self.dy;
        ret
    }
}

#[derive(Debug, PartialEq)]
pub struct Field {
    lines: Vec<Line>,
}

impl<'a> Field {
    pub fn check(&self, x: usize, y: usize) -> Option<bool> {
        if y >= self.lines.len() {
            return None;
        }

        Some(self.lines[y].check(x))
    }

    pub fn iter(&'a self, dx: usize, dy: usize) -> FieldIterator {
        FieldIterator {
            dx,
            dy,
            field_ref: &self,
            x: 0,
            y: 0,
        }
    }
}

impl FromStr for Field {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.trim_end().split('\n').collect();
        let mut lines = Vec::new();

        for (idx, l) in input.iter().enumerate() {
            match Line::from_str(l) {
                Err(e) => return Err(Self::Err::ParseError(idx, e)),
                Ok(l) => lines.push(l),
            }
        }

        Ok(Self { lines })
    }
}

#[test]
fn from_str_valid() {
    let input = "#...#.#.\n#...#.#.";
    let expected = Field {
        lines: vec![
            Line::from_str("#...#.#.").unwrap(),
            Line::from_str("#...#.#.").unwrap(),
        ],
    };

    let result = Field::from_str(input).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn from_str_error() {
    let input = "123\n#.#";
    let result = Field::from_str(input);

    assert!(match result {
        Err(ParseFieldError::ParseError(i, _)) => i == 0,
        _ => false,
    });
}

#[test]
fn check() {
    let input = "#.\n.#";
    let indices: Vec<(usize, usize)> = vec![(0, 0), (1, 1), (0, 1), (1, 0), (2, 0), (0, 2)];
    let expected = vec![
        Some(true),
        Some(true),
        Some(false),
        Some(false),
        Some(true),
        None,
    ];

    let field = Field::from_str(input).unwrap();
    let result: Vec<Option<bool>> = indices.iter().map(|&(x, y)| field.check(x, y)).collect();
    assert_eq!(expected, result);
}

#[test]
fn iterator() {
    let input = "#.\n#.";
    let field = Field::from_str(input).unwrap();

    let cnt = field.iter(1, 1).filter(|x| *x).count();
    assert_eq!(1, cnt);
}
