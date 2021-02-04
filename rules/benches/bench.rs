use criterion::{criterion_group, criterion_main, Criterion};

use rules::main_loop;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("main", |b| b.iter(|| main_loop(false)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
