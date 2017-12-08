# crockford

> Rust Crockford encoding for u64

[Crockford Base32 Encoding](https://www.crockford.com/wrmg/base32.html) is most commonly used to make numeric identifiers slightly more user-resistant. Similar to [Hashids](http://hashids.org/), the purpose here is to make the identifiers shorter and less confusing. Unlike Hashids, Crockford Base32 does nothing to conceal the real value of the number (beyond the actual encoding, anyway) and the fact that they are sequential is still pretty obvious when you see consecutive identifiers side by side.

This library does not support encoding and decoding of arbitrary data; there is [another library for that](https://crates.io/crates/base32).

The spec supports the idea of check digits, but this library currently does not.

## Usage

### Encoding

Encoding is a one-step process.

```rust
let x = crockford::encode(5111);
assert_eq!("4ZQ", &*x);
```

If you want lowercase, then... Well, tough. The library does not support that yet. If you want to encode to a buffer of your choice rather than a new one created in the function, that's also tough. Pull requests are welcome, but I have not been able to decide the best way to provide these additional features just yet.

### Decoding

Decoding is a two-step process. This is because you can feed any string to the decoder, and the decoder will return an error if you try to convince it that `"Hello, world!"` is a number. (Hint: it isn't.)

```rust
let x = crockford::decode("4zq");
let y = crockford::decode("4ZQ");

assert_eq!(5111, x.unwrap());
assert_eq!(5111, y.unwrap());
```

So, step one is to call the decode function. Step two is to match/verify/unwrap/throw away the output.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE][apc] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[apc]:https://github.com/archer884/crockford/blob/master/LICENSE-APACHE
[mit]:https://github.com/archer884/crockford/blob/master/LICENSE-MIT
