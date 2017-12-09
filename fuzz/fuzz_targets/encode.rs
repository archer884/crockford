#![no_main]

#[macro_use] extern crate libfuzzer_sys;

extern crate crockford;

fuzz_target!(|data| {
    use std::slice;

    let len = data.len() / 8;
    let ptr = data.as_ptr() as *const u64;
    let input = unsafe { slice::from_raw_parts(ptr, len) };

    for &n in input {
        let _ = crockford::encode(n);
    }
});
