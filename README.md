# rs-ipfs

[![MIT License](https://img.shields.io/github/license/micxjo/rs-ipfs.svg)](https://github.com/micxjo/rs-ipfs/blob/master/LICENSE)

This is an experimental implementation of various [IPFS](https://github.com/ipfs/ipfs)-related components in Rust.

## Contributing

Contributions are welcome!

* Please format your code with `rustfmt` before submitting a pull request. There is a `rustfmt.toml` file in the top of the repository which `rustfmt` or `cargo fmt` should pick up automatically; currently, only the max line length setting has been modified as I'm old fashioned and prefer an 80-character max.
* Compatibility with stable (1.6 as of now) and beta Rust is aimed for, so avoid features only available in the nightlies.
* Heed compiler warnings and lints. You can get an additional set of lints (from the [rust-clippy](https://github.com/Manishearth/rust-clippy) project) by building with `cargo build --features clippy`, though you'll need to be on a nightly Rust to do so (I recommend checking out [multirust](https://github.com/brson/multirust) to manage multiple Rust compiler installations if you aren't using it already). It's OK to occasionally add an `#[allow(a_specific_lint)]` attribute above certain definitions if a warning is obviously spurious, but try to avoid overdoing it.
* Documentation help is always appreciated.

## Security

This is an experimental implementation of an experimental protocol. Please do not trust any sensitive data to `rs-ipfs` at this time.