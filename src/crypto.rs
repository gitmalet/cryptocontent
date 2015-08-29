use sodiumoxide::init;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::box_;

//pub use sodiumoxide::crypto::secretbox::KEY;

/// Struct containing the needed parameters for crypto.
/// For crypto primitives the Sodium library is used. The cipher suite and MAC functions
/// are the defaults of Sodium for symmetric authenticated encryption.
/// 
/// The struct contains a symmetric key and nonce for encrypting and decrypting the data itself.
/// This key has to be stored to every device using this data.
/// It also contains a public key, secret key and a asymmetric nonce for asymmetric encryption, this is unique per client
/// and is used to exchange the secret key.
///
/// The exchange works like this:
///
/// ![Abstract overview of key exchange between two clients](../../../../doc/key-exchange-abstract.svg)
#[derive(Debug, Serializeable, Deserializeable)]
pub struct CryptoManager {
    pub symkey: secretbox::Key,
    pub symnonce: secretbox::Nonce,
    symfirst: secretbox::Nonce,
    pub pubkey: box_::PublicKey,
    pub seckey: box_::SecretKey,
    pub asymnonce: box_::Nonce,
    asymfirst: box_::Nonce,
}
/*
pub fn gen_key() -> KEY {
    secretbox::gen_key()
}
*/

fn slice_to_array(barry: &[u8]) -> [u8; secretbox::NONCEBYTES] {
    let mut array = [0u8; secretbox::NONCEBYTES];
    for (&x, p) in barry.iter().zip(array.iter_mut()) {
        *p = x;
    }
    array
}

impl CryptoManager {
    /// Generates a new CryptoManager, generating a random keys and nonces.
    /// This should only be done once per client.
    pub fn new() -> CryptoManager {
        init();

        let (p, s) = box_::gen_keypair();
        let sn = secretbox::gen_nonce();
        let an = box_::gen_nonce();

        CryptoManager {
            symkey: secretbox::gen_key(),
            symnonce: sn,
            symfirst: sn,
            pubkey: p,
            seckey: s,
            asymnonce: an,
            asymfirst: an,
        }
    }

    /// This function has to be called to ensure that crypto functions are thread-safe. The
    /// constructor for CryptoManager calls this.
    pub fn init() {
        init();
    }

    /// Generates a new nonce and saves it in the CryptoManager. This has to be done before each
    /// new encryption, because using the same nonce (think as Not more than ONCE) more than once is insecure. 
    /// TODO: Don't generate everytime, just increase
    pub fn new_nonce(&mut self) {
        self.symnonce = secretbox::gen_nonce();
    }

    /// Encrypts the str with key and new nonce
    pub fn encrypt(&mut self, plaintext: &str) -> Option<Vec<u8>> {
        //TODO: Check for errors

        self.new_nonce();
        let mut ct = secretbox::seal(plaintext.as_bytes(), &self.symnonce, &self.symkey);
        let secretbox::Nonce(nb) = self.symnonce.clone();
        let mut out = nb.to_vec();
        out.append(&mut ct);
        Some(out)
    }

    /// Decrypts the ciphertext with key and nonce. Nonce and Key has to be the same for encryption and
    /// decryption
    pub fn decrypt(&self, ciphertext: Vec<u8>) -> Option<String> {
        let (nb, ciphertext) = ciphertext.split_at(secretbox::NONCEBYTES);
        let nonce = slice_to_array(nb);
        let plain = match secretbox::open(ciphertext, &secretbox::Nonce(nonce), &self.symkey) {
            Ok(o) => o,
            Err(_) => return None,
        };
        String::from_utf8(plain).ok()
    }
}
