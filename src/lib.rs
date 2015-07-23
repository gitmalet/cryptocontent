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
    use cryptomanager;

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
        let id = eve.id;

        cal.add_event(eve);
        assert!(cal.get_events()[0].id == id);

        assert!(cal.get_event(&id).unwrap().name == "TestEvent");

        cal.delete_event(&id);
        assert!(cal.get_events().is_empty());
    }

    #[test]
    fn test_repeat_event() {
        let mut cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");

        let id = eve.id;
        cal.add_event(eve);
        cal.repeat_event_n_times(&id, 5);
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

        //Testing Event alone
        let enc = json::encode(&eve).unwrap();

        let mut options = OpenOptions::new();
        options.write(true).truncate(true).create(true);

        let path = Path::new("/home/malet/dev/Rust/cryptocontent/test_file2.json");
        let file = options.open(path).unwrap();
        let mut writer = BufWriter::new(&file);

        writer.write_all(&enc.clone().into_bytes());

        let mut dec: Event = json::decode(&enc).unwrap();



    }

    #[test]
    fn test_whole_serialize() {
        let mut cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");
        let id = eve.id;

        //Testing Calendar with event in it
        cal.add_event(eve);
        let enc = json::encode(&cal).unwrap();

        let mut options = OpenOptions::new();
        options.write(true).truncate(true).create(true);

        let path = Path::new("/home/malet/dev/Rust/cryptocontent/test_file3.json");
        let file = options.open(path).unwrap();
        let mut writer = BufWriter::new(&file);
        writer.write_all(&enc.clone().into_bytes());
        //assert_eq!(enc, "");

        let mut dec: Calendar = json::decode(&enc).unwrap();

        assert!(dec.get_events()[0].name == "TestEvent");

        dec.delete_event(&id);
        assert!(dec.get_events().is_empty());
    }

    #[test]
    fn test_encrypt() {
        let mut cal = Calendar::new("TestCalendar", "This is a test instance for calendar", true);
        let eve = Event::new("TestEvent", "This is a test instance for event", "There");
        let id = eve.id;

        cryptomanager::encrypt(&cal);
    }
}
