extern crate chrono;
extern crate dirs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
//#[macro_use]
extern crate structopt;
extern crate termion;
extern crate toml;

mod backend;
mod terminal;

// use backend::{Level, Location};
use slog::Drain;
use std::env;
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

fn find_config_path() -> Result<String, String> {
	let file_name = "crippled-god.toml";
	if let Some(ref mut path) = dirs::home_dir() {
		path.push(file_name);
		if path.as_path().is_file() {
			return Ok(path.to_str().unwrap().to_string());
		}
	}
	if let Ok(ref mut path) = env::current_dir() {
		path.push(file_name);
		if path.as_path().is_file() {
			return Ok(path.to_str().unwrap().to_string());
		}
	}
	Err(format!(
		"Couldn't find {} in the home or working directories.",
		file_name
	))
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
	let local = chrono::Local::now();
	info!(app_logger, "started up"; "seed" => options.seed, "on" => local.to_rfc2822());

	let config_path = match find_config_path() {
		Ok(path) => Some(path),
		Err(err) => {
			error!(app_logger, "{}", err);
			None
		}
	};

	terminal::run(config_path, &root_logger, options.seed);
}
