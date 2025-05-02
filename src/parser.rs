//! Brainfuck parser.  
//! It takes a vector of tokens and returns a vector of instructions.

use crate::lexer::Token;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

/// Errors that may occur during parsing.
#[derive(Debug)]
pub enum ParseError {
    /// A loop start `[` without a matching `]` after it
    UnmatchedLoopStart { position: Option<usize> },
    /// A loop end `]` without a matching `[` before it
    UnmatchedLoopEnd { position: Option<usize> },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

/// The BfOp enum represents the different Brainfuck operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BfOp {
    IncrementPointer(usize), // >
    DecrementPointer(usize), // <
    IncrementByte(u8),       // +
    DecrementByte(u8),       // -
    OutputByte,              // .
    InputByte,               // ,
    Loop(Vec<BfOp>),         // [ ... ]
}

/// Parser for Brainfuck programs.
pub struct Parser {
    /// The tokens to be parsed.
    tokens: VecDeque<Token>,
    /// The current position in the token stream.
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into(),
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<BfOp>, ParseError> {
        self.parse_sequence()
    }

    fn parse_sequence(&mut self) -> Result<Vec<BfOp>, ParseError> {
        let mut ops = Vec::new();

        while !self.tokens.is_empty() {
            let current_token = self.tokens.front().cloned();
            match current_token {
                Some(Token::LoopEnd) => break,
                Some(token) => {
                    self.tokens.pop_front();
                    self.position += 1;

                    match token {
                        Token::IncrementPointer => {
                            let count = self.count_consecutive(Token::IncrementPointer) + 1;
                            ops.push(BfOp::IncrementPointer(count));
                        }
                        Token::DecrementPointer => {
                            let count = self.count_consecutive(Token::DecrementPointer) + 1;
                            ops.push(BfOp::DecrementPointer(count));
                        }
                        Token::IncrementByte => {
                            let count = self.count_consecutive(Token::IncrementByte) + 1;
                            ops.push(BfOp::IncrementByte(count as u8));
                        }
                        Token::DecrementByte => {
                            let count = self.count_consecutive(Token::DecrementByte) + 1;
                            ops.push(BfOp::DecrementByte(count as u8));
                        }
                        Token::OutputByte => ops.push(BfOp::OutputByte),
                        Token::InputByte => ops.push(BfOp::InputByte),
                        Token::LoopStart => {
                            let loop_body = self.parse_sequence()?;

                            if self.tokens.pop_front() != Some(Token::LoopEnd) {
                                return Err(ParseError::UnmatchedLoopStart {
                                    position: Some(self.position),
                                });
                            }
                            self.position += 1;
                            ops.push(BfOp::Loop(loop_body));
                        }
                        Token::LoopEnd => {
                            return Err(ParseError::UnmatchedLoopEnd {
                                position: Some(self.position),
                            })
                        }
                    }
                }
                None => break,
            }
        }

        Ok(ops)
    }

    /// Count the number of consecutive tokens of a given type at the front of the queue.  
    /// If any tokens are counted, they are removed from the queue.
    fn count_consecutive(&mut self, token_type: Token) -> usize {
        let mut count = 0;

        while self.tokens.get(count) == Some(&&token_type) {
            count += 1;
        }

        if count > 0 {
            self.tokens.drain(0..count);
            self.position += count;
        }

        count
    }
}
