use crate::{error::BfError, interpreter::Interpreter, lexer::Lexer, parser::Parser};
use std::fs;

#[cfg(feature = "optimizer")]
use crate::optimizer::Optimizer;

#[cfg(feature = "debug")]
use std::fs::File;

#[cfg(feature = "debug")]
mod debug;
mod error;
mod interpreter;
mod lexer;
#[cfg(feature = "optimizer")]
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

    #[cfg(feature = "debug")]
    {
        std::fs::create_dir_all("out")?;

        // Dump the parsed program to a file
        let mut file = File::create("out/parsed_program.bfir")?;
        debug::write_ops_to_file(&program, &mut file, 0)?;

        // Print the basic stats
        debug::print_op_stats(&program);
    }

    // Step 2.1: Optimization - apply optimization rules
    #[cfg(feature = "optimizer")]
    let program = {
        let optimizer = Optimizer::new();
        let optimized = optimizer.optimize(program);

        #[cfg(feature = "debug")]
        {
            let mut file = File::create("out/optimized_program.bfir")?;
            debug::write_ops_to_file(&optimized, &mut file, 0)?;
            debug::print_op_stats(&optimized);
        }

        optimized
    };

    // Step 3: Execution
    let mut interpreter = Interpreter::new();
    interpreter.run(&program)?;

    Ok(())
}
