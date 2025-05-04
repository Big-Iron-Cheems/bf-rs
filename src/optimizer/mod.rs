#![cfg(feature = "optimizer")]

//! Brainfuck optimizer.
//!
//! This module provides an optimizer for Brainfuck programs,
//! which applies various optimization rules to improve the performance
//! and efficiency of the code.
//!
//! This module is available only when the `optimizer` feature is enabled.

pub mod optimizer;
mod rules;
