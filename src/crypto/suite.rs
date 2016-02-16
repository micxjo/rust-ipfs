//! Cipher suite negotiation.

use proto::Propose;
use crypto::hash;
use crypto::symm::CipherType;
use crypto::ecc::Curve;

/// A comma separated list of the ECDH curves we support.
pub const SUPPORTED_EXCHANGES: &'static str = "P-256,P-384,P-521";

/// A comma separated list of the block ciphers that we support.
pub const SUPPORTED_CIPHERS: &'static str = "AES-256,AES-128";

/// A comma separated list of the hash algorithms we support.
pub const SUPPORTED_HASHES: &'static str = "SHA512,SHA256";

/// A secio cipher suite.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct CipherSuite {
    curve: Curve,
    cipher_type: CipherType,
    hash_alg: hash::Algorithm,
}

impl CipherSuite {
    /// Returns the suite's `Curve`.
    pub fn curve(&self) -> Curve {
        self.curve
    }

    /// Returns the suite's `CipherType`.
    pub fn cipher_type(&self) -> CipherType {
        self.cipher_type
    }

    /// Returns the suite's `hash::Algorithm`.
    pub fn hash_alg(&self) -> hash::Algorithm {
        self.hash_alg
    }

    /// Returns the length of HMACs.
    pub fn hmac_length(&self) -> usize {
        match self.hash_alg {
            hash::Algorithm::SHA256 => 32,
            hash::Algorithm::SHA512 => 64,
        }
    }
}

/// Calculates the `order` parameter for use during the secio handshake.
pub fn calculate_order(first: &Propose, second: &Propose) -> i8 {
    let mut first_buf = first.get_public_key().to_vec();
    first_buf.extend_from_slice(second.get_rand());

    let mut second_buf = second.get_public_key().to_vec();
    second_buf.extend_from_slice(first.get_rand());

    let first_hash = hash::Algorithm::SHA256.hash(&first_buf[..]);
    let second_hash = hash::Algorithm::SHA256.hash(&second_buf[..]);
    if first_hash < second_hash {
        -1
    } else {
        1
    }
}

/// Selects the best matching algorithm given a secio handshake `order`
/// parameter and two comma separated lists.
pub fn select_best(order: i8, first: &str, second: &str) -> Option<String> {
    let (first, second) = if order < 0 {
        (second, first)
    } else {
        (first, second)
    };

    for f in first.split(',') {
        for s in second.split(',') {
            if !f.is_empty() && f == s {
                return Some(f.to_owned());
            }
        }
    }
    None
}

/// Given a secio handshake `order` parameter and an incoming `Propose` message,
/// try to find the matching `CipherSuite` (using the `SUPPORTED_*` consts
/// for locally supported ciphers).
pub fn select_suite(order: i8, propose_in: &Propose) -> Option<CipherSuite> {
    if !propose_in.has_ciphers() || !propose_in.has_exchanges() ||
       !propose_in.has_hashes() {
        return None;
    }

    select_best(order, SUPPORTED_CIPHERS, propose_in.get_ciphers())
        .and_then(|s| CipherType::from_name(&s))
        .and_then(|cipher| {
            select_best(order, SUPPORTED_EXCHANGES, propose_in.get_exchanges())
                .and_then(|s| Curve::from_name(&s))
                .and_then(|curve| {
                    select_best(order,
                                SUPPORTED_HASHES,
                                propose_in.get_hashes())
                        .and_then(|s| hash::Algorithm::from_name(&s))
                        .map(|hash| {
                            CipherSuite {
                                cipher_type: cipher,
                                curve: curve,
                                hash_alg: hash,
                            }
                        })
                })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_best_finds_match() {
        let ciphers = "AES-256,AES-128,Blowfish";
        assert_eq!(select_best(0, &ciphers, &ciphers),
                   Some("AES-256".to_owned()));
        assert_eq!(select_best(-1, &ciphers, &ciphers),
                   Some("AES-256".to_owned()));
        assert_eq!(select_best(1, &ciphers, &ciphers),
                   Some("AES-256".to_owned()));

        assert_eq!(select_best(0, &ciphers, "AES-128"),
                   Some("AES-128".to_owned()));
        assert_eq!(select_best(-1, &ciphers, "AES-128"),
                   Some("AES-128".to_owned()));
        assert_eq!(select_best(1, &ciphers, "AES-128"),
                   Some("AES-128".to_owned()));

        assert_eq!(select_best(0, "AES-128", &ciphers),
                   Some("AES-128".to_owned()));
        assert_eq!(select_best(-1, "AES-128", &ciphers),
                   Some("AES-128".to_owned()));
        assert_eq!(select_best(1, "AES-128", &ciphers),
                   Some("AES-128".to_owned()));

        let ciphers2 = "AES-128,AES-256,Blowfish";
        assert_eq!(select_best(0, ciphers, ciphers2),
                   Some("AES-256".to_owned()));
        assert_eq!(select_best(-1, ciphers, ciphers2),
                   Some("AES-128".to_owned()));
        assert_eq!(select_best(1, ciphers, ciphers2),
                   Some("AES-256".to_owned()));

        assert_eq!(select_best(0, ciphers2, ciphers),
                   Some("AES-128".to_owned()));
        assert_eq!(select_best(-1, ciphers2, ciphers),
                   Some("AES-256".to_owned()));
        assert_eq!(select_best(1, ciphers2, ciphers),
                   Some("AES-128".to_owned()));
    }

    #[test]
    fn test_select_best_fails_match() {
        assert_eq!(select_best(0, "", ""), None);
        assert_eq!(select_best(0, "AES-128", ""), None);
        assert_eq!(select_best(0, "", "AES-128"), None);
        assert_eq!(select_best(0, "AES-128,AES-256", "Blowfish"), None);
        assert_eq!(select_best(0, "Blowfish", "AES-128,AES-256"), None);
    }
}
