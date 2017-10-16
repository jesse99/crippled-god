extern crate rand;
extern crate termion;

#[macro_use]
mod common;
mod engine;
mod game;
mod terminal;

fn main() {
    terminal::run();
}
