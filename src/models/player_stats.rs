use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
	pub total_missions: i64,
	pub gathers: i64,
	pub expeditions: i64,
	pub hunts: i64,
	pub monsters_discovered: i64,
	pub missions_completed: i64,
	pub missions_failed: i64,
	pub solo_missions: i64,
	pub multiplayer_missions: i64,
	// Weapon stats
}