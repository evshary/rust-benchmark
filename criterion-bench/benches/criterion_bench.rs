use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use criterion_bench::{prepare_numbers, sort_numbers};

fn bench_sort_reverse_order(c: &mut Criterion) {
    c.bench_function("sort reverse order", |b| {
        b.iter(|| {
            let mut data = prepare_numbers();
            sort_numbers(black_box(&mut data));
        })
    });
}

criterion_group!(benches, bench_sort_reverse_order);
criterion_main!(benches);
