use super::*;
use file_scanner::Scanner;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Result, Write};

/// This is used to record the facts that happened during a game.
/// Services will update their state as events come in. Events are
/// persisted so that games may be persisted and replayed for
/// debugging purposes (or to recover from a crash).
#[derive(Clone, Debug)]
pub enum Event {
    AdvanceTime(Time),
    // Attacked(ID, ID, DamageType, DamageAmount, Duration),	// need details so UI can render stuff like bolts or big strikes
    /// First event that fires when player enters a brand new level.
    NewBranch, // TODO: probably want to include a branch name
    NewGame,
    /// Fires after level is initialized to allow services to finish initializing.
    NewLevel,
    // NewNPC(Point, ID, HPs),
    /// Update the current level with a name, size, and default terrain.
    /// SetTerrain events will follow this.
    ResetLevel(String, Size, Terrain),

    SetPlayer(Point),

    SetTerrain(Point, Terrain),
}

impl Event {
    pub fn from_saved(scanner: &mut Scanner<File>) -> Option<Event> {
        if let Some(name) = scanner.next() {
            match name.as_str() {
                "AdvanceTime" => {
                    if let Some(time) = Time::from_saved(scanner) {
                        Some(Event::AdvanceTime(time))
                    } else {
                        None
                    }
                }
                "NewBranch" => Some(Event::NewBranch),
                "NewGame" => Some(Event::NewGame),
                "NewLevel" => Some(Event::NewLevel),
                "ResetLevel" => {
                    match (
                        next_string(scanner),
                        Size::from_saved(scanner),
                        Terrain::from_saved(scanner),
                    ) {
                        (Some(name), Some(size), Some(terrain)) => {
                            Some(Event::ResetLevel(name, size, terrain))
                        }
                        _ => None,
                    }
                }
                handle the other cases
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn write(&self, w: &mut BufWriter<File>) -> Result<()> {
        match self {
            Event::AdvanceTime(time) => {
                write!(w, "AdvanceTime ")?;
                time.write(w)
            }
            Event::NewBranch => write!(w, "NewBranch"),
            Event::NewGame => write!(w, "NewGame"),
            Event::NewLevel => write!(w, "NewLevel"),
            Event::ResetLevel(name, size, terrain) => {
                write!(w, "ResetLevel \"{}\" ", name)?; // TODO: need to ensure level name never has a " character
                size.write(w)?;
                write!(w, " ")?;
                terrain.write(w)
            }
            Event::SetPlayer(loc) => {
                write!(w, "SetPlayer ")?;
                loc.write(w)
            }
            Event::SetTerrain(loc, terrain) => {
                write!(w, "SetTerrain ")?;
                loc.write(w)?;
                write!(w, " ")?;
                terrain.write(w)
            }
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::AdvanceTime(t) => write!(f, "AdvanceTime({})", t),
            Event::NewBranch => write!(f, "NewBranch"),
            Event::NewGame => write!(f, "NewGame"),
            Event::NewLevel => write!(f, "NewLevel"),
            Event::ResetLevel(n, s, t) => write!(f, "ResetLevel({}, {}, {})", n, s, t),
            Event::SetPlayer(l) => write!(f, "SetPlayer({})", l),
            Event::SetTerrain(l, t) => write!(f, "SetTerrain({}, {})", l, t),
        }
    }
}
