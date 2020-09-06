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
$ cargo install --git https://github.com/jbr/tide-serve --branch main
```
Soon this will just be `cargo install tide-serve`, but we are currently waiting on a `surf` release

## Usage

```sh
$ tide-serve --help
tide-serve 0.0.3-alpha.0
a simple static http server built with tide

USAGE:
    tide-serve [OPTIONS] [root]

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -o, --host <host>
            Local host or ip to listen on [env: HOST=]  [default: localhost]

    -p, --port <port>
            Local port to listen on [env: PORT=]  [default: 8080]

    -b, --bind <bind>
            Local listener spec to bind

            Examples: `--bind localhost:8080` `--bind http://localhost:8080` `--bind [::1]:1213`

            On unix-like systems only: `--bind http+unix:///var/run/some.socket` `--bind http+unix://./tmp/socket`

            --bind will override --host and --port. [env: TIDE_BIND=]
    -c, --cert-path <cert-path>
            Path to a tls certificate for tide_rustls

            This will be ignored unless key_path is also provided. providing both key_path and cert_path enables tls.

            Example: `--cert ./cert.pem --key ./key.pem` For development, try using mkcert [env: CERT_PATH=]
    -k, --key-path <key-path>
            The path to a tls key file for tide_rustls

            This will be ignored unless cert_path is also provided. providing both key_path and cert_path enables tls.

            Example: `--cert ./cert.pem --key ./key.pem` For development, try using mkcert [env: KEY_PATH=]
    -f, --forward <forward>
            Host to forward (reverse proxy) not-found requests to

            This forwards any request that would otherwise be a 404 Not Found to the specified listener spec.

            Examples: `--forward localhost:8081` `--forward http://localhost:8081` `--forward https://localhost:8081`

            Note: http+unix:// schemes are not yet supported [env: FORWARD=]

ARGS:
    <root>
            Filesystem path to serve

            Defaults to the current working directory
```

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
