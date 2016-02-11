pub mod calendar;

pub struct Account {
    pub items: Vec<Box<Content>>,
}

pub trait Content {
    fn get_id(&self) -> String;
    fn is_synchronised(&self) -> bool;
    fn marshal(&self) -> Result<String, ()>;
    fn unmarshal(&str) -> Result<Self, ()> where Self: Sized;
}
