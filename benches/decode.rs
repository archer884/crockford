#![feature(test)]

extern crate crockford;
extern crate test;

use test::Bencher;

#[bench]
fn fzq(b: &mut Bencher) {
    b.iter(|| test::black_box(crockford::decode("fzq")));
}

#[bench]
fn fzq_uppercase(b: &mut Bencher) {
    b.iter(|| test::black_box(crockford::decode("FZQ")));
}

#[bench]
fn fzzzzzzzzzzzz(b: &mut Bencher) {
    b.iter(|| test::black_box(crockford::decode("fzzzzzzzzzzzz")));
}

#[bench]
fn fzzzzzzzzzzzz_uppercase(b: &mut Bencher) {
    b.iter(|| test::black_box(crockford::decode("FZZZZZZZZZZZZ")));
}
