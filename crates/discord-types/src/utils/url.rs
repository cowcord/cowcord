pub trait ToStringQuery: serde::Serialize {
	/// Convert struct to url string query params.
	///
	/// cannot convert types like Vec or Tuples.
	///
	/// Errors will be hidden and will just return an empty string.
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
	/// assert_eq!(&query.to_string_query(), "?value=hi&value_three=25");
	/// ```
	/// without fields
	/// ```rust
	/// let query = MyStruct {}
	///
	/// assert_eq!(query.to_string_query(), "");
	/// ```
	fn to_string_query(&self) -> String {
		serde_urlencoded::to_string(self)
			.map(|q| format!("?{}", q))
			.unwrap_or_default()
	}
}

// implements the method for anything that derives `serde::ser::Serialize`
impl<T> ToStringQuery for T where T: serde::ser::Serialize {}
