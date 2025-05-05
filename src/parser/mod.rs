//! Brainfuck parser.
//!
//! This module provides a parser for Brainfuck programs.

mod error;
mod ops;
mod parser;

pub use error::ParseError;
pub use ops::BfOp;
#[cfg(feature = "optimizer")]
pub use ops::OptimizedOp;
pub use parser::Parser;
