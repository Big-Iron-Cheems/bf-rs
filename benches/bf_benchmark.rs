use bf_rs::{interpreter::Interpreter, lexer::Lexer, parser::Parser};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::io;

const SAMPLE_SIZE: usize = 10;
const EXAMPLES: &[(&str, &str)] = &[
    // ("hello_world", include_str!("../examples/hello_world.bf")),
    ("mandelbrot", include_str!("../examples/mandelbrot.bf")),
    // Add more examples as needed
];

/// Benchmark the entire pipeline
fn bench_full_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("Full Pipeline");
    group.sample_size(SAMPLE_SIZE);

    for (name, source) in EXAMPLES.iter() {
        group.bench_function(BenchmarkId::new("full", *name), |b| {
            b.iter(|| {
                // Measure the entire execution pipeline
                let mut lexer = Lexer::new(source);
                let tokens = lexer.tokenize();

                let mut parser = Parser::new(tokens);
                let program = parser.parse().expect("Parsing failed");

                let mut interpreter = Interpreter::new();
                // Use null I/O to avoid affecting benchmarks
                let null_writer = io::sink();
                let null_reader = io::empty();
                interpreter
                    .execute(
                        &program,
                        &mut io::BufWriter::new(null_writer),
                        &mut io::BufReader::new(null_reader),
                    )
                    .expect("Execution failed");
            });
        });
    }

    group.finish();
}

/// Benchmark individual components
fn bench_components(c: &mut Criterion) {
    let mut group = c.benchmark_group("Components");
    group.sample_size(SAMPLE_SIZE);

    for (name, source) in EXAMPLES.iter() {
        // Benchmark lexing
        group.bench_with_input(BenchmarkId::new("lexing", name), source, |b, src| {
            b.iter(|| {
                let mut lexer = Lexer::new(src);
                lexer.tokenize()
            });
        });

        // Benchmark parsing (with pre-lexed tokens)
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        group.bench_with_input(BenchmarkId::new("parsing", name), &tokens, |b, tokens| {
            b.iter(|| {
                let mut parser = Parser::new(tokens.clone());
                parser.parse().expect("Parsing failed")
            });
        });

        // Benchmark execution (with pre-parsed program)
        let mut parser = Parser::new(tokens);
        let program = parser.parse().expect("Parsing failed");
        group.bench_with_input(
            BenchmarkId::new("execution", name),
            &program,
            |b, program| {
                b.iter(|| {
                    let mut interpreter = Interpreter::new();
                    // Use null I/O to avoid affecting benchmarks
                    let null_writer = io::sink();
                    let null_reader = io::empty();
                    interpreter
                        .execute(
                            program,
                            &mut io::BufWriter::new(null_writer),
                            &mut io::BufReader::new(null_reader),
                        )
                        .expect("Execution failed")
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_full_pipeline, bench_components);
criterion_main!(benches);
