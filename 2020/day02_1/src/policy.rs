use crate::interval::Interval;
use crate::interval::ParseIntervalError;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ParsePolicyError {
    IntervalError(ParseIntervalError),
    MissingSymbolError,
    FormatError,
}

impl fmt::Display for ParsePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntervalError(pie) => write!(f, "Parsing Policy failed: {}", pie),
            Self::MissingSymbolError => write!(f, "Parsing Policy failed due to missing symbol"),
            Self::FormatError => write!(f, "Parsing Policy failed"),
        }
    }
}

impl Error for ParsePolicyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IntervalError(ref pie) => Some(pie),
            _ => None,
        }
    }
}

impl From<ParseIntervalError> for ParsePolicyError {
    fn from(pie: ParseIntervalError) -> Self {
        Self::IntervalError(pie)
    }
}

#[derive(Debug, PartialEq)]
pub struct Policy {
    interval: Interval,
    symbol: char,
}

impl Policy {
    pub fn apply(&self, input: &str) -> bool {
        self.interval
            .contains(input.chars().filter(|x| x == &self.symbol).count() as u32)
    }
}

impl FromStr for Policy {
    type Err = ParsePolicyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();

        if split.len() != 2 {
            return Err(Self::Err::FormatError);
        }
        Ok(Self {
            interval: Interval::from_str(split[0])?,
            symbol: match split[1].chars().next() {
                Some(c) => c,
                None => return Err(Self::Err::MissingSymbolError),
            },
        })
    }
}

#[test]
fn from_str_valid() {
    let input = "1-3 a";
    let expected = Policy {
        interval: Interval::new(1, 3).unwrap(),
        symbol: 'a',
    };
    let result = Policy::from_str(input).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn from_str_interval_error() {
    let input = "2 a";
    let result = Policy::from_str(input);

    assert!(match result {
        Err(ParsePolicyError::IntervalError(_)) => true,
        _ => false,
    });
}

#[test]
fn from_str_missing_symbol() {
    let input = "2-3 ";
    let result = Policy::from_str(input);
    let expected = Err(ParsePolicyError::MissingSymbolError);
    assert_eq!(expected, result);
}

#[test]
fn from_str_format() {
    let input = "2-3";
    let result = Policy::from_str(input);
    let expected = Err(ParsePolicyError::FormatError);
    assert_eq!(expected, result);
}

#[test]
fn apply_accept() {
    let input = "abcde";
    let policy = Policy {
        interval: Interval::new(1, 3).unwrap(),
        symbol: 'a',
    };

    assert!(policy.apply(input));
}

#[test]
fn apply_reject() {
    let input = "bcde";
    let policy = Policy {
        interval: Interval::new(1, 3).unwrap(),
        symbol: 'a',
    };

    assert!(!policy.apply(input));
}
