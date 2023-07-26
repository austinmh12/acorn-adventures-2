use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Materials {
	// Gatherables
	pub herbs: i64,

	// Exepditions

	// Hunts Only
}