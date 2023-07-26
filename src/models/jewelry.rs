use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JewelryBoost {
	Attack(i64),
	Defense(i64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amulet {
	pub boost: JewelryBoost
}