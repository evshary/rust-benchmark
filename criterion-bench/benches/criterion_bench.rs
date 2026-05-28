use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use criterion_bench::{prepare_numbers, sort_numbers};

fn bench_sort_reverse_order(c: &mut Criterion) {
    c.bench_function("sort reverse order", |b| {
        // Use `iter_batched` to separate setup from the timed work.
        // - The first closure (`|| prepare_numbers()`) is the setup and runs
        //   outside the timed section. This prevents allocation or generation
        //   of inputs from affecting the measured time.
        // - The second closure (`|mut data| ...`) receives the prepared input
        //   and contains only the work you want to measure (`sort_numbers`).
        // - `BatchSize::SmallInput` controls how often the setup is re-run
        //   relative to measurement; choose an appropriate size for your case.
        b.iter_batched(
            || prepare_numbers(),
            |mut data| sort_numbers(black_box(&mut data)),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_sort_reverse_order);
criterion_main!(benches);
