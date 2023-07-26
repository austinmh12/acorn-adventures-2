use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	},
};

use super::{
	materials::Materials,
	monster::Monster
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
	pub name: String,
	pub materials: Materials,
	pub monsters: Vec<Monster>,
}