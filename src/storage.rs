use std::io::Write;
use std::io::Read;
use serde;
use serde_json;

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

    pub fn save<T: serde::Serialize>(&mut self, e: T) {
        let enc = serde_json::to_string(&e).unwrap();

        //TODO: Encrypt

        self.write_to.write_all(&enc.into_bytes()).unwrap();
    }
/*
    pub fn load<T: Decodable>(&self) -> T {
        
    }*/
}
