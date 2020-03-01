use super::rng::*;
use super::scheduled::*;
use super::vec2::*;
use super::*;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
pub enum Key {
	UpArrow,
	DownArrow,
	LeftArrow,
	RightArrow,
	Char(char),
}

/// Used with Message.
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Topic {
	/// An operation could not be completed.
	Error,

	/// Something that doesn't affect the game.
	NonGamePlay,

	/// NPC was damaged (but not by the player).
	NpcIsDamaged, // TODO: might want to have a separate Topic for player allies

	/// NPC was attacked but not damaged (but not by the player).
	NpcIsNotDamaged,

	/// The player has caused damage.
	PlayerDidDamage,

	/// The player attacked byt did no damage.
	PlayerDidNoDamage,

	/// The player has taken damage.
	PlayerIsDamaged,

	/// The player was attacked but took no damage.
	PlayerIsNotDamaged,

	/// The player will not operate less well.
	PlayerIsImpaired, // TODO: probably also want a PlayerEnchanced

	/// The player is at risk of taking damage.
	PlayerIsThreatened,

	/// An operation was not completely successful.
	Warning,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
	pub topic: Topic,
	pub text: String,
}

#[derive(Deserialize, Serialize)]
pub struct Game {
	config: Config,
	level: Level,
	messages: VecDeque<Message>,
	rng: RNG,
	game_time: Time,
	running: bool,
}

impl Game {
	pub fn new(config_file: Result<String, String>, seed: u64) -> Game {
		let mut rng = RNG::new(seed);

		let mut messages = VecDeque::new();
		const VERSION: &str = env!("CARGO_PKG_VERSION");
		let greeting = format!("Welcome to the Crippled God version {}", VERSION);
		messages.push_back(Message {
			topic: Topic::NonGamePlay,
			text: greeting.to_string(),
		});

		let config = Config::default(config_file);
		let level = Level::new(&mut rng);
		let game_time = Time::zero();
		let running = true;
		let mut game = Game {
			config,
			level,
			messages,
			rng,
			game_time,
			running,
			// rng,
		};
		game.reload_config();
		game
	}

	pub fn with_saved(mut self) -> Game {
		self.running = true;
		self
	}

	pub fn config(&self) -> &Config {
		&self.config
	}

	pub fn rng(&mut self) -> &mut RNG {
		&mut self.rng
	}

	pub fn messages(&self) -> &VecDeque<Message> {
		&self.messages
	}

	pub fn running(&self) -> bool {
		self.running
	}

	pub fn add_message(&mut self, topic: Topic, text: &str) {
		info!("{}", text);
		self.messages.push_back(Message {
			topic,
			text: text.to_string(),
		});
		while self.messages.len() > self.config.scroll_back {
			self.messages.pop_front();
		}
	}

	pub fn get_tiles(&mut self, screen_size: Size) -> Vec2<Tile> {
		self.level.get_tiles(screen_size)
	}

	/// If it's the player's turn to move then this just returns true. Otherwise it calls execute
	/// on the next Scheduled object which is ready.
	pub fn players_time_slice(&mut self) -> bool {
		let player_time = self.level.player().ready_time();
		match self.level.other_ready_time() {
			Some(other_time) if other_time < player_time => {
				self.game_time = other_time;
				self.level.execute_others(self.game_time, &mut self.rng);
				false
			}
			_ => {
				self.game_time = player_time;
				true
			}
		}
	}

	/// Returns false if the key was not handled.
	pub fn handle_key(&mut self, key: Key) -> bool {
		match key {
			Key::UpArrow | Key::Char('8') => self.arrow_key(0, -1),
			Key::DownArrow | Key::Char('2') => self.arrow_key(0, 1),
			Key::LeftArrow | Key::Char('4') => self.arrow_key(-1, 0),
			Key::RightArrow | Key::Char('6') => self.arrow_key(1, 0),

			Key::Char('7') => self.arrow_key(-1, -1),
			Key::Char('9') => self.arrow_key(1, -1),
			Key::Char('1') => self.arrow_key(-1, 1),
			Key::Char('3') => self.arrow_key(1, 1),

			Key::Char('^') => {
				self.reload_config();
				true
			}
			Key::Char('q') => {
				self.running = false;
				true
			}
			_ => false,
		}
	}

	fn reload_config(&mut self) {
		let errors = self.config.reload();
		if errors.is_empty() {
			match self.config.config_path.clone() {
				Ok(path) => self.add_message(Topic::NonGamePlay, &format!("Loaded {}", path)),
				Err(err) => self.add_message(Topic::Warning, &err),
			}
		} else {
			for err in &errors {
				self.add_message(Topic::Error, &format!("config error: {}", err));
			}
		}
	}

	fn arrow_key(&mut self, dx: i32, dy: i32) -> bool {
		let p = self.level.player_loc();
		let loc = Location::new(p.x + dx, p.y + dy);
		if self.level.has_char(loc) {
			let player_loc = self.level.player_loc();
			self.attack(player_loc, loc);
			true
		} else if self.level.player().can_move_to(&self.level, loc) {
			self.level.move_player(loc);
			if let Terrain::ShallowWater = self.level.get_terrain(loc) {
				self.add_message(Topic::PlayerIsImpaired, "You splash through the water.")
			}
			true
		} else {
			false
		}
	}

	fn attack(&mut self, attacker_loc: Location, attackee_loc: Location) {
		fn attacker_label(game: &Game, loc: Location) -> String {
			if game.level.player_loc() == loc {
				"You".to_string()
			} else if game.level.is_visible(game.level.player_loc(), loc) {
				"The ".to_string() + game.level.npc(loc).label()
			} else {
				"Something".to_string()
			}
		}

		fn attackee_label(game: &Game, loc: Location) -> String {
			if game.level.player_loc() == loc {
				"you".to_string()
			} else if game.level.is_visible(game.level.player_loc(), loc) {
				"the ".to_string() + game.level.npc(loc).label()
			} else {
				"something".to_string()
			}
		}

		fn attack_topic(
			game: &Game,
			attacker_loc: Location,
			attackee_loc: Location,
			damage: i32,
		) -> Topic {
			if game.level.player_loc() == attacker_loc {
				if damage > 0 {
					Topic::PlayerDidDamage
				} else {
					Topic::PlayerDidNoDamage
				}
			} else if game.level.player_loc() == attackee_loc {
				if damage > 0 {
					Topic::PlayerIsDamaged
				} else {
					Topic::PlayerIsNotDamaged
				}
			} else if damage > 0 {
				Topic::NpcIsDamaged
			} else {
				Topic::NpcIsNotDamaged
			}
		}

		let attacker = self.level.npc(attacker_loc).name();
		let attackee = self.level.npc(attackee_loc).name();
		let attacks = (attributes(attacker).attacks)(self.rng());
		for attack in &attacks {
			let resist = (attributes(attackee).resistence)(attack.brand);
			let damage = (attack.damage * (100 - resist)) / 100;

			let topic = attack_topic(self, attacker_loc, attackee_loc, damage);
			let attacker = attacker_label(self, attacker_loc);
			let attackee = attackee_label(self, attackee_loc);
			let mesg = if damage > 0 {
				format!(
					"{} {} {} for {} points.",
					attacker, attack.name, attackee, damage
				)
			} else {
				format!("{} {} {} for no damage.", attacker, attack.name, attackee)
			};
			self.add_message(topic, &mesg)
		}
	}
}
