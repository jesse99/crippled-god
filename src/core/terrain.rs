use file_scanner::Scanner;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Result, Write};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Terrain {
    DeepWater,
    Ground,
    ShallowWater,
    Wall,
}

impl Terrain {
    pub fn from_saved(scanner: &mut Scanner<File>) -> Option<Terrain> {
        if let Some(name) = scanner.next() {
            match name.as_str() {
                "DeepWater" => Some(Terrain::DeepWater),
                "Ground" => Some(Terrain::Ground),
                "ShallowWater" => Some(Terrain::ShallowWater),
                "Wall" => Some(Terrain::Wall),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn write(&self, w: &mut BufWriter<File>) -> Result<()> {
        match self {
            Terrain::DeepWater => write!(w, "DeepWater"),
            Terrain::Ground => write!(w, "Ground"),
            Terrain::ShallowWater => write!(w, "ShallowWater"),
            Terrain::Wall => write!(w, "Wall"),
        }
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
