use std::collections::LinkedList;
use std::mem;
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
        let e = try!(content.marshal());
        let t = {
            let candidates: Vec<&LogEntry> = self.data.iter().filter(|x| x.obj_id == id).collect();

            if candidates.len() == 0 {
                if content.is_synchronised() {
                    EntryType::Update
                } else {
                    EntryType::Create
                }
            } else {
                if candidates.iter().any(|x| x.entry_type == EntryType::Create) {
                    EntryType::Create
                } else {
                    EntryType::Update
                }
            }
        };
        // TODO: If t == Update, diff marshals and only log the diff
        if t == EntryType::Create {
            self.data = self.data
                            .iter()
                            .filter(|x| x.obj_id != id)
                            .cloned()
                            .collect::<LinkedList<LogEntry>>();
        }
        let entry = LogEntry::new(t, id, e);
        self.data.push_back(entry);
        Ok(())
    }
}

/// Different types of entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum EntryType {
    Create,
    Update,
    Delete,
}

/// Representation of a single entry in an Eventlog.
#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
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
