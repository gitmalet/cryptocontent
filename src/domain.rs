use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;
use chrono::Date;
use chrono::DateTime;
use chrono::Local;
use chrono::Duration;
use serde;

pub struct Account {
    pub items: Vec<Box<serde::Serialize>>
}


/// This struct is used to store information about a single calendar,
/// including the events in it.
///
/// Events are stored in a HashMap, saved as Days containing a list of Events.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Calendar {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub sync: bool,
    days: HashMap<String, Vec<Event>>,
}

/// An Event stores information about, you guessed it, an event in time. They are to be stored in
/// an instance of Calendar.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub location: String,
    pub start: String,
    pub end: String,
}

impl Calendar {

    /// Function to create a new Calendar struct with name and description.
    /// The sync bit is there to determine if this calendar is to be synced with online storage or
    /// not.
    pub fn new(name: &str, desc: &str, sync: bool) -> Calendar {
        Calendar {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            desc: desc.to_string(),
            sync: sync,
            days: HashMap::new(),
        }
    }

    /// Returns a vector of all the events currently stored in this instance of Calendar.
    pub fn get_events(&self) -> Vec<&Event> {
        let d = self.days.values();
        d.flat_map(|d| d.into_iter()).collect::<Vec<_>>()
    }

    /// Returns a slice of all the Events on the specified date. None if no event is saved for the
    /// given date.
    pub fn get_events_by_day(&self, date: Date<Local>) -> Option<&[Event]> {
        let date = d_to_string(date);

        match self.days.get(&date) {
            Some(d) => Some(d),
            None => None,
        }
    }

    /// Stores an Event in the Calendar. If the date of the event isn't already a key in events
    /// hashmap the key is generated and event is saved in it's value list.
    /// TODO: WTF Ownership madness
    pub fn add_event(&mut self, e: Event) {
        let date = d_to_string(to_date(e.start.clone()));
        //let mut d2 = date.clone();

        if !(self.days.contains_key(&date)) {
            self.days.insert(date.clone(), Vec::new());
        }

        self.days.get_mut(&date).unwrap().push(e);
    }

    /// Deletes an Event in the Calendar. If the event is not found nothing happens.
    pub fn delete_event(&mut self, e: &Event) {
        let date = d_to_string(to_date(e.start.clone()));

        if !(self.days.contains_key(&date)) {
            return
        }

        let index = match self.days.get(&date).unwrap().iter().position(|x| x.id == e.id) {
            Some(i) => i,
            None => return,
        };

        self.days.get_mut(&date).unwrap().remove(index);
    }

    /// Repeats the event n times changing only the dates, with one week distance between them.
    pub fn repeat_event_n_times(&mut self, e: &Event, n: usize) {
        for _ in 0..n {
            let er = e.repeat(Duration::weeks(1));
            self.add_event(er);
        }
    }
}


impl Event {
    pub fn new(name: &str, desc: &str, location: &str) -> Event {
        // Is this correct Rust style? (starting a variable with underscore)
        let mut _start = dt_to_string(Local::now());
        let mut _end = dt_to_string(Local::now() + Duration::hours(1));

        Event {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            desc: desc.to_string(),
            location: location.to_string(),
            start: _start,
            end: _end,
        }
    }

    /// Repeats the event, returning the new instance, starting at given date and time. The
    /// difference between start and end date and time of the two events is the same.
    pub fn repeat(&self, distance: Duration) -> Event {
        let mut _start = to_datetime(self.start.clone());
        let mut _end = to_datetime(self.end.clone());

        Event {
            id: Uuid::new_v4().to_string(),
            name: self.name.clone(),
            desc: self.desc.clone(),
            location: self.location.clone(),
            start: dt_to_string(_start + distance),
            end: dt_to_string(_start + distance + (_end - _start)),
        }
    }

    pub fn get_start(&self) -> DateTime<Local> {
        return to_datetime(self.start.clone());
    }

    pub fn get_end(&self) -> DateTime<Local> {
        return to_datetime(self.end.clone());
    }
}

fn dt_to_string(date: DateTime<Local>) -> String {
    date.to_rfc3339()
}

fn d_to_string(date: Date<Local>) -> String {
    date.format("%Y-%m-%d").to_string()
}

fn to_datetime(s: String) -> DateTime<Local> {
    match s.parse::<DateTime<Local>>() {
        Ok(o) => return o,
        Err(e) => panic!("Dateconversion failed: {}", e.description()),
    };
}

fn to_date(s: String) -> Date<Local> {
   to_datetime(s).date()
}
