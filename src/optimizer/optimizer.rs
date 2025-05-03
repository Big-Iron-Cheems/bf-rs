use crate::optimizer::rules::ClearLoopRule;
use crate::parser::BfOp;

/// Represents an optimization that can be applied to a Brainfuck program.
pub trait OptimizationRule {
    /// Name of the optimization rule.
    fn name(&self) -> &'static str;
    /// Applies the optimization rule to a slice of BfOp operations.
    /// Returns `Some(Vec<BfOp>)` with the optimized operations if any optimizations were made,
    fn apply(&self, ops: &[BfOp]) -> Option<Vec<BfOp>>;
}

/// Optimizer for Brainfuck programs.
pub struct Optimizer {
    rules: Vec<Box<dyn OptimizationRule>>,
}

impl Optimizer {
    /// Create an optimizer with the default set of optimization rules.
    pub fn new() -> Self {
        let mut optimizer = Self { rules: Vec::new() };
        optimizer.register_default_rules();
        optimizer
    }

    /// Create an optimizer with no rules.
    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    /// Register an optimization rule.
    pub fn register_rule(&mut self, rule: Box<dyn OptimizationRule>) {
        self.rules.push(rule);
    }

    /// Register the default set of optimization rules.
    fn register_default_rules(&mut self) {
        self.register_rule(Box::new(ClearLoopRule {}));
        // Register other rules here
    }

    /// Optimize a Brainfuck program.
    pub fn optimize(&self, program: Vec<BfOp>) -> Vec<BfOp> {
        self.optimize_ops(program)
    }

    fn optimize_ops(&self, ops: Vec<BfOp>) -> Vec<BfOp> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < ops.len() {
            let mut optimized = false;

            // Try to find a pattern at the current position
            for rule in &self.rules {
                if let Some(replacement) = rule.apply(&ops[i..]) {
                    result.extend(replacement);
                    // Skip ahead based on pattern length
                    // Assuming the rule returns the number of BfOps it replaced
                    i += self.rule_pattern_length(&ops[i..], rule.as_ref());
                    optimized = true;
                    break;
                }
            }

            // If no optimization was applied, just copy the current operation
            if !optimized {
                match &ops[i] {
                    BfOp::Loop(body) => {
                        let optimized_body = self.optimize_ops(body.clone());
                        result.push(BfOp::Loop(optimized_body));
                    }
                    op => result.push(op.clone()),
                }
                i += 1;
            }
        }

        result
    }

    fn rule_pattern_length(&self, ops: &[BfOp], rule: &dyn OptimizationRule) -> usize {
        match rule.name() {
            "Clear Loop" => 1,
            _ => 1, // Default to 1 for other rules
        }
    }
}
