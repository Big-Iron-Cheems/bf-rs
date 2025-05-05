use crate::parser::BfOp;

/// Represents an optimization that can be applied to a Brainfuck program.
pub trait OptimizationRule {
    /// Applies the optimization rule to a slice of BfOp operations.
    /// Returns `Some((Vec<BfOp>, usize))` with the optimized operations
    /// and the number of original operations consumed if any optimizations were made.
    fn apply(&self, ops: &[BfOp]) -> Option<(Vec<BfOp>, usize)>;
}
