use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

use reqwest::Client;

const URL: &str = "https://raw.githubusercontent.com/discord-userdoccers/discord-protos/refs/heads/master/discord_protos/discord_users/v1/PreloadedUserSettings.proto";

#[tokio::main]
async fn main() {
	let client = Client::new();

	let proto = client.get(URL).send().await.unwrap().text().await.unwrap();

	let root: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..");
	let proto_file = root.join("crates/discord-types/src/api/user_settings_proto.proto");

	let mut file = File::create(&proto_file).unwrap();

	let header = fs::read_to_string(root.join("scripts/update-user-protos/header.txt")).unwrap();

	file.write_all(header.as_bytes()).unwrap();

	let mut file = OpenOptions::new()
		.write(true)
		.append(true)
		.open(&proto_file)
		.unwrap();

	file.write(proto.as_bytes()).unwrap();
}
