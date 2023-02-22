use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Mods {
	pub mods: Vec<Mod>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mod {

	// Optional fields
	pub name: Option<String>,
	pub summary: Option<String>,
	pub description: Option<String>,
	pub picture_url: Option<String>,
	pub mod_downloads: Option<u64>,
	pub mod_unique_downloads: Option<u64>,

	pub uid: u64,
	pub mod_id: u32,
	pub game_id: u32,
	pub allow_rating: bool,
	pub domain_name: String,
	pub category_id: u8,
	pub version: String,
	pub endorsement_count: u32,
	pub created_timestamp: u32,
	pub created_time: String,
	pub updated_timestamp: u32,
	pub updated_time: String,
	pub author: String,
	pub uploaded_by: String,
	pub uploaded_users_profile_url: String,
	pub contains_adult_content: bool,
	pub status: String,
	pub available: bool,
	pub user: _User,
	pub endorsement: Option<_Endorsement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct _User {
	pub member_id: u32,
	pub member_group_id: u8,
	pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct _Endorsement {
	endorse_status: String,
	timestamp: Option<String>,
	version: Option<String>,
}
