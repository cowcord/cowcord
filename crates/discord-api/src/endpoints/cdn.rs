pub fn USER_AVATAR(
	user_id: &str,
	hash: &str,
) -> String {
	format!("/avatars/{user_id}/{hash}.png")
}
