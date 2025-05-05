use crate::interpreter::InterpreterError;
use crate::parser::BfOp;
#[cfg(feature = "optimizer")]
use crate::parser::OptimizedOp;
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
