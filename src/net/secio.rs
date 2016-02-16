//! A secure network stream.

use std::{io, u32};
use std::cmp::min;
use std::io::{Read, Write, Error, ErrorKind};
use std::collections::VecDeque;
use std::net::TcpStream;

use rand;
use rand::Rng;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use openssl::crypto::memcmp;
use protobuf::Message;

use proto::{Propose, Exchange};
use crypto::{hash, suite, public_key, symm};
use crypto::public_key::PublicKey;
use crypto::ecc::EphemeralKeyPair;
use net::multiaddr::{Multiaddr, Addr};

/// A secure stream.
pub struct SecureStream<T> {
    stream: T,
    cipher_suite: suite::CipherSuite,
    local_params: EncryptionParams,
    remote_params: EncryptionParams,
    read_buf: VecDeque<u8>,
}

impl SecureStream<TcpStream> {
    /// Attempts to connect to a given `Multiaddr` via secio. Currently
    /// only supports IPv4/TCP multiaddrs.
    pub fn dial(addr: &Multiaddr,
                pub_key: PublicKey)
                -> io::Result<SecureStream<TcpStream>> {
        let parts = addr.parts();

        if let Some(&Addr::Ipv4(ip4)) = parts.get(0) {
            if let Some(&Addr::Tcp(port)) = parts.get(1) {
                let tcp_stream = try!(TcpStream::connect((ip4, port)));
                return SecureStream::new(tcp_stream, pub_key);
            }
        }

        Err(Error::new(ErrorKind::Other, "only support IPv4/TCP for now"))
    }
}

impl<T: Read + Write> SecureStream<T> {
    /// Wraps a Reader/Writer and performs a secio handshake, using
    /// `pub_key` as the local identity.
    pub fn new(stream: T, pub_key: PublicKey) -> io::Result<SecureStream<T>> {
        let mut stream = stream;

        // Send Propose.
        let propose_out = try!(generate_propose(&pub_key));
        // Unfortunately we can't define `impl From<ProtobufError> for Error`
        // due to orphan rules, so there's some manual conversions required
        // throughout this function
        let propose_out_buf = match propose_out.write_to_bytes() {
            Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
            Ok(buf) => buf,
        };
        try!(write_length_prefixed(&mut stream, &propose_out_buf[..]));

        // Receive Propose.
        let propose_in_buf = try!(read_length_prefixed(&mut stream));
        let mut propose_in = Propose::new();
        if let Err(err) = propose_in.merge_from_bytes(&propose_in_buf[..]) {
            return Err(Error::new(ErrorKind::InvalidData, err));
        }

        // Select cipher suite.
        let remote_pub_key =
            try!(PublicKey::from_bytes(propose_in.get_public_key()));
        let order = suite::calculate_order(&propose_in, &propose_out);
        let suite = match suite::select_suite(order, &propose_in) {
            None => {
                return Err(Error::new(ErrorKind::Other,
                                      "couldn't agree to cipher suite"))
            }
            Some(s) => s,
        };

        // Generate ephemeral key pair
        let eph_key_pair = match EphemeralKeyPair::generate(suite.curve()) {
            Err(_) => {
                return Err(Error::new(ErrorKind::Other,
                                      "couldn't generate ephemeral key pair"))
            }
            Ok(ekp) => ekp,
        };
        let public_point_buf = match eph_key_pair.encoded_public_key() {
            Err(_) => {
                return Err(Error::new(ErrorKind::Other,
                                      "couldn't encode public point"))
            }
            Ok(pp) => pp,
        };

        // Compose selection
        let mut selection_out = Vec::new();
        selection_out.extend_from_slice(&propose_out_buf[..]);
        selection_out.extend_from_slice(&propose_in_buf[..]);
        selection_out.extend_from_slice(&public_point_buf[..]);

        // Sign selection and send Exchange.
        let signature = pub_key.sign(hash::Algorithm::SHA256,
                                     &selection_out[..]);
        let mut exchange_out = Exchange::new();
        exchange_out.set_ephemeral_public_key(public_point_buf.clone());
        exchange_out.set_signature(signature);
        assert!(exchange_out.is_initialized());
        let exchange_out_buf = match exchange_out.write_to_bytes() {
            Err(err) => return Err(Error::new(ErrorKind::Other, err)),
            Ok(buf) => buf,
        };
        try!(write_length_prefixed(&mut stream, &exchange_out_buf[..]));

        // Receive Exchange.
        let exchange_in_buf = try!(read_length_prefixed(&mut stream));
        let mut exchange_in = Exchange::new();
        if let Err(err) = exchange_in.merge_from_bytes(&exchange_in_buf[..]) {
            return Err(Error::new(ErrorKind::InvalidData, err));
        }
        assert!(exchange_in.is_initialized());
        let remote_eph_pub_key = exchange_in.get_ephemeral_public_key();

        // Check signature
        let mut selection_in = propose_in_buf;
        selection_in.extend_from_slice(&propose_out_buf[..]);
        selection_in.extend_from_slice(&remote_eph_pub_key[..]);
        if !remote_pub_key.verify(hash::Algorithm::SHA256,
                                  &selection_in[..],
                                  exchange_in.get_signature()) {
            return Err(Error::new(ErrorKind::InvalidData, "invalid signature"));
        }

        // Compute stretched keys.
        let shared_secret =
            match eph_key_pair.compute_secret(remote_eph_pub_key) {
                Err(_) => {
                    return Err(Error::new(ErrorKind::Other,
                                          "couldn't calculate shared secret"))
                }
                Ok(secret) => secret,
            };
        let (k1, k2) = try!(symm::stretch_key(suite.cipher_type(),
                                              suite.hash_alg(),
                                              &shared_secret[..]));
        let (k1, k2) = if order >= 0 {
            (k1, k2)
        } else {
            (k2, k1)
        };

        // Make ciphers
        let local_cipher = try!(symm::Cipher::new(suite.cipher_type(),
                                                  symm::Mode::Encrypt,
                                                  k1.cipher_key(),
                                                  k1.iv()));
        let remote_cipher = try!(symm::Cipher::new(suite.cipher_type(),
                                                   symm::Mode::Decrypt,
                                                   k2.cipher_key(),
                                                   k2.iv()));

        let local_params = EncryptionParams {
            permanent_key: pub_key,
            ephemeral_key: public_point_buf,
            stretched_key: k1,
            cipher: local_cipher,
        };
        let remote_params = EncryptionParams {
            permanent_key: remote_pub_key,
            ephemeral_key: remote_eph_pub_key.to_vec(),
            stretched_key: k2,
            cipher: remote_cipher,
        };

        let mut secure_stream = SecureStream {
            stream: stream,
            cipher_suite: suite,
            local_params: local_params,
            remote_params: remote_params,
            read_buf: VecDeque::new(),
        };

        try!(secure_stream.write_all(&propose_in.get_rand()));

        let rand_in = try!(secure_stream.read_message());

        if !memcmp::eq(&rand_in[..], propose_out.get_rand()) {
            return Err(Error::new(ErrorKind::InvalidData,
                                  "received bad rand value"));
        }

        println!("Secure handshake complete.");
        Ok(secure_stream)
    }


    /// Returns this stream's negotiated `CipherSuite`.
    pub fn cipher_suite(&self) -> suite::CipherSuite {
        self.cipher_suite
    }
}

impl<T: Read> SecureStream<T> {
    fn read_message(&mut self) -> Result<Vec<u8>, Error> {
        let msg = try!(read_length_prefixed(&mut self.stream));
        let hmac_length = self.cipher_suite.hmac_length();
        if msg.len() < hmac_length {
            return Err(Error::new(ErrorKind::InvalidData,
                                  "received message too short to contain an \
                                   HMAC"));
        }

        // TODO: Move this into a function on hash::Algorithm or CipherSuite
        let data = &msg[..msg.len() - hmac_length];
        let hmac = &msg[msg.len() - hmac_length..];

        if !self.cipher_suite
                .hash_alg()
                .hmac_check(self.remote_params.stretched_key.hmac_key(),
                            data,
                            hmac) {
            println!("Invalid HMAC");
            return Err(Error::new(ErrorKind::InvalidData, "invalid HMAC"));
        }
        Ok(self.remote_params.cipher.update(&data[..]))
    }
}

impl<T: Write> Write for SecureStream<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        try!(self.write_all(buf));
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.stream.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut enc = self.local_params.cipher.update(buf);
        let hmac = self.cipher_suite
                       .hash_alg()
                       .hmac(self.local_params.stretched_key.hmac_key(),
                             &enc[..]);
        enc.extend(hmac);
        write_length_prefixed(&mut self.stream, &enc[..])
    }
}

impl<T: Read> Read for SecureStream<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let mut count = 0;
        while count < min(buf.len(), self.read_buf.len()) {
            buf[count] = self.read_buf.pop_front().unwrap();
            count += 1;
        }

        if count < buf.len() {
            let packet = try!(self.read_message());
            let diff = min(buf.len() - count, packet.len());
            for byte in packet.iter().take(diff) {
                buf[count] = *byte;
                count += 1;
            }
            self.read_buf.extend(packet[diff..].into_iter());
        }

        Ok(count)
    }
}

#[allow(dead_code)]
struct EncryptionParams {
    permanent_key: PublicKey,
    ephemeral_key: Vec<u8>,
    stretched_key: symm::StretchedKey,
    cipher: symm::Cipher,
}


/// Reads a length-prefixed (big-endian u32) message from a stream.
fn read_length_prefixed<T: Read>(r: &mut T) -> io::Result<Vec<u8>> {
    let size = try!(r.read_u32::<BigEndian>());

    // TODO: Implement a max message size
    if size == 0 {
        return Err(Error::new(ErrorKind::InvalidData, "empty message"));
    }

    let mut buf = vec![0u8; size as usize];
    try!(r.read_exact(&mut buf[..]));
    Ok(buf)
}

/// Writes a length-prefixed (big-endian u32) message to a stream. Flushes
/// the stream.
fn write_length_prefixed<T: Write>(w: &mut T, data: &[u8]) -> io::Result<()> {
    if data.len() > (u32::MAX as usize) {
        return Err(Error::new(ErrorKind::InvalidData, "message too large"));
    }

    try!(w.write_u32::<BigEndian>(data.len() as u32));
    try!(w.write_all(&data[..]));
    try!(w.flush());
    Ok(())
}

/// Generates an outgoing Propose message.
fn generate_propose(pub_key: &PublicKey) -> io::Result<Propose> {
    let mut rng = rand::thread_rng();
    let mut nonce = vec![0u8; 16];
    rng.fill_bytes(&mut nonce[..]);

    let pub_key = try!(pub_key.pub_to_bytes());

    let mut propose = Propose::new();
    propose.set_public_key(pub_key);
    propose.set_ciphers(suite::SUPPORTED_CIPHERS.to_owned());
    propose.set_exchanges(suite::SUPPORTED_EXCHANGES.to_owned());
    propose.set_hashes(suite::SUPPORTED_HASHES.to_owned());
    propose.set_rand(nonce);
    assert!(propose.is_initialized());
    Ok(propose)
}


impl From<public_key::Error> for Error {
    fn from(err: public_key::Error) -> Error {
        match err {
            public_key::Error::IoError(e) => e,
            _ => Error::new(ErrorKind::InvalidData, err),
        }
    }
}

impl From<symm::Error> for Error {
    fn from(err: symm::Error) -> Error {
        Error::new(ErrorKind::Other, err)
    }
}
