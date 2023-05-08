# crockford

> Base32 encoding for 64-bit values.

[Crockford Base32 Encoding](https://www.crockford.com/wrmg/base32.html) is most commonly used to make numeric identifiers slightly more user-resistant. Similar to [Hashids](http://hashids.org/), the purpose here is to make the identifiers shorter and less confusing. Unlike Hashids, Crockford Base32 does nothing to conceal the real value of the number (beyond the actual encoding, anyway) and the fact that they are sequential is still pretty obvious when you see consecutive identifiers side by side.

This library does not support encoding and decoding of arbitrary data; there is [another library for that](https://crates.io/crates/base32). Additionally, the spec supports the idea of check digits, but this library currently does not.

**The primary purpose of this library is to provide high performance, user-resistant encoding of numeric identifiers.** To that end, both encoding and decoding are, in fact, pretty darn fast. How fast? According to my testing, `crockford` decodes **fifty times faster** and encodes **twenty-seven times faster** than `harsh`. 

## Usage

### Encoding

Encoding is a one-step process.

```rust
let x = crockford::encode(5111);
assert_eq!("4ZQ", &*x);
```

If you want lowercase, then... Well, tough. However, we do now support encoding to a buffer of your choice rather than a new one created in the function. Read on to learn about plan B...

#### Plan B (faster encoding)

Because this is Rust, particular focus is given to runtime efficiency--or, at least, allowing the user to achieve runtime efficiency. As a result, we provide a second, more complicated encoding option.

```rust
// The longest possible representation of u64 is 13 digits.
let mut buf = Vec::with_capacity(13);
crockford::encode_into(5111, &mut buf);

let result = std::str::from_utf8(&buf)?;
assert_eq!("4ZQ", result);
```

This `encode_into` method also accepts `&mut String`, if you prefer.

### Decoding

Decoding is a two-step process. This is because you can feed any string to the decoder, and the decoder will return an error if you try to convince it that `"Hello, world!"` is a number. (Hint: it isn't.)

```rust
let x = crockford::decode("4zq");
let y = crockford::decode("4ZQ");

assert_eq!(5111, x?);
assert_eq!(5111, y?);
```

So, step one is to call the decode function. Step two is to match/verify/unwrap/throw away the output.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE][apc] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[travis-image]: https://travis-ci.org/archer884/crockford.svg?branch=master
[travis-url]: https://travis-ci.org/archer884/crockford

[apc]: https://github.com/archer884/crockford/blob/master/LICENSE-APACHE
[mit]: https://github.com/archer884/crockford/blob/master/LICENSE-MIT
