/// Convert struct to url string query params
///
/// eg.
///
/// with field(s)
/// ```rust
/// let query = MyStruct {
///     value: "hi",
///     value_two: None,
///     value_three: 25
/// }
///
/// assert_eq!(to_string_query(&query), "?value=hi&value_three=25");
/// ```
/// without fields
/// ```rust
/// let query = MyStruct {}
///
/// assert_eq!(to_string_query(&query), "");
/// ```
pub fn to_string_query<T: serde::ser::Serialize>(query: &T) -> String {
	serde_urlencoded::to_string(&query)
		.map(|q| format!("?{}", q))
		.unwrap_or_default()
}
