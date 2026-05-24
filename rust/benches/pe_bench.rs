use criterion::{Criterion, criterion_group, criterion_main};
use project_euler_rust::solutions::solution_8::{solution_8_functional, solution_8_procedural};
use std::hint::black_box;

fn bench_8_procedural(c: &mut Criterion) {
    c.bench_function("solution_8_procedural", |b| {
        b.iter(|| solution_8_procedural())
    });
}

fn bench_8_functional(c: &mut Criterion) {
    c.bench_function("solution_8_functional", |b| {
        b.iter(|| solution_8_functional())
    });
}

criterion_group!(bench_8, bench_8_procedural, bench_8_functional);
criterion_main!(bench_8);
