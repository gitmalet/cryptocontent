use sodiumoxide::init;
use sodiumoxide::crypto;
use sodiumoxide::crypto::secretbox;
use domain::Calendar;
use std::str;

pub struct CryptoManager {
    pub key: secretbox::Key,
    pub nonce: secretbox::Nonce,
}

impl CryptoManager {
    
    pub fn new() -> Option<CryptoManager> {
        if !(init()) {
            return None;
        };

        Some(CryptoManager {
            key: secretbox::gen_key(),
            nonce: secretbox::gen_nonce(),
        })
    }

    pub fn encrypt(&self, plaintext: &str) -> Option<Vec<u8>> {
        Some(secretbox::seal(plaintext.as_bytes(), &self.nonce, &self.key))
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Option<String> {
        let plain = match secretbox::open(ciphertext, &self.nonce, &self.key) {
            Ok(o) => o,
            Err(e) => return None,
        };
        String::from_utf8(plain).ok()
    }
}
