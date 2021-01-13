use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn uuid_benchmark(c: &mut Criterion) {
    c.bench_function("uuid", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, uuid_benchmark);
criterion_main!(benches);
