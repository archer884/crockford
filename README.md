# crockford
Rust Crockford encoding for u64

There is another Crockford Base32 library available for Rust; I just thought it was incredibly annoying to use, because it doesn't allow you to just plug in a `u64` and get back an encoded value. Rather than take a dependency on the existing library, this library is a marginally faithful port of a C# library that performs the same job.

Wanted: check digits. Wanted bad enough to add them myself? Maybe not.
