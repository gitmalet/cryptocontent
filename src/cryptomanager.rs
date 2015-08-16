use sodiumoxide::init;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::box_;

/// Struct containing the needed extra parameters for encrypting and decrypting operations.
/// For encryption and decryption the Sodium library is used. The cipher suite and MAC functions
/// are the defaults of Sodium for symmetric authenticated encryption.
/// 
/// This functions need a key that has to be secret and a nonce that has to be just unique but not
/// secret.
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct CryptoManager {
    pub symkey: secretbox::Key,
    pub symnonce: secretbox::Nonce,
    symfirst: secretbox::Nonce,
    pub pubkey: box_::PublicKey,
    pub seckey: box_::SecretKey,
    pub asymnonce: box_::Nonce,
    asymfirst: box_::Nonce,
}

impl CryptoManager {
 
    /// Generates a new CryptoManager, generating a random key and a random nonce
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

    /// Generates a new nonce and saves it in the CryptoManager. This has to be done before each
    /// new encryption, because using the same nonce (think as not more than once) more than once is insecure. 
    pub fn new_nonce(&mut self) {
        self.symnonce = secretbox::gen_nonce();
    }

    /// Encrypts the str with key and current nonce
    pub fn encrypt(&self, plaintext: &str) -> Option<Vec<u8>> {
        //TODO: Check for errors
        Some(secretbox::seal(plaintext.as_bytes(), &self.symnonce, &self.symkey))
    }

    /// Decrypts the ciphertext with key and nonce. Nonce and Key has to be the same for encryption and
    /// decryption
    pub fn decrypt(&self, ciphertext: &[u8]) -> Option<String> {
        let plain = match secretbox::open(ciphertext, &self.symnonce, &self.symkey) {
            Ok(o) => o,
            Err(_) => return None,
        };
        String::from_utf8(plain).ok()
    }
}
