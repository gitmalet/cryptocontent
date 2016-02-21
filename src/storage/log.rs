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

    pub fn count_creates(&self) -> usize {
        self.count_entry_type(EntryType::Create)
    }
    pub fn count_updates(&self) -> usize {
        self.count_entry_type(EntryType::Update)
    }
    pub fn count_removes(&self) -> usize {
        self.count_entry_type(EntryType::Delete)
    }

    fn count_entry_type(&self, t: EntryType) -> usize {
        self.data.iter().filter(|x| x.entry_type == t).count()
    }

    pub fn add_entry<C>(&mut self, content: C) -> Result<(), ()>
        where C: Content
    {
        let id: String = content.get_id();
        let e = try!(content.marshal());
        let t = {
            let candidates: Vec<&LogEntry> = self.data.iter().filter(|x| x.obj_id == id).collect();
            let creates = candidates.iter().any(|x| x.entry_type == EntryType::Create);

            match candidates.len() {
                l if l == 0 && content.is_synchronised() => EntryType::Update,
                l if l == 0 && !(content.is_synchronised()) => EntryType::Create,
                l if l > 0 && creates => EntryType::Create,
                l if l > 0 && !(creates) => EntryType::Update,
                _ => unreachable!(),
            }

        };
        // If Create, delete all old entrys from the local log because they are invalid
        if t == EntryType::Create {
            self.data = self.data
                            .iter()
                            .filter(|x| x.obj_id != id)
                            .cloned()
                            .collect::<LinkedList<LogEntry>>();
        }
        // TODO: If Update, diff marshals and only log the diff
        if t == EntryType::Update {
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
