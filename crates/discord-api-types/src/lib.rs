pub const DISCORD_URL: &str = "https://discord.com/";

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum ApiVerion {
	v10,
	v9,
	#[deprecated]
	v8,
	#[deprecated]
	v7,
	#[deprecated]
	v6,
}
