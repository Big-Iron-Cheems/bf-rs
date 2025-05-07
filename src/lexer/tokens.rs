use std::fmt;
use std::fmt::Formatter;

/// The Token enum represents the different Brainfuck commands.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    IncrementPointer, // >
    DecrementPointer, // <
    IncrementByte,    // +
    DecrementByte,    // -
    OutputByte,       // .
    InputByte,        // ,
    LoopStart,        // [
    LoopEnd,          // ]
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::IncrementPointer => write!(f, ">"),
            Token::DecrementPointer => write!(f, "<"),
            Token::IncrementByte => write!(f, "+"),
            Token::DecrementByte => write!(f, "-"),
            Token::OutputByte => write!(f, "."),
            Token::InputByte => write!(f, ","),
            Token::LoopStart => write!(f, "["),
            Token::LoopEnd => write!(f, "]"),
        }
    }
}
