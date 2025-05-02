//! Brainfuck lexer.  
//! It takes a Brainfuck program as string and returns a vector of tokens.

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

/// Lexer for Brainfuck programs.
pub struct Lexer<'a> {
    /// The source string as bytes.
    source: &'a [u8],
    /// The current position in the source string.
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source: source.as_bytes(),
            position: 0,
        }
    }

    /// Generate tokens from the source string.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(self.source.len()); // Pre-allocating more than enough memory
        tokens.extend(std::iter::from_fn(|| self.new_token()));
        tokens
    }

    /// Create a new token from the source string.
    fn new_token(&mut self) -> Option<Token> {
        while self.position < self.source.len() {
            let current_byte = self.source[self.position];
            self.position += 1;

            match current_byte {
                b'>' => return Some(Token::IncrementPointer),
                b'<' => return Some(Token::DecrementPointer),
                b'+' => return Some(Token::IncrementByte),
                b'-' => return Some(Token::DecrementByte),
                b'.' => return Some(Token::OutputByte),
                b',' => return Some(Token::InputByte),
                b'[' => return Some(Token::LoopStart),
                b']' => return Some(Token::LoopEnd),
                _ => continue, // Ignore non-command characters
            }
        }
        None
    }
}
