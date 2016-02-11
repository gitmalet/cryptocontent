#![feature(box_syntax)]
//! CryptoContent is a library to store data at some cloud storage and manage it with multiple
//! clients.
//! No server application is used for this purpose, so the clients have to manage everything.
//! The data stored on remote destinations by CryptoContent is encrypted.
//! This library uses Sodium crypto libs for all the crypto primitives.

extern crate chrono;
extern crate uuid;
extern crate sodiumoxide;
extern crate rustc_serialize;

/// This module contains all the data types that are used to store information. They are all
/// serializeble and of course also deserializeble.
pub mod domain;

/// Module for encrypting and decrypting stuff.
pub mod crypto;

/// Module for managing storage
pub mod storage;

#[cfg(test)]
mod tests {

    use domain::calendar::Calendar;
    use domain::calendar::Event;
    use chrono::Duration;
    use std::fs::OpenOptions;
    use std::path::Path;
    use std::io::Write;
    use std::io::BufWriter;
    use std::io::BufReader;
    use crypto::CryptoManager;
    use std::error::Error;
    use std::fs;
    use storage::{load, save};
    use storage::log::Log;
    use rustc_serialize::json;

    #[test]
    fn test_calendar() {
        let cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        assert!(cal.name == "TestCalendar");
        assert!(cal.desc == "This is a test instance for calendar");
        assert!(cal.sync == true);
    }

    #[test]
    fn test_event() {
        let eve = Event::new("TestEvent", "This is a test instance", "There");
        assert!(eve.name == "TestEvent");
        assert!(eve.desc == "This is a test instance");
        assert!(eve.location == "There");

        let e2 = eve.repeat(Duration::weeks(1));
        assert!(e2.name == "TestEvent");
        assert!(e2.desc == "This is a test instance");
        assert!(e2.location == "There");

        assert_eq!(e2.start - eve.start, Duration::weeks(1));
    }

    #[test]
    fn test_calendar_event() {
        let mut cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");

        cal.add_event(eve.clone());
        assert!(cal.get_events_by_day(eve.start.date()).unwrap()[0].id == eve.id);

        assert!(cal.get_events_by_day(eve.start.date()).unwrap()[0].name == "TestEvent");

        cal.delete_event(&eve);
        assert!(cal.get_events_by_day(eve.start.date()).unwrap().is_empty());
    }

    #[test]
    fn test_repeat_event() {
        let mut cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");

        cal.add_event(eve.clone());
        cal.repeat_event_n_times(&eve, 5usize);
        assert_eq!(cal.get_events().len(), 6);
    }

    #[test]
    fn test_serialize() {
        let cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");

        //Testing Calendar alone
        let enc = match json::encode(&cal) {
            Ok(o) => o,
            Err(e) => panic!("Encryption error! {}", e.description()),
        };

        let mut options = OpenOptions::new();
        options.write(true).truncate(true).create(true);

        let path = Path::new("test_file1.json");
        let file = options.open(path).unwrap();
        let mut writer = BufWriter::new(&file);

        writer.write_all(&enc.clone().into_bytes()).unwrap();

        let dec: Calendar = match json::decode(&enc) {
            Ok(o) => o,
            Err(e) => panic!("Decryption failed! {} on {}", e.description(), enc),
        };

        assert_eq!(cal, dec);

        //Testing Event alone
        let enc = json::encode(&eve).unwrap();

        let mut options = OpenOptions::new();
        options.write(true).truncate(true).create(true);

        let path = Path::new("test_file2.json");
        let file = options.open(path).unwrap();
        let mut writer = BufWriter::new(&file);

        writer.write_all(&enc.clone().into_bytes()).unwrap();

        let dec: Event = json::decode(&enc).unwrap();

        assert_eq!(eve, dec);
        fs::remove_file("test_file1.json").unwrap();
        fs::remove_file("test_file2.json").unwrap();

    }

    #[test]
    fn test_whole_serialize() {
        let mut cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");

        //Testing Calendar with event in it
        cal.add_event(eve.clone());

        let enc = json::encode(&cal).unwrap();

        let mut options = OpenOptions::new();
        options.write(true).truncate(true).create(true);

        let path = Path::new("test_file3.json");
        let file = options.open(path).unwrap();
        let mut writer = BufWriter::new(&file);
        writer.write_all(&enc.clone().into_bytes()).unwrap();

        let mut dec: Calendar = json::decode(&enc).unwrap();

        assert_eq!(dec.get_events_by_day(eve.start.date()).unwrap()[0].name, "TestEvent");

        dec.delete_event(&eve);
        assert!(dec.get_events_by_day(eve.start.date()).unwrap().is_empty());
        fs::remove_file("test_file3.json").unwrap();
    }

    #[test]
    fn test_encrypt() {
        let cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);

        let mut cm = CryptoManager::new();

        let enc = match json::encode(&cal) {
            Ok(o) => o,
            Err(e) => panic!("Encryption error: {}", e.description()),
        };

        let cipher = match cm.encrypt(&enc) {
            Some(s) => s,
            None => panic!("Failed to encrypt"),
        };
        let plain = match cm.decrypt(cipher) {
            Some(s) => s,
            None => panic!("Failed to decrypt"),
        };

        assert_eq!(enc, plain);
    }

    #[test]
    fn test_encrypt_new_nonce() {
        let mut cm = CryptoManager::new();

        let cipher = match cm.encrypt("hello world!") {
            Some(s) => s,
            None => panic!("Failed to encrypt"),
        };
        let plain = match cm.decrypt(cipher) {
            Some(s) => s,
            None => panic!("Failed to decrypt"),
        };

        assert_eq!("hello world!".to_string(), plain);

        cm.new_nonce();
        let cipher = match cm.encrypt("hello world! 2") {
            Some(s) => s,
            None => panic!("Failed to encrypt"),
        };
        let plain = match cm.decrypt(cipher) {
            Some(s) => s,
            None => panic!("Failed to decrypt"),
        };

        assert_eq!("hello world! 2".to_string(), plain);
    }

    #[test]
    fn test_storage() {
        let cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let mut cm = CryptoManager::new();

        let mut options = OpenOptions::new();
        options.write(true).truncate(true).create(true);

        let path = Path::new("test_file4.json");
        let wfile = options.open(path).unwrap();
        let mut writer = BufWriter::new(wfile);

        save(&mut writer, &mut cm, &cal);

        drop(writer);

        let path = Path::new("test_file4.json");
        let mut options = OpenOptions::new();
        options.read(true);
        let rfile = options.open(path).unwrap();
        let mut reader = BufReader::new(rfile);
        let loadedcal = load(&cm, &mut reader);

        assert_eq!(cal, loadedcal);
        fs::remove_file("test_file4.json").unwrap();
    }

    #[test]
    fn test_log() {
        let mut log = Log::new();
        let mut eve = Event::new("TestEvent", "This is a test", "There");
        let mut sync = Event::new("Synced Event", "This is a test", "There");
        sync.sync == true;

        let counts = |x: &Log| (x.len(), x.count_creates(), x.count_updates(), x.count_removes());

        assert_eq!(counts(&log), (0,0,0,0));
        log.add_entry(eve.clone()).unwrap();
        assert_eq!(counts(&log), (1,1,0,0));

        eve.name = "Changed Name".to_string();
        log.add_entry(eve.clone()).unwrap();
        assert_eq!(counts(&log), (1,1,0,0));

        log.add_entry(sync.clone()).unwrap();
        assert_eq!(counts(&log), (2,1,1,0));

        log.add_entry(sync.clone()).unwrap();
        assert_eq!(counts(&log), (2,1,1,0));
    }
}
