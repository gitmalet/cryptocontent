use std::io::Write;
use std::io::Read;
use rustc_serialize::Encodable;
use rustc_serialize::Decodable;
use rustc_serialize::json;

//TODO: Fix to normal data instead of box
pub struct StorageManager {
    pub write_to: Box<Write>,
    //pub read_from: Box<Read>,
}

impl StorageManager {
    pub fn new(w: Box<Write>/*, r: Box<Read>*/) -> StorageManager {
        StorageManager {
            write_to: w,
            //read_from: r,
        }
    }

    pub fn save<T: Encodable>(&mut self, e: T) {
        let enc = json::encode(&e).unwrap();

        //TODO: Encrypt

        self.write_to.write_all(&enc.into_bytes()).unwrap();
    }
/*
    pub fn load<T: Decodable>(&self) -> T {
        
    }*/
}
