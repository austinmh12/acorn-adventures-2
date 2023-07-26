use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	},
};

use super::element::Element;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
	pub name: String,
	pub base_hp: i64,
	pub defense: i64,
	pub resistance: Element,
	pub weakness: Element,
}