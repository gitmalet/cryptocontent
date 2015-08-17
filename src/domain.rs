use std::collections::HashMap;
use uuid::Uuid;
use chrono::Date;
use chrono::DateTime;
use chrono::Local;
use chrono::Duration;
use rustc_serialize::Encoder;
use rustc_serialize::Decoder;

/// This struct is used to store information about a single calendar,
/// including the events in it.
///
/// Events are stored in a HashMap, saved as Days containing a list of Events.
#[derive(Debug, PartialEq, RustcEncodable,RustcDecodable)]
pub struct Calendar {
    pub id: Uuid,
    pub name: String,
    pub desc: String,
    pub sync: bool,
    days: HashMap<Date<Local>, Vec<Event>>,
}

/// An Event stores information about, you guessed it, an event in time. They are to be stored in
/// an instance of Calendar.
#[derive(Debug, PartialEq, Clone, RustcEncodable, RustcDecodable)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub desc: String,
    pub location: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
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
            days: HashMap::new(),
        }
    }

    /// Returns a vector of all the events currently stored in this instance of Calendar.
    pub fn get_events(&self) -> Vec<&Event> {
        let d = self.days.values();
        let e = d.flat_map(|d| d.into_iter()).collect::<Vec<_>>();
        e
    }

    /// Returns a slice of all the Events on the specified date. None if no event is saved for the
    /// given date.
    pub fn get_events_by_day(&self, date: &Date<Local>) -> Option<&[Event]> {
        match self.days.get(date) {
            Some(d) => Some(d),
            None => None,
        }
    }

    /// Stores an Event in the Calendar. If the date of the event isn't already a key in events
    /// hashmap the key is generated and event is saved in it's value list.
    pub fn add_event(&mut self, e: Event) {
        if !(self.days.contains_key(&e.start.date())) {
            self.days.insert(e.start.date(), Vec::new());
        }
        self.days.get_mut(&e.start.date()).unwrap().push(e);
    }

    /// Deletes an Event in the Calendar. If the event is not found nothing happens.
    pub fn delete_event(&mut self, e: &Event) {
        if !(self.days.contains_key(&e.start.date())) {
            return
        }
        let index = match self.days.get(&e.start.date()).unwrap().iter().position(|x| x.id == e.id) {
            Some(i) => i,
            None => return,
        };
        self.days.get_mut(&e.start.date()).unwrap().remove(index);
    }

    /// Repeats the event n times changing only the dates, with one week distance between them.
    pub fn repeat_event_n_times(&mut self, e: &Event, n: usize) {
        for _ in 0..n {
            let er = e.repeat(e.start + Duration::weeks(1));
            self.add_event(er);
        }
    }
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

    /// Repeats the event, returning the new instance, starting at given date and time. The
    /// difference between start and end date and time of the two events is the same.
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
