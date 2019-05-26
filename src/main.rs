use std::collections::HashMap; // TODO: may want to use a faster hash
use std::hash::{Hash, Hasher};
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
//#[macro_use]
extern crate structopt;

use slog::Drain;
use std::fs::OpenOptions;
use std::str::FromStr;
use structopt::StructOpt;

use std::sync::atomic::{AtomicUsize, Ordering};

static ENTITY_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Usually entities are indexes into a Vec. But:
// 1) An index isn't very meaningful in isolation.
// 2) Speed isn't a huge concern here so the contiguousness of a Vec isn't too important.
// 3) If we did use a Vec we'd wind up with lots of holes as the player kills off monsters.
#[derive(Clone, Copy, Debug)]
pub struct Entity {
	prefix: &'static str, // static so that we can cheaply copy these
	count: usize,
}

impl Entity {
	fn new(prefix: &'static str) -> Entity {
		Entity {
			prefix,
			count: ENTITY_COUNTER.fetch_add(1, Ordering::SeqCst),
		}
	}
}

impl PartialEq for Entity {
	fn eq(&self, other: &Self) -> bool {
		self.count == other.count
	}
}

impl Eq for Entity {}

impl Hash for Entity {
	fn hash<S: Hasher>(&self, state: &mut S) {
		self.count.hash(state); // count is the unique part of an Enity so we can save time by ignoring prefix
	}
}

impl slog::Value for Entity {
	fn serialize(
		&self,
		_: &slog::Record<'_>,
		key: slog::Key,
		serializer: &mut dyn slog::Serializer,
	) -> Result<(), slog::Error> {
		serializer.emit_arguments(key, &format_args!("{}", self.count))
	}
}

struct PlayerComponent {
	name: String,
}

// top-left is (0, 0)
struct PositionComponent {
	x: i32,
	y: i32,
}

struct Level {
	player_components: HashMap<Entity, PlayerComponent>,
	position_components: HashMap<Entity, PositionComponent>,
}

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
	seed: usize,
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

	let monster_logger = root_logger.new(o!("name" => "Ay", "level" => "Dungeon #5"));
	debug!(monster_logger, "woke up"; "reason" => "heard player");

	let e1 = Entity::new("player");
	let e2 = Entity::new("ay");
	info!(app_logger, "rntities"; "player" => e1, "nps" => e2);
}
