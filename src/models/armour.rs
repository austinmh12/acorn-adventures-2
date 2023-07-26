use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armour {
	pub name: String,
	pub defense: i64,
}