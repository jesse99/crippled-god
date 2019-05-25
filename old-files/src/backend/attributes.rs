use super::rng::*;
use super::*;
use std::f32;

#[derive(Clone, Copy, Debug)]
pub enum Brand {
	Physical,
}

#[derive(Debug)]
pub struct Attack {
	pub name: String,
	pub damage: i32,
	pub brand: Brand,
}

/// The custom bits of a character.
pub struct Attributes {
	/// Hitpoints. When this goes to zero the character dies.
	pub max_hps: fn(rng: &mut RNG) -> i32,

	/// Range is [0, 100]. At 0 attacks have no damage reduction. At 100 attacks are completely
	/// blocked.
	pub resistence: fn(brand: Brand) -> i32,

	/// Should be based on BASE_MOVEMENT_SPEED. Use f32::INFINITY for impassable terrain.
	pub movement_delay: fn(terrain: Terrain) -> f32,

	/// Any number of attacks are allowed. Damage can be zero for any attack.
	pub attacks: fn(rng: &mut RNG) -> Vec<Attack>,
}

pub fn attributes(name: CharName) -> Attributes {
	match name {
		CharName::Ay => Attributes {
			max_hps: |_| 100,
			resistence: |_| 0,
			movement_delay: |terrain| normal_movement_delay(0.8, terrain),
			attacks: |_| {
				vec![Attack {
					name: "bites".to_string(),
					damage: 10, // TODO: maybe we should use a fn to scale damage
					brand: Brand::Physical,
				}]
			},
		},
		CharName::Bhederin => Attributes {
			max_hps: |_| 60,
			resistence: |_| 0,
			movement_delay: |terrain| normal_movement_delay(0.9, terrain),
			attacks: |_| {
				vec![Attack {
					name: "bites".to_string(),
					damage: 20,
					brand: Brand::Physical,
				}]
			},
		},
		CharName::Human => Attributes {
			max_hps: |_| 75,
			resistence: |_| 0,
			movement_delay: |terrain| normal_movement_delay(1.0, terrain),
			attacks: |_| {
				vec![Attack {
					name: "hits".to_string(),
					damage: 15,
					brand: Brand::Physical,
				}]
			},
		},
	}
}

fn normal_movement_delay(scaling: f32, terrain: Terrain) -> f32 {
	const BASE_MOVEMENT_SPEED: f32 = 5.0;

	match terrain {
		Terrain::Blank => {
			assert!(false); // blank should only be used for rendering
			f32::INFINITY
		}
		Terrain::DeepWater => f32::INFINITY,
		Terrain::Ground => scaling * BASE_MOVEMENT_SPEED,
		Terrain::ShallowWater => 0.9 * scaling * BASE_MOVEMENT_SPEED,
		Terrain::Wall => f32::INFINITY,
	}
}
