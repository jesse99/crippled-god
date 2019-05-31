#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
//#[macro_use]
extern crate structopt;

mod backend::{Level, Location};

use slog::Drain;
use std::fs::OpenOptions;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
	name = "Crippled God",
	about = "Rogue-like based on the Malazan Books of the Fallen."
)]
struct Options {
	#[structopt(long = "log-path", default_value = "crippled-god.log")]
	log_path: String,

	#[structopt(long = "log-level", default_value = "info")]
	log_level: String,

	#[structopt(long = "seed", default_value = "0")]
	seed: u64,
}

fn main() {
	let options = Options::from_args();

	let log_file = OpenOptions::new()
		.create(true)
		.write(true)
		.truncate(true)
		.open(options.log_path)
		.unwrap();

	let level = match slog::Level::from_str(&options.log_level) {
		Ok(l) => l,
		Err(_) => {
			eprintln!("--log-level should be critical, error, warning, info, debug, or trace");
			std::process::exit(1);
		}
	};
	let decorator = slog_term::PlainDecorator::new(log_file);
	let drain = slog_term::FullFormat::new(decorator).build().fuse();
	let drain = slog_async::Async::new(drain).build().fuse();
	let drain = slog::LevelFilter::new(drain, level).fuse();

	let root_logger = slog::Logger::root(drain, o!());

	let app_logger = root_logger.new(o!("version" => env!("CARGO_PKG_VERSION")));
	info!(app_logger, "started up"; "seed" => options.seed);

	let level_logger = root_logger.new(o!());
	let mut level = Level::new(level_logger);
	backend::player_system::delta_player_system(&mut level, Location::new(0, 1));
	backend::player_system::delta_player_system(&mut level, Location::new(0, -1));
	backend::player_system::delta_player_system(&mut level, Location::new(0, -1));
}
