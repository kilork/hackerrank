#[macro_use]

extern crate criterion;
extern crate morgan_and_a_string_bfs;

use criterion::Criterion;
use morgan_and_a_string_bfs::minimal_sequence_finder;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("minimal sequence finder", |b| b.iter(|| minimal_sequence_finder(
            "ABACABA", "ABACABA"
        )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
