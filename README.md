# tide-serve
## a simple file system server binary built on [tide](https://github.com/http-rs/tide)

* [CI ![CI][ci-badge]][ci]
* [Releases][releases] [![crates.io version][version-badge]][lib-rs]
* [Contributing][contributing]

[ci]: https://github.com/jbr/tide-serve/actions?query=workflow%3ACI
[ci-badge]: https://github.com/jbr/tide-serve/workflows/CI/badge.svg
[releases]: https://github.com/jbr/tide-serve/releases
[contributing]: https://github.com/jbr/tide-serve/blob/master/.github/CONTRIBUTING.md
[lib-rs]: https://lib.rs/tide-serve
[version-badge]: https://img.shields.io/crates/v/tide-serve.svg?style=flat-square

## Installation
```sh
$ cargo install tide-serve
```

## Usage
```sh
$ tide-serve ./static # defaults to . if omitted
```

For now is no way to specify a different listener than
http://localhost:8000, but that's coming soon

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
