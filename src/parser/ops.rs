use std::fmt;
use std::fmt::Formatter;

/// The BfOp enum represents the different Brainfuck operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BfOp {
    IncrementPointer(usize), // >
    DecrementPointer(usize), // <
    IncrementByte(u8),       // +
    DecrementByte(u8),       // -
    OutputByte,              // .
    InputByte,               // ,
    Loop(Vec<BfOp>),         // [ ... ]

    // Optimized operations
    #[cfg(feature = "optimizer")]
    Optimized(OptimizedOp),
}

/// The OptimizedOp enum represents optimized operations that can be applied to Brainfuck programs.
/// These are not standard Brainfuck operations, and are toggled by the `optimizer` feature.
#[cfg(feature = "optimizer")]
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizedOp {
    ClearCell, // [+] or [-]
}

impl fmt::Display for BfOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BfOp::IncrementPointer(count) => write!(
                f,
                ">{}",
                if *count > 1 {
                    count.to_string()
                } else {
                    String::new()
                }
            ),
            BfOp::DecrementPointer(count) => write!(
                f,
                "<{}",
                if *count > 1 {
                    count.to_string()
                } else {
                    String::new()
                }
            ),
            BfOp::IncrementByte(count) => write!(
                f,
                "+{}",
                if *count > 1 {
                    count.to_string()
                } else {
                    String::new()
                }
            ),
            BfOp::DecrementByte(count) => write!(
                f,
                "-{}",
                if *count > 1 {
                    count.to_string()
                } else {
                    String::new()
                }
            ),
            BfOp::OutputByte => write!(f, "."),
            BfOp::InputByte => write!(f, ","),
            BfOp::Loop(ops) => {
                write!(f, "[")?;
                for op in ops {
                    write!(f, "{}", op)?;
                }
                write!(f, "]")
            }
            #[cfg(feature = "optimizer")]
            BfOp::Optimized(opt_op) => opt_op.fmt(f),
        }
    }
}

#[cfg(feature = "optimizer")]
impl fmt::Display for OptimizedOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OptimizedOp::ClearCell => write!(f, "[0]"), // TODO: figure out a good repr for this
        }
    }
}
