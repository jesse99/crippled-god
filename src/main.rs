extern crate chrono;
#[macro_use]
extern crate clap;
extern crate fnv;
#[macro_use]
extern crate log;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate simplelog;
extern crate termion;
extern crate time;
extern crate toml;

use std::env;
use std::process;
use std::str::FromStr;

mod backend;
mod terminal;

#[derive(Clone)]
struct Options {
	log_file: String,
	log_level: log::LevelFilter,
	seed: usize,
}

impl Options {
	fn new() -> Self {
		Options {
			log_file: "crippled-god.log".to_string(),
			log_level: log::LevelFilter::Info,
			seed: time::get_time().nsec as usize,
		}
	}
}

fn options_err(message: &str) -> ! {
	eprintln!("{}", message);
	process::exit(1);
}

fn parse_options() -> Options {
	let mut options = Options::new();

	// see https://docs.rs/clap/2.32.0/clap/ for syntax
	let usage = format!(
        "--log-file=[PATH] 'Where to put the log file [{default_log_file}]'
        --log-level=[LEVEL] 'Default log level: error, warn, info, debug, or trace [{default_level}]'
        --seed=[N] 'Random number generator seed [random]'",
        default_log_file = options.log_file,
        default_level = options.log_level.to_string(),
    );

	let matches = clap::App::new("crippled-god")
		.version("0.1.0")
		.author("Jesse Vorisek <jesse.vorisek@gmail.com>")
		.about("Rogue-like based on the Malazan Books of the Fallen.")
		.args_from_usage(&usage)
		.get_matches();

	if matches.is_present("log-file") {
		options.log_file = matches.value_of("log-file").unwrap().to_string();
	}

	if matches.is_present("log-level") {
		options.log_level = log::LevelFilter::from_str(matches.value_of("log-level").unwrap())
			.unwrap_or_else(|_e| {
				options_err("--level argument should be one of error, warn, info, debug, or trace")
			});
	}

	if matches.is_present("seed") {
		options.seed = value_t!(matches, "seed", usize)
			.unwrap_or_else(|_e| options_err("--seed should be an unsigned int"));
	}

	options
}

fn find_config_path() -> Result<String, String> {
	let file_name = "crippled-god.toml";
	if let Ok(ref mut path) = env::current_dir() {
		path.push(file_name);
		if path.as_path().is_file() {
			return Ok(path.to_str().unwrap().to_string());
		}
	}
	if let Some(ref mut path) = env::home_dir() {
		path.push(file_name);
		if path.as_path().is_file() {
			return Ok(path.to_str().unwrap().to_string());
		}
	}
	Err(format!(
		"Couldn't find {} in the working or home directories.",
		file_name
	))
}

fn main() {
	let options = parse_options();
	match std::fs::File::create(&options.log_file) {
		Ok(file) => {
			let _ =
				simplelog::WriteLogger::init(options.log_level, simplelog::Config::default(), file)
					.unwrap_or_else(|e| options_err(&format!("Couldn't create logger: {}", e)));

			let args: Vec<String> = std::env::args().collect();
			let joined = args.join(" ");
			info!("started up with {}", joined);

			let local = chrono::Local::now();
			info!("on {}", local.to_rfc2822());

			terminal::run(find_config_path(), options.seed);
		}
		Err(err) => options_err(&format!("Couldn't create log file: {}", err)),
	}
}
