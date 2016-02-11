pub mod calendar;

use rustc_serialize::Encodable;

use domain::calendar::Calendar;

pub struct Account {
    pub items: Vec<Box<Content>>,
}

pub trait Content {
    fn get_id(&self) -> String;
    fn marshal(&self) -> Result<String, ()>;
    fn unmarshal(&str) -> Result<Self, ()> where Self: Sized;
}
