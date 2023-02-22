use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
	category_id: u8,
	name: String,
	parent_category: Option<u8>,
}
