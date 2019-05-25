//#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
	name = "Crippled God",
	about = "Rogue-like based on the Malazan Books of the Fallen."
)]
struct Options {
	#[structopt(long = "log-path", default_value = "crippled-god.log")]
	log_path: String,

	// #[structopt(long = "log-level", default_value = "42")]
	// log_level: log::LogLevelFilter,
	#[structopt(long = "seed", default_value = "0")]
	seed: usize,
}

fn main() {
	let options = Options::from_args();
	println!("log_path={} seed={}", options.log_path, options.seed);
}
