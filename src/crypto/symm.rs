//! Symmetric encryption.

use std::{fmt, error};

use openssl::crypto::symm as ssl_symm;

// TODO: Investigate blowfish support.
/// A symmetric cipher type.
#[allow(missing_docs, non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum CipherType {
    AES_128_CTR,
    AES_256_CTR,
}

use self::CipherType::*;

impl CipherType {
    /// Returns this cipher type's key length in bytes.
    pub fn key_len(&self) -> usize {
        match *self {
            AES_128_CTR => 16,
            AES_256_CTR => 32,
        }
    }

    /// Returns the `openssl::crypto::symm::Type` corresponding to this type.
    fn to_openssl(&self) -> ssl_symm::Type {
        match *self {
            AES_128_CTR => ssl_symm::Type::AES_128_CTR,
            AES_256_CTR => ssl_symm::Type::AES_256_CTR,
        }
    }
}

/// An encryption mode.
#[allow(missing_docs)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

use self::Mode::*;

impl Mode {
    /// Returns the `openssl::crypto::symm::Mode` corresponding to this mode.
    fn to_openssl(&self) -> ssl_symm::Mode {
        match *self {
            Encrypt => ssl_symm::Mode::Encrypt,
            Decrypt => ssl_symm::Mode::Decrypt,
        }
    }
}

/// A symmetric cipher context.
pub struct Cipher {
    mode: Mode,
    crypter: ssl_symm::Crypter,
}

impl Cipher {
    /// Returns a new cipher for the given `mode`, `key` and `iv`
    /// (initialization vector). Returns `Error::InvalidKeyLength` if
    /// `key.len()` is not correct.
    pub fn new(ct: CipherType,
               mode: Mode,
               key: &[u8],
               iv: &[u8])
               -> Result<Cipher, Error> {
        if key.len() != ct.key_len() {
            return Err(InvalidKeyLength);
        }
        let crypter = ssl_symm::Crypter::new(ct.to_openssl());
        crypter.pad(true);
        crypter.init(mode.to_openssl(), &key[..], &iv[..]);
        Ok(Cipher {
            mode: mode,
            crypter: crypter,
        })
    }

    /// Updates the cipher, returning the encrypted/decrypted bytes.
    pub fn update(&mut self, data: &[u8]) -> Vec<u8> {
        // ssl_symm::Crypter::update actually doesn't take a mutable reference,
        // but that's sketchy, so we require mut anyway.
        self.crypter.update(data)
    }

    /// Returns this cipher's encryption mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }
}

/// An error encountered during symmetric crypto operations.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Error {
    /// An invalid key length was supplied.
    InvalidKeyLength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "key of invalid length provided")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "key of invalid length provided"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

use self::Error::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let mut enc = Cipher::new(CipherType::AES_256_CTR,
                                  Mode::Encrypt,
                                  b"secret key1111111111111111111111",
                                  b"iv")
                          .unwrap();
        let mut dec = Cipher::new(CipherType::AES_256_CTR,
                                  Mode::Decrypt,
                                  b"secret key1111111111111111111111",
                                  b"iv")
                          .unwrap();
        assert_eq!(enc.mode(), Mode::Encrypt);
        assert_eq!(dec.mode(), Mode::Decrypt);
        let buf = enc.update(b"a secret message here");
        assert_eq!(&dec.update(&buf[..])[..], b"a secret message here");
    }

    #[test]
    fn test_invalid_key_length() {
        assert!(Cipher::new(CipherType::AES_256_CTR,
                            Mode::Encrypt,
                            b"too short",
                            b"iv")
                    .is_err());
    }
}
