#![cfg(feature = "debug")]
#![allow(dead_code)]

//! Debug utilities for the Brainfuck interpreter.
//!
//! These utilities are available only when the `debug` feature is enabled.

use crate::parser::BfOp;
#[cfg(feature = "optimizer")]
use crate::parser::OptimizedOp;
use std::collections::HashMap;

pub fn print_op_stats(ops: &[BfOp]) {
    let mut basic_stats = HashMap::new();
    let mut optimized_stats = HashMap::new();
    count_ops(ops, &mut basic_stats, &mut optimized_stats);

    println!("Basic Operations:");
    println!(
        "  Increment pointer (>): {}",
        basic_stats.get("increment_pointer").unwrap_or(&0)
    );
    println!(
        "  Decrement pointer (<): {}",
        basic_stats.get("decrement_pointer").unwrap_or(&0)
    );
    println!(
        "  Increment byte (+): {}",
        basic_stats.get("increment_byte").unwrap_or(&0)
    );
    println!(
        "  Decrement byte (-): {}",
        basic_stats.get("decrement_byte").unwrap_or(&0)
    );
    println!(
        "  Output byte (.): {}",
        basic_stats.get("output_byte").unwrap_or(&0)
    );
    println!(
        "  Input byte (,): {}",
        basic_stats.get("input_byte").unwrap_or(&0)
    );
    println!("  Loops ([...]): {}", basic_stats.get("loop").unwrap_or(&0));

    let basic_total: usize = basic_stats.values().sum();
    println!("Total Basic Operations: {}", basic_total);

    if !optimized_stats.is_empty() {
        println!("Optimized Operations:");
        println!(
            "  Clear cell: {}",
            optimized_stats.get("clear_cell").unwrap_or(&0)
        );
        let optimized_total: usize = optimized_stats.values().sum();
        println!("Total Optimized Operations: {}", optimized_total);
    } else {
        println!("No optimized operations found.");
    }
}

fn count_ops(
    ops: &[BfOp],
    basic_stats: &mut HashMap<&'static str, usize>,
    optimized_stats: &mut HashMap<&'static str, usize>,
) {
    for op in ops {
        match op {
            // Basic BF operations
            BfOp::IncrementPointer(_) => *basic_stats.entry("increment_pointer").or_insert(0) += 1,
            BfOp::DecrementPointer(_) => *basic_stats.entry("decrement_pointer").or_insert(0) += 1,
            BfOp::IncrementByte(_) => *basic_stats.entry("increment_byte").or_insert(0) += 1,
            BfOp::DecrementByte(_) => *basic_stats.entry("decrement_byte").or_insert(0) += 1,
            BfOp::OutputByte => *basic_stats.entry("output_byte").or_insert(0) += 1,
            BfOp::InputByte => *basic_stats.entry("input_byte").or_insert(0) += 1,
            BfOp::Loop(body) => {
                *basic_stats.entry("loop").or_insert(0) += 1;
                count_ops(body, basic_stats, optimized_stats);
            }

            // Optimized operations
            #[cfg(feature = "optimizer")]
            BfOp::Optimized(opt_op) => match opt_op {
                OptimizedOp::ClearCell => {
                    *optimized_stats.entry("clear_cell").or_insert(0) += 1;
                }
            },
        }
    }
}

/// Prints the program's operations to stdout with indentation.
pub fn print_ops(ops: &[BfOp], indent_level: usize) {
    for op in ops {
        match op {
            BfOp::Loop(body) => {
                println!("{}[", " ".repeat(indent_level));
                print_ops(body, indent_level + 2);
                println!("{}]", " ".repeat(indent_level));
            }
            _ => println!("{}{}", " ".repeat(indent_level), op),
        }
    }
}

/// Writes the program's operations to a file with indentation.
pub fn write_ops_to_file(
    ops: &[BfOp],
    file: &mut impl std::io::Write,
    indent_level: usize,
) -> std::io::Result<()> {
    for op in ops {
        match op {
            BfOp::Loop(body) => {
                writeln!(file, "{}[", " ".repeat(indent_level))?;
                write_ops_to_file(body, file, indent_level + 2)?;
                writeln!(file, "{}]", " ".repeat(indent_level))?;
            }
            _ => writeln!(file, "{}{}", " ".repeat(indent_level), op)?,
        }
    }
    Ok(())
}
