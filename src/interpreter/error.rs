use std::error::Error;
use std::{fmt, io};

/// Errors that may occur during program execution.
#[derive(Debug)]
pub enum InterpreterError {
    /// Attempted to move the pointer before the beginning of memory
    PointerUnderflow {
        position: usize,
        attempted_move: usize,
    },
    /// Error while reading from input
    InputError(io::Error),
    /// Error while writing to output
    OutputError(io::Error),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::PointerUnderflow {
                position,
                attempted_move,
            } => write!(f, "Pointer underflow: attempted to move left {} steps when pointer was at position {}",
                    attempted_move, position),
            InterpreterError::InputError(err) => write!(f, "Input error: {}", err),
            InterpreterError::OutputError(err) => write!(f, "Output error: {}", err),
        }
    }
}

impl Error for InterpreterError {}
