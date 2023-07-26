use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
	Fire,
	Water,
	Earth,
	Air,
	Poison,
	Dragon
}