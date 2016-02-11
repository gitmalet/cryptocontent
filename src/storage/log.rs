use std::collections::LinkedList;
use chrono::DateTime;
use chrono::Local;
use domain::Content;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Log {
    pub data: LinkedList<LogEntry>,
}

impl Log {
    pub fn new() -> Log {
        Log { data: LinkedList::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn add_entry<C>(&mut self, content: C) -> Result<(), ()>
        where C: Content
    {
        let id: String = content.get_id();
        let enc = try!(content.marshal());
        let entry = LogEntry::new(EntryType::Create, id, enc);
        self.data.push_back(entry);
        Ok(())
    }
}

/// Different types of entries.
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum EntryType {
    Create,
    Update,
    Delete,
}

/// Representation of a single entry in an Eventlog.
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LogEntry {
    pub time: DateTime<Local>,
    pub entry_type: EntryType,
    pub obj_id: String,
    pub data: String,
}

impl LogEntry {
    pub fn new(entry_type: EntryType, obj_id: String, data: String) -> LogEntry {
        LogEntry {
            time: Local::now(),
            entry_type: entry_type,
            obj_id: obj_id,
            data: data,
        }
    }
}
