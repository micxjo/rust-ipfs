#![warn(missing_docs)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
//! An experimental implementation of various
//! [IPFS](https://github.com/ipfs/ipfs)-related components.

extern crate openssl;

pub mod crypto;
