use crate::error::ParseGlyphError;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseFieldError {
    ParseError(usize, ParseGlyphError),
}

impl fmt::Display for ParseFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(idx, glyph_error) => {
                write!(f, "Field parsing failed in line {}: {}", idx, glyph_error)
            }
        }
    }
}

impl Error for ParseFieldError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseError(_, ref pge) => Some(pge),
        }
    }
}
