# bf-rs: Brainfuck Interpreter in Rust

A simple Brainfuck interpreter written in Rust.

## Features

- **Standard Interpreter**: Executes standard Brainfuck code
- **Optional Optimizer**: Improves execution speed with pattern recognition
- **Debugging Tools**: Helps with development and troubleshooting
- **Performance Benchmarking**: Measure execution speed with [Criterion](https://github.com/bheisler/criterion.rs)

## Installation

```bash
# Clone the repository
git clone https://github.com/Big-Iron-Cheems/bf-rs.git
cd bf-rs

# Build the project
cargo build --release
```

## Basic Usage

```bash
# Run a Brainfuck program
cargo run -- path/to/your/program.bf

# Or use the compiled binary directly
./target/release/bf-rs path/to/your/program.bf
```

## Feature Flags

The interpreter supports several optional features you can enable:

### Debug Mode

Enables detailed logging and debugging information:

```bash
cargo run --features debug -- path/to/your/program.bf
```

### Optimizer

Enables optimization patterns that improve execution speed:

```bash
cargo run --features optimizer -- path/to/your/program.bf
```

### Combined Features

You can enable multiple features at once:

```bash
cargo run --features "debug optimizer" -- path/to/your/program.bf
```

## Performance Benchmarking

The project includes benchmarks using the Criterion framework:

### Running All Benchmarks

```bash
# Standard benchmarks (no optimizations)
cargo bench

# With optimizer enabled
cargo bench --features optimizer
```

### Targeted Benchmarks

```bash
# Only benchmark the full interpreter pipeline
cargo bench -- full

# Only benchmark individual components
cargo bench -- components

# Full pipeline with optimizer enabled
cargo bench --features optimizer -- full
```

## Optimization Details

When the `optimizer` feature is enabled, the interpreter recognizes common Brainfuck patterns and replaces them with
optimized operations:

- **Clear Cell**: Patterns like `[-]` or `[+]` are optimized to directly zero a cell
