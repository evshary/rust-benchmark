# Rust benchmark

This is a simple repository to demonstrate how to run benchmark for Rust functions.

## Crates

- `builtin-bench`: Demonstrates Rust's built-in benchmark support using the `#[bench]` attribute and the test harness. This requires a nightly toolchain.

  - Code under test: [builtin-bench/src/lib.rs](builtin-bench/src/lib.rs#L1)
  - Benchmark: [builtin-bench/benches/builtin_bench.rs](builtin-bench/benches/builtin_bench.rs#L1)

- `criterion-bench`: Uses the `criterion` crate for statistically robust benchmarking with reports and configurable measurement. This runs on stable Rust.

  - Code under test: [criterion-bench/src/lib.rs](criterion-bench/src/lib.rs#L1)
  - Benchmark: [criterion-bench/benches/criterion_bench.rs](criterion-bench/benches/criterion_bench.rs#L1)

Both crates contain a small sample that sorts a reverse-ordered `Vec<u32>` and show how to separate setup from measured work (Criterion uses `iter_batched`).
