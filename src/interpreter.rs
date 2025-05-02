//! Brainfuck interpreter.  
//! This module contains the implementation of a Brainfuck interpreter in Rust.

use crate::parser::BfOp;
use std::io;
use std::io::{Read, Write};

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

    pub fn run(&mut self, program: &[BfOp]) -> io::Result<()> {
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
    ) -> io::Result<()> {
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
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Pointer underflow",
                        ));
                    }
                }
                BfOp::IncrementByte(count) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(*count);
                }
                BfOp::DecrementByte(count) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(*count);
                }
                BfOp::OutputByte => {
                    stdout.write_all(&[self.memory[self.pointer]])?;
                    stdout.flush()?;
                }
                BfOp::InputByte => {
                    let mut buffer = [0];
                    if stdin.read_exact(&mut buffer).is_ok() {
                        self.memory[self.pointer] = buffer[0];
                    }
                }
                BfOp::Loop(body) => {
                    while self.memory[self.pointer] != 0 {
                        self.execute(body, stdout, stdin)?
                    }
                }
            }
        }

        Ok(())
    }
}
