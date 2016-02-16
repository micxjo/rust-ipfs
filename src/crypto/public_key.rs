//! Public key cryptography.

use std::{io, fmt, error};
use std::io::{Write, Cursor};

use protobuf;
use protobuf::Message;
use openssl::crypto::pkey::{PKey, Role};

use proto;
use crypto::hash;

/// A public key, possibly with a private key attached.
pub struct PublicKey(PKey);

impl PublicKey {
    /// Generates a public/private key pair of `bit_size` bits.
    pub fn generate(bit_size: usize) -> PublicKey {
        // TODO: Verify that bit_size is reasonable. Does IPFS enforce
        // a min/max key size?
        let mut pkey = PKey::new();
        pkey.gen(bit_size);

        // Can these ever be false? rust-openssl's failure modes are not
        // always clear, but let's assert and fail-fast just in case.
        assert!(pkey.can(Role::Sign));
        assert!(pkey.can(Role::Verify));

        PublicKey(pkey)
    }

    /// Signs `data`, after hashing using `hash_algorithm`. Note:
    /// provide plain text `data`, this function will handle the hashing.
    pub fn sign(&self, hash_alg: hash::Algorithm, data: &[u8]) -> Vec<u8> {
        assert!(self.0.can(Role::Sign));
        let hashed = hash_alg.hash(data);
        // PKey::sign_with_hash doesn't actually hash data for you,
        // it expects to receive a hash and just adds the appropriate
        // ASN.1 information to the signature.
        self.0.sign_with_hash(&hashed[..], hash_alg.to_openssl())
    }

    /// Verifies `signature` of data, hashed using `hash_algorithm`. Note:
    /// provide plain text `data`, this function will handle the hashing.
    pub fn verify(&self,
                  hash_alg: hash::Algorithm,
                  data: &[u8],
                  signature: &[u8])
                  -> bool {
        // Maybe this should return a Result, so that users get warned if they
        // accidentally ignore the result.
        assert!(self.0.can(Role::Verify));
        let hashed = hash_alg.hash(data);
        self.0.verify_with_hash(&hashed[..], signature, hash_alg.to_openssl())
    }

    /// Tries to read a public key `bytes`, which should be encoded as
    /// an IPFS protobuf.
    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, Error> {
        let mut msg = proto::PublicKey::new();
        try!(msg.merge_from_bytes(bytes));
        if !msg.is_initialized() {
            return Err(ProtobufError("message not initialized".to_owned()));
        }
        if msg.get_key_type() != proto::KeyType::RSA {
            // RSA is the only key type supported by IPFS for now.
            return Err(InvalidKeyType);
        }
        let mut pkey = PKey::new();
        // What exactly is the failure model of load_pub? Will it ever panic,
        // or if it fails will pkey.can(Role::Verify) just be false?
        // TODO: Investigate failure model of PKey::load_pub
        pkey.load_pub(msg.get_bytes());
        if !pkey.can(Role::Verify) {
            // Should probably add a separate Error type for this
            return Err(ProtobufError("did not decode a valid public key"
                                         .to_owned()));
        }
        Ok(PublicKey(pkey))
    }

    /// Writes a public key using the IPFS protobuf encoding, possibly
    /// resulting in an `io::Error`. Note: this *only* writes the
    /// public key even if the private key is also present.
    pub fn write_pub<T: Write>(&self, w: &mut T) -> Result<(), Error> {
        let pub_der = self.0.save_pub();
        let mut msg = proto::PublicKey::new();
        msg.set_key_type(proto::KeyType::RSA);
        msg.set_bytes(pub_der);
        if !msg.is_initialized() {
            // This should never occur unless there is an error in
            // rust-protobuf, but let's just be sure.
            return Err(ProtobufError("failed to initialize public key \
                                      protobuf"
                                         .to_owned()));
        }
        try!(msg.write_to_writer(w));
        Ok(())
    }

    /// Returns the bytes of the public key in the IPFS protobuf encoding.
    /// Note: this *only* writes the public key even if the private key
    /// is also present.
    pub fn pub_to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut buf = Cursor::new(Vec::new());
        try!(self.write_pub(&mut buf));
        Ok(buf.into_inner())
    }

    /// Returns true if this key can be used for signing (in other words,
    /// a private key is attached).
    pub fn can_sign(&self) -> bool {
        self.0.can(Role::Sign)
    }

    /// Returns true if the public components are equal.
    pub fn public_eq(&self, other: &PublicKey) -> bool {
        self.0.public_eq(&other.0)
    }
}

/// An error encountered during public key operations.
#[derive(Debug)]
pub enum Error {
    /// An invalid public key type was encountered during decoding.
    InvalidKeyType,
    /// An `io::Error` was encountered while attempting to write this key.
    IoError(io::Error),
    /// A protobuf encoding or decoding error.
    ProtobufError(String),
}

use self::Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidKeyType => {
                write!(f, "an invalid public key type was encountered")
            }
            IoError(ref err) => err.fmt(f),
            ProtobufError(ref s) => {
                write!(f,
                       "protobuf error encountered during public key encoding \
                        or decoding: {}",
                       s)
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            InvalidKeyType => "an invalid public key type was encountered",
            IoError(ref err) => err.description(),
            ProtobufError(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            IoError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Error {
        match err {
            protobuf::ProtobufError::IoError(e) => IoError(e),
            protobuf::ProtobufError::WireError(s) => ProtobufError(s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::hash;

    #[test]
    fn test_generate_sign_verify() {
        let key = PublicKey::generate(1024);
        assert!(key.can_sign());
        let mut sig = key.sign(hash::Algorithm::SHA512, b"hello PublicKey");
        assert!(key.verify(hash::Algorithm::SHA512,
                           b"hello PublicKey",
                           &sig[..]));

        // Try invalidating the signature, just in case
        sig[0] ^= 1;
        assert!(!key.verify(hash::Algorithm::SHA512,
                            b"hello PublicKey",
                            &sig[..]));
    }

    #[test]
    fn test_read_write() {
        let key = PublicKey::generate(1024);
        let encoded = key.pub_to_bytes().unwrap();
        let decoded = PublicKey::from_bytes(&encoded[..]).unwrap();
        assert!(decoded.public_eq(&key));
        assert_eq!(decoded.pub_to_bytes().unwrap(), &encoded[..]);
    }
}
