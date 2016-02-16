//! Cryptographic hash algorithms.

use openssl::crypto::hash as ssl_hash;

/// A hashing algorithm.
#[allow(missing_docs)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Algorithm {
    SHA256,
    SHA512,
}

use self::Algorithm::*;

impl Algorithm {
    /// Computes the hash of the provided `data`.
    pub fn hash(&self, data: &[u8]) -> Vec<u8> {
        ssl_hash::hash(self.to_openssl(), data)
    }

    /// Gets the `Algorithm` for a given IPFS hash name.
    pub fn from_name(name: &str) -> Option<Algorithm> {
        match name {
            "SHA256" => Some(SHA256),
            "SHA512" => Some(SHA512),
            _ => None,
        }
    }

    /// Returns the `openssl::crypto::hash::Type` corresponding to this
    /// algorithm.
    pub fn to_openssl(&self) -> ssl_hash::Type {
        match *self {
            SHA256 => ssl_hash::Type::SHA256,
            SHA512 => ssl_hash::Type::SHA512,
        }
    }
}
