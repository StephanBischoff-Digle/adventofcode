use std::convert::From;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

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
