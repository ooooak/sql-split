use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use sql_split_reader::Reader;

pub fn criterion_benchmark(c: &mut Criterion) {
    for _ in 0..2 { // Run the whole benchmark setup twice
        c.bench_function("get large file", |b| {
            b.iter(|| {
                let file = File::open("../resources/test_db/small.sql").unwrap();
                let mut reader = Reader::new(file);
                while reader.peek() != 0 {
                    reader.get();
                }
            });
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
