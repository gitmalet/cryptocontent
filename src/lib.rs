//#![feature(trace_macros)]
//trace_macros!(true);

extern crate chrono;
extern crate uuid;
extern crate rustc_serialize;
extern crate sodiumoxide;

mod domain;
mod cryptomanager;

#[cfg(test)]
mod tests {

    use domain::Calendar;
    use domain::Event;
    use chrono::Duration;
    use rustc_serialize::json;
    use std::fs::OpenOptions;
    use std::fs::File;
    use std::path::Path;
    use std::io::Write;
    use std::io::BufWriter;
    use cryptomanager::CryptoManager;
    use std::error::Error;
    use std::fs;
    #[test]
    fn test_calendar() {
        let cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        assert!(cal.name == "TestCalendar");
        assert!(cal.desc == "This is a test instance for calendar");
        assert!(cal.sync == true);
    }


    #[test]
    fn test_event() {
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");
        assert!(eve.name == "TestEvent");
        assert!(eve.desc == "This is a test instance for event");
        assert!(eve.location == "There");

        let e2 = eve.repeat(eve.start + Duration::weeks(1));
        assert!(e2.name == "TestEvent");
        assert!(e2.desc == "This is a test instance for event");
        assert!(e2.location == "There");

        assert_eq!(e2.start - eve.start, Duration::weeks(1));
    }

    #[test]
    fn test_calendar_event() {
        let mut cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");

        cal.add_event(eve.clone());
        assert!(cal.get_events_by_day(&eve.start.date()).unwrap()[0].id == eve.id);

        assert!(cal.get_events_by_day(&eve.start.date()).unwrap()[0].name == "TestEvent");

        cal.delete_event(&eve);
        assert!(cal.get_events_by_day(&eve.start.date()).unwrap().is_empty());
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
        let mut cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");
        let id = eve.id;

        //Testing Calendar alone
        let enc = json::encode(&cal).unwrap();

        let mut options = OpenOptions::new();
        options.write(true).truncate(true).create(true);

        let path = Path::new("/home/malet/dev/Rust/cryptocontent/test_file1.json");
        let file = options.open(path).unwrap();
        let mut writer = BufWriter::new(&file);

        writer.write_all(&enc.clone().into_bytes());

        let mut dec: Calendar = json::decode(&enc).unwrap();
        assert_eq!(cal, dec);

        //Testing Event alone
        let enc = json::encode(&eve).unwrap();

        let mut options = OpenOptions::new();
        options.write(true).truncate(true).create(true);

        let path = Path::new("/home/malet/dev/Rust/cryptocontent/test_file2.json");
        let file = options.open(path).unwrap();
        let mut writer = BufWriter::new(&file);

        writer.write_all(&enc.clone().into_bytes());

        let mut dec: Event = json::decode(&enc).unwrap();

        assert_eq!(eve, dec);
        fs::remove_file("/home/malet/dev/Rust/cryptocontent/test_file1.json").unwrap();
        fs::remove_file("/home/malet/dev/Rust/cryptocontent/test_file2.json").unwrap();

    }

    #[test]
    fn test_whole_serialize() {
        let mut cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");
        let id = eve.id;

        //Testing Calendar with event in it
        cal.add_event(eve.clone());
        
        let enc = match json::encode(&cal) {
            Ok(o) => o,
            Err(e) => panic!("Panic at encoding {},\nDescription: {},\nCause: {}", e, e.description(), match e.cause() {
                Some(o) => o.description(),
                None => "No cause found",
            }),
        };

        let mut options = OpenOptions::new();
        options.write(true).truncate(true).create(true);

        let path = Path::new("/home/malet/dev/Rust/cryptocontent/test_file3.json");
        let file = options.open(path).unwrap();
        let mut writer = BufWriter::new(&file);
        writer.write_all(&enc.clone().into_bytes());

        let mut dec: Calendar = match json::decode(&enc) {
            Ok(t) => t,
            Err(e) => panic!("Panic at decoding {},\nDescription: {}", e, e.description()),
        };

        assert_eq!(dec.get_events_by_day(&eve.start.date()).unwrap()[0].name, "TestEvent");

        dec.delete_event(&eve);
        assert!(dec.get_events_by_day(&eve.start.date()).unwrap().is_empty());
        fs::remove_file("/home/malet/dev/Rust/cryptocontent/test_file1.json").unwrap();
    }

    #[test]
    fn test_encrypt() {
        let cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);

        let cm = CryptoManager::new();
        
        let enc = match json::encode(&cal) {
            Ok(o) => o,
            Err(e) => panic!("Panic at encoding {}, Description: {}", e, e.description()),
        };

        let cipher = match cm.encrypt(&enc) {
            Some(s) => s,
            None => panic!("Failed to encrypt"),
        };
        let plain = match cm.decrypt(&cipher) {
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
        let plain = match cm.decrypt(&cipher) {
            Some(s) => s,
            None => panic!("Failed to decrypt"),
        };

        assert_eq!("hello world!".to_string(), plain);

        cm.new_nonce();
        let cipher = match cm.encrypt("hello world! 2") {
            Some(s) => s,
            None => panic!("Failed to encrypt"),
        };
        let plain = match cm.decrypt(&cipher) {
            Some(s) => s,
            None => panic!("Failed to decrypt"),
        };

        assert_eq!("hello world! 2".to_string(), plain);
    }
}
