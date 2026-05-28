#![feature(test)]

extern crate test;

use test::Bencher;

use builtin_bench::{prepare_numbers, sort_numbers};

#[bench]
fn bench_sort_reverse_order(b: &mut Bencher) {
    b.iter(|| {
        let mut data = prepare_numbers();
        sort_numbers(&mut data);
        test::black_box(data);
    });
}
