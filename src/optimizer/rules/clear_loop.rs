use crate::optimizer::OptimizationRule;
use crate::parser::{BfOp, OptimizedOp};

/// Rule to optimize clear loops like `[+]` or `[-]` to set memory cell to 0.
pub struct ClearLoopRule {}

impl OptimizationRule for ClearLoopRule {
    fn apply(&self, ops: &[BfOp]) -> Option<(Vec<BfOp>, usize)> {
        if ops.is_empty() {
            return None;
        }

        if let BfOp::Loop(body) = &ops[0] {
            if body.len() == 1 {
                if let BfOp::Increment(_) = body[0] {
                    // Detected a `[+]` or `[-]` loop, set cell to 0
                    // Consumes 1 BfOp (the loop itself) and replaces it with a clear cell operation
                    return Some((vec![BfOp::Optimized(OptimizedOp::ClearCell)], 1));
                }
            }
        }

        None
    }
}
