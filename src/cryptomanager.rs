use sodiumoxide::init;
use sodiumoxide::crypto;
use domain::Calendar;

pub fn encrypt(cal: &Calendar) -> Option<Vec<u8>>{
    if !(init()) {
        return None;
    };
    return None;    
}
