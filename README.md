Proper Crypto
===
A non-serious crypto library and CLI tool, written in Rust. *This project is for the sole purpose of demonstrating a bunch of concepts in Rust. It comes with no cryptographic guarantees and should not be used in production code*.

### Build Status
| | |
|:---------|:------|
| Linux    |[![Build Status](https://travis-ci.org/gmbeard/proper-crypto.svg?branch=master)](https://travis-ci.org/gmbeard/proper-crypto)|
| Windows  | (No CI, yet)|

### Features
- Everyone's favourite *ROT13*!
- On Windows only, a Rust-friendly interface to `CryptProtectData`

### Examples

```rust
extern crate proper_crypto;
use proper_crypto::Rot13;

fn main() {
    let result = Rot13::new()
        .transform(b"Hello, World!")
        .unwrap();

    assert_eq!(b"Uryyb, Jbeyq!", &*result);
}
```
