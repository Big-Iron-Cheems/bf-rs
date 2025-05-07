use crate::optimizer::OptimizationRule;
use crate::parser::BfOp;

/// Rule to optimize sequences of byte increments/decrements.
///
/// # Example
///
/// These sequences are optimized as follows:
/// - `+-+-+` => `+` (net change: +1)
/// - `-+-+-` => `-` (net change: -1)
/// - `++--` => `0` (net change: 0)
///
/// This rule handles wrapping around the byte value (0-255) safely.
pub struct ByteAdjustmentRule {}

impl OptimizationRule for ByteAdjustmentRule {
    fn apply(&self, ops: &[BfOp]) -> Option<(Vec<BfOp>, usize)> {
        if ops.is_empty() {
            return None;
        }

        let mut consumed = 0;
        let mut net_change = 0u8;

        for op in ops {
            match op {
                BfOp::IncrementByte(count) => {
                    net_change = net_change.wrapping_add(*count);
                    consumed += 1;
                }
                BfOp::DecrementByte(count) => {
                    net_change = net_change.wrapping_sub(*count);
                    consumed += 1;
                }
                _ => break,
            }
        }

        if consumed <= 1 {
            return None;
        }

        let result = match net_change {
            0 => vec![],
            n if n <= 128 => vec![BfOp::IncrementByte(n)],
            n => vec![BfOp::DecrementByte(n.wrapping_neg())],
        };

        Some((result, consumed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_adjustment_rule() {
        let rule = ByteAdjustmentRule {};

        // Test case: Increment and decrement operations
        let ops = vec![
            BfOp::IncrementByte(5),
            BfOp::DecrementByte(3),
            BfOp::IncrementByte(2),
        ];
        let result = rule.apply(&ops);
        assert_eq!(result, Some((vec![BfOp::IncrementByte(4)], 3)));

        // Test case: No net change
        let ops = vec![BfOp::IncrementByte(5), BfOp::DecrementByte(5)];
        let result = rule.apply(&ops);
        assert_eq!(result, Some((vec![], 2)));

        // Test case: Single operation
        let ops = vec![BfOp::IncrementByte(5)];
        let result = rule.apply(&ops);
        assert_eq!(result, None);

        // Test case: Empty input
        let ops: Vec<BfOp> = vec![];
        let result = rule.apply(&ops);
        assert_eq!(result, None);

        // Test case: Wrapping from positive to negative
        let ops = vec![BfOp::IncrementByte(255), BfOp::IncrementByte(2)];
        let result = rule.apply(&ops);
        assert_eq!(result, Some((vec![BfOp::IncrementByte(1)], 2)));

        // Test case: Wrapping from negative to positive
        let ops = vec![BfOp::DecrementByte(255), BfOp::DecrementByte(2)];
        let result = rule.apply(&ops);
        assert_eq!(result, Some((vec![BfOp::DecrementByte(1)], 2)));

        // Test case: Large increments that wrap multiple times (to the upper limit)
        let ops = vec![BfOp::IncrementByte(200), BfOp::IncrementByte(200)];
        let result = rule.apply(&ops);
        // (200 + 200) % 256 = 144
        // 256 - 144 = 112 → it's shorter to go backward by 112
        // So we use DecrementByte(112) as the shortest path to reach net +144
        assert_eq!(result, Some((vec![BfOp::DecrementByte(112)], 2)));

        // Test case: Large increments that wrap multiple times (to the lower limit)
        let ops = vec![BfOp::DecrementByte(200), BfOp::DecrementByte(200)];
        let result = rule.apply(&ops);
        // (-200 - 200) % 256 = -400 % 256 = 112
        // 112 in 0..=128, it's shorter to go forward by 112
        // So we use IncrementByte(112) as the shortest path to reach net -400
        assert_eq!(result, Some((vec![BfOp::IncrementByte(112)], 2)));
    }
}
