use crate::{
    error::BfError, interpreter::Interpreter, lexer::Lexer, optimizer::optimizer::Optimizer,
    parser::write_ops_to_file, parser::Parser,
};
use std::fs;
use std::fs::File;

mod error;
mod interpreter;
mod lexer;
mod optimizer;
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

    // Dump the parsed program to a file
    let mut file = File::create("parsed_program.bf")?;
    write_ops_to_file(&program, &mut file, 0)?;

    // Step 2.1: Optimization - apply optimization rules
    let optimizer = Optimizer::new();
    let optimized_program = optimizer.optimize(program);

    // Dump the optimized program to a file
    let mut file = File::create("optimized_program.bf")?;
    write_ops_to_file(&optimized_program, &mut file, 0)?;

    // Step 3: Execution
    let mut interpreter = Interpreter::new();
    interpreter.run(&optimized_program)?;

    Ok(())
}
