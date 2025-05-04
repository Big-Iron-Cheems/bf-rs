//! Brainfuck interpreter.  
//! This module contains the implementation of a Brainfuck interpreter in Rust.

use crate::parser::BfOp;
#[cfg(feature = "optimizer")]
use crate::parser::OptimizedOp;
use std::error::Error;
use std::io::{Read, Write};
use std::{fmt, io};

/// Errors that may occur during program execution.
#[derive(Debug)]
pub enum InterpreterError {
    /// Attempted to move the pointer before the beginning of memory
    PointerUnderflow {
        position: usize,
        attempted_move: usize,
    },
    /// Error while reading from input
    InputError(io::Error),
    /// Error while writing to output
    OutputError(io::Error),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::PointerUnderflow {
                position,
                attempted_move,
            } => write!(f, "Pointer underflow: attempted to move left {} steps when pointer was at position {}",
                    attempted_move, position),
            InterpreterError::InputError(err) => write!(f, "Input error: {}", err),
            InterpreterError::OutputError(err) => write!(f, "Output error: {}", err),
        }
    }
}

impl Error for InterpreterError {}

/// The `Interpreter` struct represents the state of the Brainfuck interpreter.
pub struct Interpreter {
    /// Memory used by the interpreter.
    memory: Vec<u8>,
    /// Pointer to the current position in the memory.
    pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            memory: vec![0; 30_000], // Initialize memory with standard 30_000 cells
            pointer: 0,
        }
    }

    pub fn run(&mut self, program: &[BfOp]) -> Result<(), InterpreterError> {
        let stdout = io::stdout();
        let mut stdout_handle = stdout.lock();
        let stdin = io::stdin();
        let mut stdin_handle = stdin.lock();

        self.execute(program, &mut stdout_handle, &mut stdin_handle)
    }

    pub fn execute(
        &mut self,
        ops: &[BfOp],
        stdout: &mut impl Write,
        stdin: &mut impl Read,
    ) -> Result<(), InterpreterError> {
        for op in ops {
            match op {
                BfOp::IncrementPointer(count) => {
                    self.pointer += count;
                    while self.pointer >= self.memory.len() {
                        self.memory.push(0);
                    }
                }
                BfOp::DecrementPointer(count) => {
                    let count = *count;
                    if self.pointer >= count {
                        self.pointer -= count;
                    } else {
                        return Err(InterpreterError::PointerUnderflow {
                            position: self.pointer,
                            attempted_move: count,
                        });
                    }
                }
                BfOp::IncrementByte(count) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(*count);
                }
                BfOp::DecrementByte(count) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(*count);
                }
                BfOp::OutputByte => {
                    stdout
                        .write_all(&[self.memory[self.pointer]])
                        .map_err(|e| InterpreterError::OutputError(e))?;
                    stdout
                        .flush()
                        .map_err(|e| InterpreterError::OutputError(e))?;
                }
                BfOp::InputByte => {
                    let mut buffer = [0];
                    match stdin.read_exact(&mut buffer) {
                        Ok(_) => self.memory[self.pointer] = buffer[0],
                        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                            // Handle EOF by keeping the cell unchanged
                        }
                        Err(e) => return Err(InterpreterError::InputError(e)),
                    }
                }
                BfOp::Loop(body) => {
                    while self.memory[self.pointer] != 0 {
                        self.execute(body, stdout, stdin)?
                    }
                }
                #[cfg(feature = "optimizer")]
                BfOp::Optimized(opt_op) => match opt_op {
                    OptimizedOp::ClearCell => {
                        self.memory[self.pointer] = 0;
                    }
                },
            }
        }

        Ok(())
    }
}
