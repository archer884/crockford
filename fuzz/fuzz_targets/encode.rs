#![no_main]

#[macro_use] extern crate libfuzzer_sys;

extern crate crockford;

fuzz_target!(|data: &[u8]| {
    for chunk in data.chunks(8) {
        let n = chunk.iter().fold(0u64, |a, &b| {
            (a << 8) | u64::from(b)
        });

        let _ = crockford::encode(n);
    }
});
