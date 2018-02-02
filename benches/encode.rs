#![feature(test)]

extern crate crockford;
extern crate test;

use test::Bencher;

#[bench]
fn encode_5111(b: &mut Bencher) {
    b.iter(|| test::black_box(crockford::encode(5111)));
}

#[bench]
fn encode_18446744073709551615(b: &mut Bencher) {
    b.iter(|| test::black_box(crockford::encode(18446744073709551615)));
}

#[bench]
fn encode_into_5111(b: &mut Bencher) {
    let mut buffer = String::with_capacity(13);
    b.iter(|| {
        buffer.clear();
        test::black_box(crockford::encode_into(5111, &mut buffer));
    });
}

#[bench]
fn encode_into_18446744073709551615(b: &mut Bencher) {
    let mut buffer = String::with_capacity(13);
    b.iter(|| {
        buffer.clear();
        test::black_box(crockford::encode_into(18446744073709551615, &mut buffer));
    });
}

