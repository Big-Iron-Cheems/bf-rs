#![cfg(feature = "optimizer")]

use crate::optimizer::optimizer::OptimizationRule;
use crate::parser::{BfOp, OptimizedOp};

/// Rule to optimize clear loops like `[+]` or `[-]` to set memory cell to 0.
pub struct ClearLoopRule {}

impl OptimizationRule for ClearLoopRule {
    fn name(&self) -> &'static str {
        "Clear Loop"
    }

    fn apply(&self, ops: &[BfOp]) -> Option<Vec<BfOp>> {
        if ops.is_empty() {
            return None;
        }

        if let BfOp::Loop(body) = &ops[0] {
            if body.len() == 1 {
                if let BfOp::IncrementByte(_) | BfOp::DecrementByte(_) = body[0] {
                    // Detected a `[+]` or `[-]` loop, set cell to 0
                    return Some(vec![BfOp::Optimized(OptimizedOp::ClearCell)]);
                }
            }
        }

        None
    }
}
