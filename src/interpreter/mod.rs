//! Brainfuck interpreter.
//!
//! This module provides an interpreter for Brainfuck programs.

mod error;
mod interpreter;

pub use error::InterpreterError;
pub use interpreter::Interpreter;
