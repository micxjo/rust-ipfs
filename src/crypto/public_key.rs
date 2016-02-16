//! Public key cryptography.

use openssl::crypto::pkey::{PKey, Role};

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

    /// Returns true if this key can be used for signing (in other words,
    /// a private key is attached).
    pub fn can_sign(&self) -> bool {
        self.0.can(Role::Sign)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::hash;

    #[test]
    fn generate_sign_verify() {
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
}
