use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	},
};

use super::element::Element;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponType {
	Sword,
	Hammer,
	Lance,
	Bow,
	Glaive,
	Dagger,
	Axe,
	Mace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
	pub name: String,
	pub weapon_type: WeaponType,
	pub damage: i64,
	pub element: Option<Element>,
	pub element_damage: i64,
}