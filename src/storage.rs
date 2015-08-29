use std::io::Write;
use std::io::Read;
use serde;
use serde_json;
use crypto::CryptoManager;

pub fn save<W: Write, S: serde::Serialize>(w: &mut W, c: &mut CryptoManager, s: &S) {
    let enc = serde_json::to_string(s).unwrap();

    //TODO: Encrypt
    //TODO: Resolve unwrap
    let enc = c.encrypt(&enc).unwrap();
    w.write_all(&enc).unwrap();
}

pub fn load<R: Read, D: serde::Deserialize>(c: &CryptoManager, r: &mut R) -> D {
    let mut enc = Vec::new();

    r.read_to_end(&mut enc);
    let enc = c.decrypt(enc).unwrap();

    serde_json::from_str(&enc).unwrap()
}
