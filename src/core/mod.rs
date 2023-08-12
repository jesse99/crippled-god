//! Items used by multiple services.
pub mod event;
pub mod events;
pub mod point;
pub mod pov;
pub mod size;
pub mod store;
pub mod terrain;
pub mod time;
pub mod vec2d;

pub use event::Event;
pub use events::{ExecutedEvents, PendingEvents};
pub use point::Point;
pub use pov::POV;
pub use size::Size;
pub use store::*;
pub use terrain::Terrain;
pub use time::*;

use file_scanner::Scanner;
use std::fs::File;

#[cfg(test)]
pub use vec2d::Vec2d;

pub fn next_string(scanner: &mut Scanner<File>) -> Option<String> {
    let old = scanner.get_delim();

    scanner.set_delim_str("\"");
    let result = scanner.next();

    scanner.set_delim(*old);
    match scanner.next() {
        Some(c) if c == "\"" => result,
        _ => None,
    }
}
