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
