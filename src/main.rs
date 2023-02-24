use clap::Parser;
use reqwest::Client;
// TODO: Tracing

use nms::opts::NmsOpts;
#[allow(unused_imports)]
use nms::types::{
	game::Game,
	game_mod::{Mod, Mods},
	user::User,
};

const COLOURSCHEMES: &str = "/v1/colourschemes.json";

const GAME: &str = "/v1/games/{game_domain_name}.json";
const GAMES: &str = "/v1/games.json";

const MOD: &str = "/v1/games/{game_domain_name}/mods/{mod_id}.json";
const MOD_CHANGELOGS: &str = "/v1/games/{game_domain_name}/mods/{mod_id}/changelogs.json";

const MODS_ADDED: &str = "/v1/games/{game_domain_name}/mods/latest_added.json";
const MODS_UPDATED: &str = "/v1/games/{game_domain_name}/mods/latest_updated.json";
const MODS_TRENDING: &str = "/v1/games/{game_domain_name}/mods/trending.json";

const FILE: &str = "/v1/games/{game_domain_name}/mods/{mod_id}/files/{file_id}.json";
const FILES: &str = "/v1/games/{game_domain_name}/mods/{mod_id}/files.json";
const FILE_DOWNLOAD: &str = "/v1/games/{game_domain_name}/mods/{mod_id}/files/{file_id}/download_link.json";

const USERS_VALIDATE: &str = "/v1/users/validate.json";
const USERS_TRACKED: &str = "/v1/user/tracked_mods.json";
const USERS_ENDORSEMENTS: &str = "/v1/user/endorsements.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let opts = NmsOpts::parse();
	let route = &opts.route
		.replace("{game_domain_name}", &opts.game)
		.replace("{file_id}", &opts.file_id)
		.replace("{mod_id}", &opts.mod_id);

	let req = Client::new()
		.get(format!("https://api.nexusmods.com/{}", route))
		.header("User-Agent", "reqwest/0.11.14")
		.header("apikey", &opts.apikey);

	let res = req
		.send()
		.await?;

	match opts.route.as_str() {

		COLOURSCHEMES => todo!(),

		GAME => println!("{}", serde_json::to_string(&res.json::<Game>().await?)?),

		GAMES => println!("{}", serde_json::to_string(&res.json::<Vec<Game>>().await?)?),

		MOD => println!("{}", serde_json::to_string(&res.json::<Mod>().await?)?),

		MOD_CHANGELOGS |
		MODS_ADDED     |
		MODS_UPDATED   |
		MODS_TRENDING => println!("{}", serde_json::to_string(&res.json::<Vec<Mod>>().await?)?),

		FILE => todo!(),
		FILES => todo!(),
		FILE_DOWNLOAD => todo!(),

		USERS_VALIDATE => println!("{}", serde_json::to_string(&res.json::<User>().await?)?),

		USERS_TRACKED => todo!(),
		USERS_ENDORSEMENTS => todo!(),

		_ => println!("Nothing configured for this route: {}", opts.route.as_str()),
	}

	Ok(())
}
