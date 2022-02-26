use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn encode_benchmark(c: &mut Criterion) {
    c.bench_function("encode 5111", |b| {
        let mut buffer = String::with_capacity(13);
        b.iter(|| {
            buffer.clear();
            crockford::encode_into(black_box(5111), &mut buffer);
        })
    });

    c.bench_function("encode 184long", |b| {
        let mut buffer = String::with_capacity(13);
        b.iter(|| {
            buffer.clear();
            crockford::encode_into(black_box(18446744073709551615), &mut buffer);
        })
    });
}

criterion_group!(encode, encode_benchmark);
criterion_main!(encode);
