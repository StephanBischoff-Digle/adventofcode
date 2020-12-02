use std::convert::From;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ParseIntervalError {
    WrongComponentOrder,
    WrongComponentNumber,
    IntegerError(ParseIntError),
}

impl Error for ParseIntervalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IntegerError(ref pie) => Some(pie),
            _ => None,
        }
    }
}

impl fmt::Display for ParseIntervalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongComponentNumber => write!(f, "Wrong number of interval components"),
            Self::WrongComponentOrder => write!(f, "Wrong order of components"),
            Self::IntegerError(ie) => write!(f, "Interval parsing failed: {}", ie),
        }
    }
}

impl From<ParseIntError> for ParseIntervalError {
    fn from(pie: ParseIntError) -> Self {
        Self::IntegerError(pie)
    }
}

#[derive(Debug, PartialEq)]
pub struct Interval {
    min: u32,
    max: u32,
}

impl FromStr for Interval {
    type Err = ParseIntervalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split('-');
        let mut components = Vec::new();

        // sadly parsing in a maping of the split does not allow reasonable
        // error handling :(
        for i in split {
            components.push(i.parse::<u32>()?);
        }

        if components.len() == 2 {
            if components[0] <= components[1] {
                return Ok(Self {
                    min: components[0],
                    max: components[1],
                });
            }
            return Err(ParseIntervalError::WrongComponentOrder);
        }
        Err(ParseIntervalError::WrongComponentNumber)
    }
}

impl Interval {
    pub fn new(min: u32, max: u32) -> Result<Self, ParseIntervalError> {
        if min <= max {
            return Ok(Self { min, max });
        }
        Err(ParseIntervalError::WrongComponentOrder)
    }

    pub fn contains(&self, other: u32) -> bool {
        self.min <= other && other <= self.max
    }
}

#[test]
fn from_str_valid() {
    let input = "1-2";
    let expected = Ok(Interval { min: 1, max: 2 });
    let result = Interval::from_str(input);
    assert_eq!(expected, result);
}

#[test]
fn from_str_wrong_order() {
    let input = "2-1";
    let expected = Err(ParseIntervalError::WrongComponentOrder);
    let result = Interval::from_str(input);
    assert_eq!(expected, result);
}

#[test]
fn from_str_wrong_format() {
    let input = "2+1";
    let result = Interval::from_str(input);

    assert!(match result {
        Err(ParseIntervalError::IntegerError(_)) => true,
        _ => false,
    });
}

#[test]
fn from_str_wrong_number() {
    let input = "1-2-3";
    let expected = Err(ParseIntervalError::WrongComponentNumber);
    let result = Interval::from_str(input);
    assert_eq!(expected, result);

    let input = "1";
    let expected = Err(ParseIntervalError::WrongComponentNumber);
    let result = Interval::from_str(input);
    assert_eq!(expected, result);
}

#[test]
fn contains() {
    let interval = Interval::new(10, 12).unwrap();

    let expected = vec![false, true, false];
    let results = vec![
        interval.contains(9),
        interval.contains(11),
        interval.contains(13),
    ];

    expected
        .iter()
        .zip(results.iter())
        .for_each(|(a, b)| assert_eq!(a, b));
}
