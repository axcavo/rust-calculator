use std::{fmt, usize};

pub enum ScanError {
    UnexpectedChar(char, usize),
}


impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::UnexpectedChar(c, pos) => {
                write!(f, "Unexpected char {} at position {}.", c, pos)
            },
        }
    }
}