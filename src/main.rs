#[macro_use]
extern crate slog;
// extern crate slog_async;
// extern crate slog_term;

mod core;
mod level;
mod level_generator;
mod player;
mod terminal;

use crate::core::*;
use level_generator::*;
use player::*;
use sloggers::Build;
use std::str::FromStr;
use terminal::*;

fn main() {
	// let severity = match sloggers::types::Severity::from_str(&options.log_level) {
	let severity = match sloggers::types::Severity::from_str("debug") {
		Ok(l) => l,
		Err(_) => {
			eprintln!("--log-level should be critical, error, warning, info, debug, or trace");
			std::process::exit(1);
		}
	};

	// "event" => event			uses slog::Value trait (so that output is structured)
	// "event" => %event		uses Display trait
	// "event" => ?event		uses Debug trait
	let path = std::path::Path::new("crippled-god.log");
	let mut builder = sloggers::file::FileLoggerBuilder::new(path);
	builder.format(sloggers::types::Format::Compact);
	builder.overflow_strategy(sloggers::types::OverflowStrategy::Block); // TODO: logging is async which is kinda lame
	builder.source_location(sloggers::types::SourceLocation::None);
	builder.level(severity);
	builder.truncate();
	let root_logger = builder.build().unwrap();

	let local = chrono::Local::now();
	info!(root_logger, "started up"; "on" => local.to_rfc2822(), "version" => env!("CARGO_PKG_VERSION"));
	//	info!(root_logger, "started up"; "seed" => options.seed, "on" => local.to_rfc2822());

	let mut store = EventStore::new();
	let mut level = level::Level::new();
	let mut level_gen = LevelGenerator::new();
	let mut player = Player::new();
	let mut terminal = Terminal::new(&root_logger);

	let mut queued = QueuedEvents::new();
	queued.push_back(Event::NewBranch);

	loop {
		// Handle all the events that are queued up.
		match process_events(
			&root_logger,
			&mut queued,
			&mut store,
			&mut level,
			&mut level_gen,
			&mut player,
			&mut terminal,
		) {
			TerminalEventResult::NotRunning => break,
			TerminalEventResult::Running => (),
		}

		// Once all the services have processed figure out which service will be
		// ready next and queue up an event to advance time to that point.
		let time = find_next_scheduled(&level, &level_gen, &terminal);
		queued.push_back(Event::AdvanceTime(time));
	}
}

fn process_events(
	root_logger: &slog::Logger,
	queued: &mut QueuedEvents,
	store: &mut EventStore,
	level: &mut level::Level,
	level_gen: &mut LevelGenerator,
	player: &mut Player,
	terminal: &mut Terminal,
) -> TerminalEventResult {
	while !queued.is_empty() {
		// Grab the next event,
		let event = queued.pop_front();
		debug!(root_logger, "processing"; "event" => %event);

		// save it into the store (so that if there is a problem we can replay
		// the event that caused it),
		store.append(&event);

		// and give each service a chance to respond to the event.
		level.on_event(&event, queued);
		level_gen.on_event(&event, queued);
		player.on_event(&event, queued, &level);
		match terminal.on_event(&event, queued, &level, player) {
			TerminalEventResult::NotRunning => return TerminalEventResult::NotRunning,
			TerminalEventResult::Running => (),
		}
	}
	TerminalEventResult::Running
}

fn find_next_scheduled(
	level: &level::Level,
	level_gen: &LevelGenerator,
	terminal: &Terminal,
) -> Time {
	let mut time = INFINITE_TIME;

	time = std::cmp::min(time, level.ready_time());
	time = std::cmp::min(time, level_gen.ready_time());
	time = std::cmp::min(time, terminal.ready_time());

	time
}
