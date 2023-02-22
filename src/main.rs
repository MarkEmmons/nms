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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let opts = NmsOpts::parse();
	//let game = "stardewvalley";
	let route = &opts.route;

	let client = Client::new();

	let req = client
		.get(format!("https://api.nexusmods.com/{}", route))
		.header("User-Agent", "reqwest/0.11.14")
		.header("apikey", &opts.apikey);

	match opts.route.as_str() {

		//format!("/v1/games/{}/mods/updated.json", game) => println!("ROUTE: {}", &opts.route),
		"/v1/games.json" => println!("ROUTE: {}", &opts.route),
		"/v1/users/validate.json" => println!("ROUTE: {}", &opts.route),
		_ => println!("Unrecognized route: {}", &opts.route),
	}

	// TODO: match request to output type
	let res = req
		.send()
		.await?
		.json::<Mod>()
		//.json::<Vec<Mod>>()
		//.json::<Game>()
		//.json::<User>()
		.await?;

	println!("{:#?}", res);
	Ok(())
}

// /v1/games/{game_domain_name}/mods/{mod_id}/changelogs.json
// /v1/games/{game_domain_name}/mods/latest_added.json
// /v1/games/{game_domain_name}/mods/latest_updated.json
// /v1/games/{game_domain_name}/mods/trending.json
// /v1/games/{game_domain_name}/mods/{id}.json
// /v1/games/{game_domain_name}/mods/md5_search/{md5_hash}.json
// /v1/games/{game_domain_name}/mods/{id}/endorse.json
// /v1/games/{game_domain_name}/mods/{id}/abstain.json
// /v1/games/{game_domain_name}/mods/{mod_id}/files.json
// /v1/games/{game_domain_name}/mods/{mod_id}/files/{file_id}.json
// /v1/games/{game_domain_name}/mods/{mod_id}/files/{id}/download_link.json
// /v1/games.json
// /v1/games/{game_domain_name}.json
// /v1/user/tracked_mods.json
// /v1/user/tracked_mods.json
// /v1/user/tracked_mods.json
// /v1/user/endorsements.json
// /v1/colourschemes.json
