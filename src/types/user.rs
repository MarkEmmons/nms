use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
	pub user_id: u32,
	// TODO: Redact key
	pub key: String,
	pub name: String,
	pub email: String,
	pub profile_url: Option<String>,
	pub is_supporter: bool,
	pub is_premium: bool,
}
