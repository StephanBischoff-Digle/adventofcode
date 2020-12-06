use crate::error::ParseGlyphError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Line {
    line: Vec<bool>,
}

impl Line {
    pub fn check(&self, index: usize) -> bool {
        let mapped_index = index % self.line.len();
        self.line[mapped_index]
    }
}

impl FromStr for Line {
    type Err = ParseGlyphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = Vec::new();
        for (idx, c) in s.chars().enumerate() {
            if c == '.' {
                line.push(false);
            } else if c == '#' {
                line.push(true);
            } else {
                return Err(Self::Err::GlyphError(c, idx));
            }
        }

        Ok(Self { line })
    }
}

#[test]
fn from_str_valid() {
    let input = "#...#.#.";
    let expected = Line {
        line: vec![true, false, false, false, true, false, true, false],
    };
    let result = Line::from_str(input).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn from_str_glyph_error() {
    let input = "123";
    let result = Line::from_str(input);

    assert!(match result {
        Err(ParseGlyphError::GlyphError(c, i)) => c == '1' && i == 0,
        _ => false,
    });
}

#[test]
fn check() {
    let input = "#.";
    let indices: Vec<usize> = vec![0, 1, 2, 3, 4];
    let expected = vec![true, false, true, false, true];

    let line = Line::from_str(input).unwrap();
    let result: Vec<bool> = indices.iter().map(|&idx| line.check(idx)).collect();
    assert_eq!(expected, result);
}
