use crate::{error::BfError, interpreter::Interpreter, lexer::Lexer, parser::Parser};
use std::fs;

mod error;
mod interpreter;
mod lexer;
mod parser;

fn main() -> Result<(), BfError> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <brainfuck_file>", args[0]);
        return Ok(());
    }

    let source = fs::read_to_string(&args[1])?;

    // Step 1: Lexical analysis - convert source to tokens
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    // Step 2: Parsing - convert tokens to abstract syntax tree
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    // Step 3: Execution
    let mut interpreter = Interpreter::new();
    interpreter.run(&program)?;

    Ok(())
}
