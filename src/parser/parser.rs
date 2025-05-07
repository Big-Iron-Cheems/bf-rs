use crate::{lexer::Token, parser::BfOp, parser::ParseError};
use std::collections::VecDeque;

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
        self.parse_sequence(false, None)
    }

    fn parse_sequence(
        &mut self,
        inside_loop: bool,
        loop_start_pos: Option<usize>,
    ) -> Result<Vec<BfOp>, ParseError> {
        let mut ops = Vec::new();

        while !self.tokens.is_empty() {
            let current_token = self.tokens.front().cloned();
            match current_token {
                Some(Token::LoopEnd) => {
                    if inside_loop {
                        break;
                    } else {
                        self.tokens.pop_front();
                        return Err(ParseError::UnmatchedLoopEnd {
                            position: Some(self.position),
                        });
                    }
                }
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
                            let loop_start_position = self.position - 1;
                            let loop_body = self.parse_sequence(true, Some(loop_start_position))?;

                            if self.tokens.pop_front() != Some(Token::LoopEnd) {
                                return Err(ParseError::UnmatchedLoopStart {
                                    position: Some(loop_start_position),
                                });
                            }
                            self.position += 1;
                            ops.push(BfOp::Loop(loop_body));
                        }
                        Token::LoopEnd => {
                            return Err(ParseError::UnmatchedLoopEnd {
                                position: Some(self.position - 1),
                            })
                        }
                    }
                }
                None => {
                    if inside_loop {
                        return Err(ParseError::UnmatchedLoopStart {
                            position: loop_start_pos,
                        });
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(ops)
    }

    /// Counts and removes consecutive tokens of the specified type from the front of the queue.
    ///
    /// # Arguments
    /// * `token_type` - The token type to count (must be a repeatable token type)
    ///
    /// # Returns
    /// The number of consecutive matching tokens found (may be capped by type-specific limits)
    ///
    /// # Details
    /// - For byte operations (`IncrementByte`/`DecrementByte`), counts up to `u8::MAX` (255)
    /// - For pointer operations (`IncrementPointer`/`DecrementPointer`), counts up to `usize::MAX - 1`
    /// - Matching tokens are removed from the queue and position is adjusted accordingly
    /// - Issues a warning when byte operations exceed the `u8::MAX` limit
    ///
    /// # Panics
    /// If called with a token type that doesn't support consecutive counting (e.g., `OutputByte`, `InputByte`, etc.)
    fn count_consecutive(&mut self, token_type: Token) -> usize {
        let limit = match token_type {
            Token::IncrementByte | Token::DecrementByte => u8::MAX as usize,
            Token::IncrementPointer | Token::DecrementPointer => usize::MAX - 1,
            _ => panic!(
                "Unexpected token type for count_consecutive: {:?}",
                token_type
            ),
        };
        let mut count = 0;

        while count < limit && self.tokens.get(count) == Some(&token_type) {
            count += 1;
        }

        if count == limit && self.tokens.get(count) == Some(&&token_type) {
            let mut excess = 0;
            while self.tokens.get(count + excess) == Some(&token_type) {
                excess += 1;
            }

            if token_type == Token::IncrementByte || token_type == Token::DecrementByte {
                eprintln!(
                    "Warning: More than {} consecutive `{}` tokens truncated to maximum value.",
                    limit, token_type
                );
            }
        }

        if count > 0 {
            self.tokens.drain(0..count);
            self.position += count;
        }

        count
    }
}
