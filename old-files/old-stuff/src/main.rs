#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate rand;
extern crate simplelog;
extern crate termion;
extern crate time;

#[macro_use]
mod common;
mod engine;
mod game;
mod terminal;

use std::fmt::Display;
use std::str::FromStr;
use std::process;

#[derive(Clone)]
struct Config {
    log_file: String,
    log_level: log::LogLevelFilter,
    seed: usize,
}

impl Config {
    fn new() -> Self {
        Config {
            log_file: "crippled-god.log".to_string(),
            log_level: log::LogLevelFilter::Info,
            seed: time::get_time().nsec as usize,
        }
    }
}

fn config_err(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1);
}

// Min and max are inclusive.
fn match_num<T>(matches: &clap::ArgMatches, name: &str, min: T, max: T) -> T
where
    T: Copy + Display + FromStr + PartialOrd,
{
    match value_t!(matches.value_of(name), T) {
        Ok(value) if value < min => config_err(
            &format!("--{} should be greater than {}", name, min),
        ),
        Ok(value) if value > max => config_err(&format!("--{} should be less than {}", name, max)),
        Ok(value) => value,
        _ => config_err(&format!("--{} should be a number", name)),
    }
}

fn parse_options() -> Config {
    let mut config = Config::new();

    // see https://docs.rs/clap/2.24.2/clap/struct.Arg.html#method.from_usage for syntax
    let usage = format!(
        "--log-file=[PATH] 'Where to put the log file [{default_log_file}]'
		--log-level=[LEVEL] 'Default log level: error, warn, info, debug, or trace [{default_level}]'
		--seed=[N] 'Random number generator seed [random]'",
        default_log_file = config.log_file,
        default_level = format!("{}", config.log_level)
    );

    let matches = clap::App::new("crippled-god")
        .version("0.1.0")
        .author("Jesse Jones <jesse9jones@gmail.com>")
        .about("Rogue-like based on the Malazan Books of the Fallen.")
        .args_from_usage(&usage)
        .get_matches();

    if matches.is_present("seed") {
        config.seed = match_num(&matches, "seed", 1, usize::max_value());
    }

    if matches.is_present("log-file") {
        config.log_file = matches.value_of("log-file").unwrap().to_string();
    }

    if matches.is_present("log-level") {
        if let Ok(level) = log::LogLevelFilter::from_str(matches.value_of("log-level").unwrap()) {
            config.log_level = level;
        } else {
            config_err(
                "--level argument should be one of error, warn, info, debug, or trace",
            )
        }
    }

    config
}

fn main() {
    let config = parse_options();
    match std::fs::File::create(&config.log_file) {
        Ok(file) => {
            let _ =
                simplelog::WriteLogger::init(config.log_level, simplelog::Config::default(), file)
                    .unwrap();
            info!("started up with level {:?}", config.log_level);
            terminal::run(config.seed);
        }
        Err(err) => config_err(&format!("Couldn't create log file: {}", err)),
    }
}
