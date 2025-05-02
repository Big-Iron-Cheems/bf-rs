//! Error handling for the Brainfuck interpreter.
//!
//! This module provides a unified error type (`BfError`) that encompasses all possible
//! error conditions that may occur during lexing, parsing, and execution of Brainfuck programs.

use crate::{interpreter::InterpreterError, parser::ParseError};
use std::error::Error;
use std::fmt;
use std::io;

/// Errors that may occur in the Brainfuck interpreter.
#[derive(Debug)]
pub enum BfError {
    /// Error during parsing
    Parse(ParseError),
    /// Error during runtime execution
    Runtime(InterpreterError),
    /// Error during I/O operations
    Io(io::Error),
}

impl fmt::Display for BfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BfError::Parse(e) => write!(f, "Parse error: {}", e),
            BfError::Runtime(e) => write!(f, "Runtime error: {}", e),
            BfError::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for BfError {}

impl From<ParseError> for BfError {
    fn from(error: ParseError) -> Self {
        BfError::Parse(error)
    }
}

impl From<InterpreterError> for BfError {
    fn from(error: InterpreterError) -> Self {
        BfError::Runtime(error)
    }
}

impl From<io::Error> for BfError {
    fn from(error: io::Error) -> Self {
        BfError::Io(error)
    }
}
