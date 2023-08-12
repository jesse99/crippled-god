use super::*;

use std::fs::File;
use std::io::{BufWriter, Result, Write};

// Events which have executed.
pub struct ExecutedEvents {
    events: Vec<Event>, // TODO: use a deque?
}

// [`Event`]s which are pending execution.
pub struct PendingEvents {
    events: Vec<Event>, // TODO: use a deque?
}

impl ExecutedEvents {
    pub fn new() -> ExecutedEvents {
        ExecutedEvents { events: Vec::new() }
    }

    pub fn append(&mut self, event: &Event) {
        // TODO: persist it (probably want to flush too)
        self.events.push(event.clone());
    }

    // We don't use the Write trait to avoid the dynamic dispatch that will
    // incur (altho that would make unit tests a bit nicer).
    pub fn save(&self, writer: &mut BufWriter<File>) -> Result<()> {
        // TODO:
        // should be able to use Write trait
        for event in self.events.iter() {
            event.write(writer)?;
            writeln!(writer)?;
        }
        Ok(())
    }
}

impl PendingEvents {
    pub fn new() -> PendingEvents {
        PendingEvents { events: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn push_back(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn pop_front(&mut self) -> Event {
        self.events.remove(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn save_events(suffix: &str) -> String {
        let path = format!("/tmp/crippled-god-{}.txt", suffix);
        let f = File::create(&path).unwrap();
        let mut f = BufWriter::new(f);
        let mut events = ExecutedEvents::new();
        events.append(&Event::AdvanceTime(Time::from_secs(1.2)));
        events.append(&Event::NewBranch);
        events.append(&Event::NewGame);
        events.append(&Event::NewLevel);
        events.append(&Event::ResetLevel(
            "town".to_string(),
            Size::new(12, 10),
            Terrain::Wall,
        ));
        events.append(&Event::SetPlayer(Point::new(5, 4)));
        events.append(&Event::SetTerrain(Point::new(2, 3), Terrain::Ground));
        events.save(&mut f).expect("save events failed");
        f.flush().expect("flush failed");
        path
    }

    #[test]
    fn test_write() {
        let path = save_events("write");
        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(
            contents,
            "AdvanceTime 12
NewBranch
NewGame
NewLevel
ResetLevel \"town\" 12 10 Wall
SetPlayer 5 4
SetTerrain 2 3 Ground
"
        );
    }

    #[test]
    fn test_round_trip() {
        // load events
        // do a match of the events
        //      verify args are as expected
        //      append a token
        // verify string is as expected
        let path = save_events("round-trip");

        let mut tokens = "";
        assert_eq!(tokens, "AT NB NG NL RL SP, ST");
    }

    // TODO: probably should have a couple more tests:
    // empty file
    // missing file?
    // complete garbage file
    // malformed file
    // maybe one with trailing garbage
}
