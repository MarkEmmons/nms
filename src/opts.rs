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
}
