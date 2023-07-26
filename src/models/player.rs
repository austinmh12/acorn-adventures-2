use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		oid::ObjectId,
	},
};

use super::{
	armour::Armour,
	jewelry::{Amulet, JewelryBoost},
	materials::Materials,
	medals::Medals,
	mission::{Mission, MissionType},
	monster::Monster,
	pets::Pets,
	player_stats::PlayerStats,
	weapon::Weapon, 
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub guild_id: i64,
	pub user_id: i64,
	pub stats: PlayerStats,
	pub weapon: Option<Weapon>,
	pub helm: Option<Armour>,
	pub chest: Option<Armour>,
	pub legs: Option<Armour>,
	pub boots: Option<Armour>,
	pub amulet: Option<Amulet>,
	pub materials: Materials,
	pub current_mission: Option<Mission>,
	pub zenny: i64,
	pub hunt_tokens: i64,
	pub medals: Medals,
	pub pets: Pets,
	pub monsters_discovered: Vec<Monster>
}