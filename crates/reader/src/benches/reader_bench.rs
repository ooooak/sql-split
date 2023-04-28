use reader::Reader;
use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion
};
use std::fs::{
    File,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench large file", |b| b.iter(|| {
        let file = black_box(File::open("../../resources/test_files/big.sql").unwrap());
        let mut reader = Reader::new(file);
        loop {
            reader.peek();
            if reader.get() == 0 {
                break;
            }
        }
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);