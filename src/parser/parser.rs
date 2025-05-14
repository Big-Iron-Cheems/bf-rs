use crate::{lexer::Token, parser::BfOp, parser::ParseError};
use std::collections::VecDeque;
use std::num::Wrapping;

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
                        Token::IncrementPointer | Token::DecrementPointer => {
                            let net = self.count_net_pointer_ops(token);
                            if net != 0 {
                                ops.push(BfOp::PointerIncrement(net));
                            }
                        }
                        Token::IncrementByte | Token::DecrementByte => {
                            let net = self.count_net_byte_ops(token);
                            if net != 0 {
                                ops.push(BfOp::Increment(Wrapping(net)));
                            }
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

    /// Counts the net pointer operations (increment/decrement) from the front of the queue.
    ///
    /// # Details
    /// If an overflow/underflow occurs, the net value is clamped to `isize::MAX` or `isize::MIN`.
    fn count_net_pointer_ops(&mut self, token: Token) -> isize {
        let mut net: isize = match token {
            Token::IncrementPointer => 1,
            Token::DecrementPointer => -1,
            _ => unreachable!("Unexpected token type for count_net_pointer_ops"),
        };

        let mut clamped = None;
        while let Some(next_token) = self.tokens.front() {
            match next_token {
                Token::IncrementPointer => {
                    match net.checked_add(1) {
                        Some(result) => net = result,
                        None => clamped = Some(isize::MAX),
                    }
                    self.tokens.pop_front();
                    self.position += 1;
                }
                Token::DecrementPointer => {
                    match net.checked_sub(1) {
                        Some(result) => net = result,
                        None => clamped = Some(isize::MIN),
                    }
                    self.tokens.pop_front();
                    self.position += 1;
                }
                _ => break,
            }
        }

        if let Some(limit) = clamped {
            eprintln!(
                "Warning: Pointer movement overflowed. Clamped to {}.",
                limit
            );
            return limit;
        }

        net
    }

    /// Counts the net byte operations (increment/decrement) from the front of the queue.
    ///
    /// # Details
    /// If an overflow/underflow occurs, the net value is clamped to `i8::MAX` or `i8::MIN`.
    fn count_net_byte_ops(&mut self, token: Token) -> i8 {
        let mut net: i8 = match token {
            Token::IncrementByte => 1,
            Token::DecrementByte => -1,
            _ => unreachable!("Unexpected token type for count_net_byte_ops"),
        };

        let mut clamped = None;
        while let Some(next_token) = self.tokens.front() {
            match next_token {
                Token::IncrementByte => {
                    match net.checked_add(1) {
                        Some(result) => net = result,
                        None => clamped = Some(i8::MAX),
                    }
                    self.tokens.pop_front();
                    self.position += 1;
                }
                Token::DecrementByte => {
                    match net.checked_sub(1) {
                        Some(result) => net = result,
                        None => clamped = Some(i8::MIN),
                    }
                    self.tokens.pop_front();
                    self.position += 1;
                }
                _ => break,
            }
        }

        if let Some(limit) = clamped {
            eprintln!("Warning: Byte operation overflowed. Clamped to {}.", limit);
            return limit;
        }

        net
    }
}
