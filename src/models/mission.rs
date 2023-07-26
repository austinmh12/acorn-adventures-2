use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	},
};
use chrono::{
	Utc,
	Duration,
	DateTime,
	Local,
};
use std::time::Duration as StdDuration;

use super::{
	area::Area,
	monster::Monster
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MissionType {
	Gather(Area),
	Exepdition(Area),
	Hunt(Area, Monster),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
	pub mission_type: MissionType,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub start_time: DateTime<Utc>,
}