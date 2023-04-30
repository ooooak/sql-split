use reader::Reader;
use criterion::{
    criterion_group, 
    criterion_main, 
    Criterion
};
use std::fs::{
    File,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get large file", |b| b.iter(|| {
        let file = File::open("../../resources/test_files/big.sql").unwrap();
        let mut reader = Reader::new(file);
        loop {
            if reader.get() == 0 {
                break;
            }
        }
    }));

    c.bench_function("get large file", |b| b.iter(|| {
        let file = File::open("../../resources/test_files/big.sql").unwrap();
        let mut reader = Reader::new(file);
        loop {
            if reader.get() == 0 {
                break;
            }
        }
    }));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);