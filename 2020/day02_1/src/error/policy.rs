use crate::error::ParseIntervalError;
use std::convert::From;
use std::error::Error;
use std::fmt;

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
