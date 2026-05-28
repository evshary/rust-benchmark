# builtin-bench

This crate demonstrates a Rust built-in benchmark target using Cargo's `bench` command.

## Run the benchmark

This benchmark uses Rust's built-in test harness and requires a nightly toolchain.

```bash
cd builtin-bench
cargo +nightly bench
```

## What is included

- `benches/builtin_bench.rs` — benchmark entrypoint for the crate
- `Cargo.toml` — package manifest with a benchmark target
- `src/lib.rs` — library code under test

## Benchmark example

The sample benchmark sorts a reverse-ordered vector of integers using `sort_unstable()`.

If you want to add more benchmarks, create additional files under `benches/` or add more `#[bench]` functions.
