use serde::{Deserialize, Serialize};

use crate::types::category::Category;

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
	id: u32,
	name: String,
	forum_url: String,
	nexusmods_url: String,
	genre: String,
	file_count: u32,
	downloads: u64,
	domain_name: String,
	approved_date: u64,
	file_views: u64,
	authors: u32,
	file_endorsements: u64,
	mods: u32,
	categories: Vec<Category>,
}
