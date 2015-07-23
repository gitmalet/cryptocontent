use uuid::Uuid;
use chrono::DateTime;
use chrono::Local;
use chrono::FixedOffset;
use chrono::Duration;
use rustc_serialize::Encodable;
use rustc_serialize::Decodable;
use rustc_serialize::Encoder;
use rustc_serialize::Decoder;

/// This struct is used to store information about a single calendar,
/// including the events in it.
///
/// Events are stored as a Vector of Event struct.
#[derive(PartialEq, RustcEncodable,RustcDecodable)]
pub struct Calendar {
    pub id: Uuid,
    pub name: String,
    pub desc: String,
    pub sync: bool,
    events: Vec<Event>,
}

impl Calendar {
    /// Function to create a new Calendar struct with name and description.
    /// The sync bit is there to determine if this calendar is to be synced with online storage or
    /// not.
    pub fn new(name: &str, desc: &str, sync: bool) -> Calendar {
        Calendar {
            id: Uuid::new_v4(),
            name: name.to_string(),
            desc: desc.to_string(),
            sync: sync,
            events: Vec::new(),
        }
    }

    pub fn get_event(&self, id: &Uuid) -> Option<Event> {
        match self.events.iter().find(|&x| x.id == *id) {
            Some(i) => Some(i.clone()),
            None => None,
        }
    }

    fn get_positione(&self, id: &Uuid) -> Option<usize> {
        match self.events.iter().position(|&x| x.id == *id) {
            Some(i) => Some(i),
            None => None,
        }

    }
    pub fn add_event(&mut self, e: Event) {
        self.events.push(e);
    }

    pub fn delete_event(&mut self, id: &Uuid) {
        let index = match self.get_position(id) {
            Some(i) => i,
            None => return,
        };
        self.events.remove(index);
    }

    pub fn repeat_event_n_times(&mut self, id: &Uuid, n: usize) {
        let e = match self.events.iter().find(|x| &x.id == id) {
            Some(i) => i.repeat(i.start + Duration::weeks(1)),
            None => return,
        };
        for i in 0..n {
            self.add_event(e.clone());

            let e = e.repeat(e.start + Duration::weeks(1));
        }
    }


    pub fn get_events(&self) -> &[Event] {
        &self.events
    }
}

#[derive(PartialEq, Clone, RustcEncodable, RustcDecodable)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub desc: String,
    pub location: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

impl Event {
    pub fn new(name: &str, desc: &str, location: &str) -> Event {
        Event {
            id: Uuid::new_v4(),
            name: name.to_string(),
            desc: desc.to_string(),
            location: location.to_string(),
            start: Local::now(),
            end: Local::now() + Duration::hours(1),
        }
    }

    pub fn repeat(&self, start: DateTime<Local>) -> Event {
        Event {
            id: Uuid::new_v4(),
            name: self.name.clone(),
            desc: self.desc.clone(),
            location: self.location.clone(),
            start: start,
            end: start + (self.end - self.start),
        }
    }
}
