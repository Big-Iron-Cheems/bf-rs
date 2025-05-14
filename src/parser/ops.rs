use std::fmt;
use std::fmt::Formatter;
use std::num::Wrapping;

/// The BfOp enum represents the different Brainfuck operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BfOp {
    PointerIncrement(isize), // > or <
    Increment(Wrapping<i8>), // + or -
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
            BfOp::PointerIncrement(offset) => {
                if *offset == 0 {
                    write!(f, "")
                } else {
                    let (symbol, value) = if *offset > 0 {
                        (">", *offset)
                    } else {
                        ("<", -offset)
                    };

                    if value == 1 {
                        write!(f, "{}", symbol)
                    } else {
                        write!(f, "{}{}", symbol, value)
                    }
                }
            }
            BfOp::Increment(count) => {
                if count.0 == 0 {
                    write!(f, "")
                } else {
                    let (symbol, value) = if count.0 > 0 {
                        ("+", count.0)
                    } else {
                        ("-", count.0.abs())
                    };

                    if value == 1 {
                        write!(f, "{}", symbol)
                    } else {
                        write!(f, "{}{}", symbol, value)
                    }
                }
            }
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
            OptimizedOp::ClearCell => write!(f, "[0]"),
        }
    }
}
