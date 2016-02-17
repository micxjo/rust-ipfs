//! Cryptographic hash algorithms and multihash support.

use std::{fmt, error};

use openssl::crypto::hash as ssl_hash;
use openssl::crypto::{hmac, memcmp};
use rust_base58::base58::{FromBase58, ToBase58, FromBase58Error};

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

    /// Computes the multihash of the `data`.
    pub fn multihash(&self, data: &[u8]) -> Multihash {
        let code = self.code();
        let len = self.digest_len() as u8;
        let mut bytes = vec![code, len];
        bytes.extend(self.hash(data).into_iter());
        Multihash(bytes)
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

    /// Calculates the HMAC of the given `key` and `data`.
    pub fn hmac(&self, key: &[u8], data: &[u8]) -> Vec<u8> {
        hmac::hmac(self.to_openssl(), key, data)
    }

    /// Returns true if the HMAC of the key and data is valid.
    pub fn hmac_check(&self, key: &[u8], data: &[u8], hmac_in: &[u8]) -> bool {
        // Maybe this should return a Result so that there's a warning
        // if the result is ignored?
        let hmac_calc = self.hmac(key, data);
        memcmp::eq(hmac_in, &hmac_calc[..])
    }

    /// Returns the digest size in bytes of this algorithm.
    pub fn digest_len(&self) -> usize {
        match *self {
            SHA256 => 32,
            SHA512 => 64,
        }
    }

    /// Tries to decode a multihash code/size header.
    fn from_multihash_header(code: u8, size: u8) -> Result<Algorithm, Error> {
        match (code, size) {
            (0x12, 32) => Ok(SHA256),
            (0x13, 64) => Ok(SHA512),
            _ => Err(UnsupportedAlgorithm),
        }
    }

    /// Returns this algorithm's multihash code byte.
    fn code(&self) -> u8 {
        match *self {
            SHA256 => 0x12,
            SHA512 => 0x13,
        }
    }
}

/// A multihash.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Multihash(Vec<u8>);

impl Multihash {
    /// Tries to decode a binary-encoded multihash.
    pub fn from_bytes(data: &[u8]) -> Result<Multihash, Error> {
        if data.len() < 2 {
            return Err(InvalidLength);
        }

        let algorithm = try!(Algorithm::from_multihash_header(data[0],
                                                              data[1]));

        if data.len() != algorithm.digest_len() + 2 {
            return Err(InvalidLength);
        }
        Ok(Multihash(Vec::from(data)))
    }

    /// Returns the binary-encoded multihash.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }

    /// Tries to parse a multihash from a base58-encoded `str`.
    pub fn from_base58(data: &str) -> Result<Multihash, Error> {
        let bytes = try!(data.from_base58());
        Multihash::from_bytes(&bytes[..])
    }

    /// Returns the base58-encoded (using the Bitcoin/IPFS alphabet) multihash.
    pub fn to_base58(&self) -> String {
        self.0.to_base58()
    }

    /// Returns the algorithm of this multihash.
    pub fn algorithm(&self) -> Algorithm {
        // This should be safe to unwrap, as the only public methods
        // of creating a Multihash all verify the header is valid.
        Algorithm::from_multihash_header(self.0[0], self.0[1]).unwrap()
    }

    /// Returns the digest part of this algorithm (doesn't include the
    /// algorithm code or size, to get the full multhash encoding use
    /// `as_bytes`).
    pub fn digest(&self) -> &[u8] {
        &self.0[2..]
    }
}

/// An error encountered during (multi)hash operations.
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// An unsupported multihash algorithm was encountered.
    UnsupportedAlgorithm,
    /// A multihash of the wrong length was encountered.
    InvalidLength,
    /// An invalid base58 character was encountered.
    InvalidBase58(FromBase58Error),
}

use self::Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnsupportedAlgorithm => {
                write!(f, "unsupported multihash algorithm")
            }
            InvalidLength => write!(f, "invalid multihash length"),
            InvalidBase58(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            UnsupportedAlgorithm => "unsupported multihash algorithm",
            InvalidLength => "invalid multihash length",
            InvalidBase58(_) => "invalid base58 character",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        // Unfortunately FromBase58Error doesn't implement error::Error
        None
    }
}

impl From<FromBase58Error> for Error {
    fn from(err: FromBase58Error) -> Error {
        InvalidBase58(err)
    }
}

#[cfg(test)]
mod tests {
    use super::{Multihash, Algorithm};

    #[test]
    fn test_sha256_multihash() {
        let mhash = Algorithm::SHA256.multihash(b"foo bar baz");

        assert_eq!(mhash.algorithm(), Algorithm::SHA256);
        assert_eq!(mhash.to_base58(),
                   "Qmd8kgzaFLGYtTS1zfF37qKGgYQd5yKcQMyBeSa8UkUz4W");
    }

    #[test]
    fn test_from_base58() {
        let mhash = Multihash::from_base58("Qmd8kgzaFLGYtTS1zfF37qKGgYQd5yKcQ\
                                            MyBeSa8UkUz4W")
                        .unwrap();

        assert_eq!(mhash.algorithm(), Algorithm::SHA256);
        assert_eq!(mhash.to_base58(),
                   "Qmd8kgzaFLGYtTS1zfF37qKGgYQd5yKcQMyBeSa8UkUz4W");
    }
}
