use clap::Parser;

/// nms - Nexus Mod Scraper
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct NmsOpts {

	/// The apikey for authentication
	#[arg(short, long)]
	pub apikey: String,

	/// The route to query
	#[arg(short, long)]
	pub route: String,

	/// The game
	#[arg(short, long, default_value = "stardewvalley")]
	pub game: String,

	/// The mod
	#[arg(short, long, default_value = "")]
	pub mod_id: String,

	/// The file
	#[arg(short, long, default_value = "")]
	pub file_id: String,
}
