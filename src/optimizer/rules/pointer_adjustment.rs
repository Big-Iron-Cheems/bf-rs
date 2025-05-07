use crate::optimizer::OptimizationRule;
use crate::parser::BfOp;

/// Rule to optimize sequences of pointer increments/decrements.
///
/// # Example
///
/// These sequences are optimized as follows:
/// - `><><>` => `>` (net movement: +1)
/// - `<><><` => `<` (net movement: -1)
/// - `>><<` => `` (net movement: 0)
///
/// This rule handles overflow of the shift by not applying in that case.
pub struct PointerAdjustmentRule {}

impl OptimizationRule for PointerAdjustmentRule {
    fn apply(&self, ops: &[BfOp]) -> Option<(Vec<BfOp>, usize)> {
        if ops.is_empty() {
            return None;
        }

        let mut consumed = 0;
        let mut right_shift = 0usize;
        let mut left_shift = 0usize;

        for op in ops {
            match op {
                BfOp::IncrementPointer(count) => {
                    match right_shift.checked_add(*count) {
                        Some(val) => right_shift = val,
                        None => return None,
                    }
                    consumed += 1;
                }
                BfOp::DecrementPointer(count) => {
                    match left_shift.checked_add(*count) {
                        Some(val) => left_shift = val,
                        None => return None,
                    }
                    consumed += 1;
                }
                _ => break,
            }
        }

        if consumed <= 1 {
            return None;
        }

        let result = if right_shift == left_shift {
            vec![]
        } else if right_shift > left_shift {
            vec![BfOp::IncrementPointer(right_shift - left_shift)]
        } else {
            vec![BfOp::DecrementPointer(left_shift - right_shift)]
        };

        Some((result, consumed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_adjustment_rule() {
        let rule = PointerAdjustmentRule {};

        // Test case: Basic sequence of increments and decrements
        let ops = vec![
            BfOp::IncrementPointer(1),
            BfOp::DecrementPointer(1),
            BfOp::IncrementPointer(1),
        ];
        let result = rule.apply(&ops);
        // Net movement: +1 - 1 + 1 = +1
        assert_eq!(result, Some((vec![BfOp::IncrementPointer(1)], 3)));

        // Test case: Sequence resulting in net left movement
        let ops = vec![
            BfOp::DecrementPointer(1),
            BfOp::IncrementPointer(1),
            BfOp::DecrementPointer(1),
        ];
        let result = rule.apply(&ops);
        // Net movement: -1 + 1 - 1 = -1
        assert_eq!(result, Some((vec![BfOp::DecrementPointer(1)], 3)));

        // Test case: Sequence resulting in no net movement
        let ops = vec![BfOp::IncrementPointer(2), BfOp::DecrementPointer(2)];
        let result = rule.apply(&ops);
        // Net movement: +2 - 2 = 0
        assert_eq!(result, Some((vec![], 2)));

        // Test case: Only increments
        let ops = vec![BfOp::IncrementPointer(3), BfOp::IncrementPointer(2)];
        let result = rule.apply(&ops);
        // Net movement: +3 + 2 = +5
        assert_eq!(result, Some((vec![BfOp::IncrementPointer(5)], 2)));

        // Test case: Only decrements
        let ops = vec![BfOp::DecrementPointer(4), BfOp::DecrementPointer(1)];
        let result = rule.apply(&ops);
        // Net movement: -4 - 1 = -5
        assert_eq!(result, Some((vec![BfOp::DecrementPointer(5)], 2)));

        // Test case: Single operation (should not apply)
        let ops = vec![BfOp::IncrementPointer(1)];
        let result = rule.apply(&ops);
        assert_eq!(result, None);

        // Test case: Empty input (should not apply)
        let ops: Vec<BfOp> = vec![];
        let result = rule.apply(&ops);
        assert_eq!(result, None);

        // Test case: Sequence followed by a non-pointer operation
        let ops = vec![
            BfOp::IncrementPointer(1),
            BfOp::DecrementPointer(1),
            BfOp::IncrementByte(1), // Non-pointer operation
            BfOp::IncrementPointer(1),
        ];
        let result = rule.apply(&ops);
        // Should only consume the first two pointer operations. Net movement: +1 - 1 = 0
        assert_eq!(result, Some((vec![], 2)));

        // Test case: Potential overflow (should return None)
        // This test relies on usize::MAX, which is platform-dependent.
        // We use checked_add in the rule to handle this safely.
        let ops = vec![
            BfOp::IncrementPointer(usize::MAX),
            BfOp::IncrementPointer(1),
        ];
        let result = rule.apply(&ops);
        assert_eq!(result, None);

        // Test case: Another potential overflow scenario
        let ops = vec![
            BfOp::DecrementPointer(usize::MAX),
            BfOp::DecrementPointer(1),
        ];
        let result = rule.apply(&ops);
        assert_eq!(result, None);

        // Test case: Large but non-overflowing shifts
        let ops = vec![
            BfOp::IncrementPointer(100000),
            BfOp::DecrementPointer(50000),
        ];
        let result = rule.apply(&ops);
        // Net movement: +100000 - 50000 = +50000
        assert_eq!(result, Some((vec![BfOp::IncrementPointer(50000)], 2)));
    }
}
