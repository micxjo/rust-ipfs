#![allow(non_camel_case_types)]
//! Elliptic curve DH.

extern crate libc;
use self::libc::{c_int, size_t};

use std::ptr;

enum EC_KEY { }
enum EC_POINT { }
enum EC_GROUP { }

/// An ephemeral key pair.
pub struct EphemeralKeyPair {
    key: *mut EC_KEY,
    curve: Curve,
}

/// An elliptic curve.
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Curve {
    P256,
    P384,
    P521,
}

impl Curve {
    /// Maps a curve name from IPFS to a `Curve`.
    pub fn from_name(name: &str) -> Option<Curve> {
        match name {
            "P-256" => Some(Curve::P256),
            "P-384" => Some(Curve::P384),
            "P-521" => Some(Curve::P521),
            _ => None,
        }
    }

    fn to_nid(self) -> c_int {
        match self {
            Curve::P256 => 415,
            Curve::P384 => 715,
            Curve::P521 => 716,
        }
    }

    fn field_len(self) -> usize {
        match self {
            Curve::P256 => 256,
            Curve::P384 => 384,
            Curve::P521 => 521,
        }
    }

    fn encoded_public_key_len(self) -> usize {
        1 + (2 * ((self.field_len() + 7) / 8))
    }

    fn secret_len(self) -> usize {
        (self.field_len() + 7) / 8
    }
}

impl EphemeralKeyPair {
    /// Generate an ephemeral key pair on the given `Curve`.
    pub fn generate(curve: Curve) -> Result<EphemeralKeyPair, ()> {
        let key = unsafe { EC_KEY_new_by_curve_name(curve.to_nid()) };

        if key == ptr::null_mut() {
            return Err(());
        }

        match unsafe { EC_KEY_generate_key(key) } {
            1 => {
                Ok(EphemeralKeyPair {
                    key: key,
                    curve: curve,
                })
            }
            _ => Err(()),
        }
    }

    fn public_point_len(&self) -> usize {
        self.curve.encoded_public_key_len()
    }

    fn fill_with_encoded_public_key(&self, out: &mut [u8]) -> Result<(), ()> {
        unsafe {
            let point = EC_KEY_get0_public_key(self.key);
            let group = EC_KEY_get0_group(self.key);
            EC_POINT_point2oct(group,
                               point,
                               4,
                               out.as_mut_ptr(),
                               out.len(),
                               ptr::null_mut());
        }
        Ok(())

    }

    /// Returns the encoded public point.
    pub fn encoded_public_key(&self) -> Result<Vec<u8>, ()> {
        let mut ret = Vec::new();
        ret.resize(self.public_point_len(), 0);
        try!(self.fill_with_encoded_public_key(&mut ret[..]));
        Ok(ret)
    }

    fn oct_to_point(&self, point_buf: &[u8]) -> Result<*mut EC_POINT, ()> {
        let group = unsafe { EC_KEY_get0_group(self.key) };

        let point = unsafe { EC_POINT_new(group) };

        if point == ptr::null_mut() {
            return Err(());
        }

        match unsafe {
            EC_POINT_oct2point(group,
                               point,
                               point_buf.as_ptr(),
                               point_buf.len(),
                               ptr::null_mut())
        } {
            1 => Ok(point),
            _ => Err(()),
        }
    }

    /// Computes the shared secret, given the bytes of the other side's
    /// public point.
    pub fn compute_secret(self, peer_point: &[u8]) -> Result<Vec<u8>, ()> {
        let peer_point = try!(self.oct_to_point(peer_point));
        let mut secret = Vec::new();
        let secret_size = self.curve.secret_len();
        secret.resize(secret_size, 0);
        match unsafe {
            let len = ECDH_compute_key(secret.as_mut_ptr(),
                                       secret.len(),
                                       peer_point,
                                       self.key,
                                       ptr::null());
            EC_POINT_free(peer_point);
            len
        } {
            n if n == (secret_size as i32) => Ok(secret),
            _ => Err(()),
        }
    }
}

impl Drop for EphemeralKeyPair {
    fn drop(&mut self) {
        unsafe {
            EC_KEY_free(self.key);
        }
    }
}

#[link(name="ssl")]
extern "C" {
    fn EC_KEY_new_by_curve_name(nid: c_int) -> *mut EC_KEY;
    fn EC_KEY_generate_key(key: *mut EC_KEY) -> c_int;
    fn EC_KEY_get0_public_key(key: *const EC_KEY) -> *const EC_POINT;
    fn EC_KEY_get0_group(key: *const EC_KEY) -> *const EC_GROUP;
    fn EC_KEY_free(key: *mut EC_KEY);

    fn ECDH_compute_key(out: *mut u8,
                        out_len: size_t,
                        pub_key: *const EC_POINT,
                        ec_key: *mut EC_KEY,
                        kdf: *const u8)
                        -> c_int;

    fn EC_POINT_new(group: *const EC_GROUP) -> *mut EC_POINT;
    fn EC_POINT_free(point: *mut EC_POINT);
    fn EC_POINT_point2oct(group: *const EC_GROUP,
                          point: *const EC_POINT,
                          form: c_int,
                          buf: *mut u8,
                          len: size_t,
                          bin_ct: *mut u8)
                          -> size_t;
    fn EC_POINT_oct2point(group: *const EC_GROUP,
                          point: *const EC_POINT,
                          buf: *const u8,
                          len: size_t,
                          bin_ct: *mut u8)
                          -> c_int;
}
