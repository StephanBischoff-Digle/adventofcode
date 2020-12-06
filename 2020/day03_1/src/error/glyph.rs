use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseGlyphError {
    GlyphError(char, usize),
}

impl fmt::Display for ParseGlyphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GlyphError(glyph, idx) => write!(f, "Invalid glyph '{}' at index {}", glyph, idx),
        }
    }
}

impl Error for ParseGlyphError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
