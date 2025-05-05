#![cfg(feature = "optimizer")]

//! Brainfuck optimizer.
//!
//! This module provides an optimizer for Brainfuck programs,
//! which applies various optimization rules to improve the performance
//! and efficiency of the code.
//!
//! This module is available only when the `optimizer` feature is enabled.

mod optimization_rule;
mod optimizer;
mod rules;

pub use optimization_rule::OptimizationRule;
pub use optimizer::Optimizer;
