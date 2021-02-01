use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn decode_benchmark(c: &mut Criterion) {
    c.bench_function("decode fzq", |b| {
        b.iter(|| crockford::decode(black_box("fzq")))
    });

    c.bench_function("decode fzq upper", |b| {
        b.iter(|| crockford::decode(black_box("FZQ")))
    });

    c.bench_function("decode fzzlong", |b| {
        b.iter(|| crockford::decode(black_box("fzzzzzzzzzzzz")))
    });

    c.bench_function("decode fzzlong upper", |b| {
        b.iter(|| crockford::decode(black_box("FZZZZZZZZZZZZ")))
    });
}

criterion_group!(decode, decode_benchmark);

criterion_main!(decode);
