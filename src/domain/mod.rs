pub mod calendar;

use rustc_serialize::Encodable;

use domain::calendar::Calendar;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Account {
    pub items: Vec<Calendar>,
}

pub trait Content : Encodable {
    fn get_id(&self) -> String;
}
