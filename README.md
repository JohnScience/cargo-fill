<!-- cargo-rdme start -->

# `cargo-fill`

[![Crates.io](https://img.shields.io/crates/v/cargo-fill)](https://crates.io/crates/cargo-fill)
[![Downloads](https://img.shields.io/crates/d/cargo-fill.svg)](https://crates.io/crates/cargo-fill)
[![License](https://img.shields.io/crates/l/cargo-fill)](https://crates.io/crates/cargo-fill)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/cargo-fill/status.svg)](https://deps.rs/repo/github/JohnScience/cargo-fill)

An interactive CLI tool for filling the fields of `Cargo.toml` quickly.

## Installation

```console
cargo install cargo-fill
```

## Usage

```console
cargo fill
```

## Features

* Fills all known fields in the `[package]` section of `Cargo.toml`.
* Allows using `cargo-msrv` to fill the `rust-version` field.
* Allows using `git config --get <user.name|user.email|remote.origin.url>` to guess the `authors` and `repository` fields.

## License

Licensed under either of [Apache License, Version 2.0] or [MIT license] at your option.

[Apache License, Version 2.0]: https://www.apache.org/licenses/LICENSE-2.0
[MIT license]: https://opensource.org/licenses/MIT

<!-- cargo-rdme end -->
