use std::error::Error;
use std::fmt;

/// Errors that may occur during parsing.
#[derive(Debug)]
pub enum ParseError {
    /// A loop start `[` without a matching `]` after it
    UnmatchedLoopStart { position: Option<usize> },
    /// A loop end `]` without a matching `[` before it
    UnmatchedLoopEnd { position: Option<usize> },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnmatchedLoopStart { position } => {
                write!(
                    f,
                    "Unmatched loop start{}",
                    position.map_or(String::new(), |p| format!(" at position {}", p))
                )
            }
            ParseError::UnmatchedLoopEnd { position } => {
                write!(
                    f,
                    "Unmatched loop end{}",
                    position.map_or(String::new(), |p| format!(" at position {}", p))
                )
            }
        }
    }
}

impl Error for ParseError {}
