# criterion-bench

This crate demonstrates a Rust benchmark using the `criterion` crate.

## Run the benchmark

```bash
cd criterion-bench
cargo bench
```

Criterion provides a production-quality benchmarking framework with statistical analysis, overhead measurement, and report generation.

You can see the result under `target/criterion/report/index.html`.

## What is included

- `benches/criterion_bench.rs` — Criterion benchmark definition
- `Cargo.toml` — package manifest with `criterion` dependency
- `src/lib.rs` — library code under test

## Benchmark example

The sample benchmark sorts a reverse-ordered vector of integers using `sort_unstable()`.

Add more benchmarks by extending `benches/criterion_bench.rs` or adding new benchmark files under `benches/`.
