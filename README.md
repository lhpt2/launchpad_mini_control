# LaunchpadMini_Control


## Overview

## Example

```toml
[dependencies]
tokio = { version = "1.32.0", features = ["full"] }
```
Then, on your main.rs:

```rust,no_run

fn main() {
}
```

## Contributing

:balloon: Thanks for your help improving the project! 

## Related Projects

In addition to the crates in this repository, the Tokio project also maintains
several other libraries, including:

* [`hyper`]: A fast and correct HTTP/1.1 and HTTP/2 implementation for Rust.

* [`tonic`]: A gRPC over HTTP/2 implementation focused on high performance, interoperability, and flexibility.

* [`warp`]: A super-easy, composable, web server framework for warp speeds.

* [`tower`]: A library of modular and reusable components for building robust networking clients and servers.

* [`tracing`] (formerly `tokio-trace`): A framework for application-level tracing and async-aware diagnostics.

* [`rdbc`]: A Rust database connectivity library for MySQL, Postgres and SQLite.

* [`mio`]: A low-level, cross-platform abstraction over OS I/O APIs that powers
  `tokio`.

* [`bytes`]: Utilities for working with bytes, including efficient byte buffers.

* [`loom`]: A testing tool for concurrent Rust code

[`warp`]: https://github.com/seanmonstar/warp
[`hyper`]: https://github.com/hyperium/hyper
[`tonic`]: https://github.com/hyperium/tonic
[`tower`]: https://github.com/tower-rs/tower
[`loom`]: https://github.com/tokio-rs/loom
[`rdbc`]: https://github.com/tokio-rs/rdbc
[`tracing`]: https://github.com/tokio-rs/tracing
[`mio`]: https://github.com/tokio-rs/mio
[`bytes`]: https://github.com/tokio-rs/bytes

## Changelog

The Tokio repository contains multiple crates. Each crate has its own changelog.

 * `tokio` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio/CHANGELOG.md)
 * `tokio-util` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-util/CHANGELOG.md)
 * `tokio-stream` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-stream/CHANGELOG.md)
 * `tokio-macros` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-macros/CHANGELOG.md)
 * `tokio-test` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-test/CHANGELOG.md)

## Supported Rust Versions

<!--
When updating this, also update:
- .github/workflows/ci.yml
- CONTRIBUTING.md
- README.md
- tokio/README.md
- tokio/Cargo.toml
- tokio-util/Cargo.toml
- tokio-test/Cargo.toml
- tokio-stream/Cargo.toml
-->

Tokio will keep a rolling MSRV (minimum supported rust version) policy of **at
least** 6 months. When increasing the MSRV, the new Rust version must have been
released at least six months ago. The current MSRV is 1.63.

Note that the MSRV is not increased automatically, and only as part of a minor
release. The MSRV history for past minor releases can be found below:

 * 1.30 to now - Rust 1.63
 * 1.27 to 1.29 - Rust 1.56
 * 1.17 to 1.26 - Rust 1.49
 * 1.15 to 1.16 - Rust 1.46
 * 1.0 to 1.14 - Rust 1.45

Note that although we try to avoid the situation where a dependency transitively
increases the MSRV of Tokio, we do not guarantee that this does not happen.
However, every minor release will have some set of versions of dependencies that
works with the MSRV of that minor release.

## Release schedule

Tokio doesn't follow a fixed release schedule, but we typically make one to two
new minor releases each month. We make patch releases for bugfixes as necessary.

## Bug patching policy

For the purposes of making patch releases with bugfixes, we have designated
certain minor releases as LTS (long term support) releases. Whenever a bug
warrants a patch release with a fix for the bug, it will be backported and
released as a new patch release for each LTS minor version. Our current LTS
releases are:

 * `1.20.x` - LTS release until September 2023. (MSRV 1.49)
 * `1.25.x` - LTS release until March 2024. (MSRV 1.49)
 * `1.32.x` - LTS release until September 2024 (MSRV 1.63)

Each LTS release will continue to receive backported fixes for at least a year.
If you wish to use a fixed minor release in your project, we recommend that you
use an LTS release.

To use a fixed minor version, you can specify the version with a tilde. For
example, to specify that you wish to use the newest `1.25.x` patch release, you
can use the following dependency specification:
```text
tokio = { version = "~1.25", features = [...] }
```

### Previous LTS releases

 * `1.8.x` - LTS release until February 2022.
 * `1.14.x` - LTS release until June 2022.
 * `1.18.x` - LTS release until June 2023.

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/tokio-rs/tokio/blob/master/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tokio by you, shall be licensed as MIT, without any additional
terms or conditions.
