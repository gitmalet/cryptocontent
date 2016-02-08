use std::io::Write;
use std::io::Read;
use crypto::CryptoManager;
use rustc_serialize::{Encodable, Decodable, json};

pub fn save<W: Write, S: Encodable>(w: &mut W, c: &mut CryptoManager, s: &S) {
    let enc = json::encode(s).unwrap();

    //TODO: Encrypt
    //TODO: Resolve unwrap
    let enc = c.encrypt(&enc).unwrap();
    w.write_all(&enc).unwrap();
}

pub fn load<R: Read, D: Decodable>(c: &CryptoManager, r: &mut R) -> D {
    let mut enc = Vec::new();

    r.read_to_end(&mut enc).unwrap();
    let enc = c.decrypt(enc).unwrap();

    json::decode(&enc).unwrap()
}
