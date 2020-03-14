#[macro_use]
extern crate slog;
// extern crate slog_async;
// extern crate slog_term;

mod core;
mod level;
mod level_generator;
mod player;
mod terminal;

use slog::Drain;
use std::fs::OpenOptions;
use std::str::FromStr;

fn main() {
	let mut store = core::EventStore::new();
	let mut level = level::Level::new();
	let mut level_gen = level_generator::LevelGenerator::new();
	let mut player = player::Player::new();
	let mut terminal = terminal::Terminal::new();

	let mut queued = core::QueuedEvents::new();
	queued.push_back(core::Event::NewBranch);

	// let level = match slog::Level::from_str(&options.log_level) {
	let log_level = match slog::Level::from_str("debug") {
		Ok(l) => l,
		Err(_) => {
			eprintln!("--log-level should be critical, error, warning, info, debug, or trace");
			std::process::exit(1);
		}
	};

	let log_file = OpenOptions::new()
		.create(true)
		.write(true)
		.truncate(true)
		.open("crippled-god.log")
		//		.open(options.log_path)
		.unwrap();
	let decorator = slog_term::PlainDecorator::new(log_file);
	let drain = slog_term::FullFormat::new(decorator).build().fuse();
	let drain = slog_async::Async::new(drain).build().fuse();
	let drain = slog::LevelFilter::new(drain, log_level).fuse();
	let root_logger = slog::Logger::root(drain, o!());

	let local = chrono::Local::now();
	info!(root_logger, "started up"; "on" => local.to_rfc2822(), "version" => env!("CARGO_PKG_VERSION"));
	//	info!(root_logger, "started up"; "seed" => options.seed, "on" => local.to_rfc2822());
	let mut running = true;
	while running {
		// Grab the next event,
		let event = queued.pop_front();
		debug!(root_logger, "processing"; "event" => %event);

		// save it into the store (so that if there is a problem we can replay
		// the event that caused it),
		store.append(&event);

		// and give each service a chance to respond to the event.
		level.on_event(&event, &mut queued);
		level_gen.on_event(&event, &mut queued);
		player.on_event(&event, &mut queued, &level);
		running = terminal.on_event(&event, &mut queued, &level);
	}
}
