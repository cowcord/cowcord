#![cfg(test)]

mod serialization {
	use serde::*;

	use crate::*;

	#[test]
	fn basics() {
		#[derive(Deserialize, Serialize, Debug, PartialEq)]
		struct Thing {
			#[serde(with = "as_str")]
			foo: Hex,
			#[serde(with = "as_num")]
			bar: Hex,
		}

		let thing = Thing {
			foo: 16777215.into(),
			bar: "123abc".into(),
		};

		let thing_json = serde_json::to_string(&thing).unwrap();

		let json = r#"{"foo":"ffffff","bar":1194684}"#;

		assert_eq!(thing_json, json);
	}

	#[test]
	fn option_nesting() {
		#[derive(Deserialize, Serialize, Debug, PartialEq)]
		struct Thing {
			#[serde(with = "as_str")]
			foo: Option<Hex>,
			#[serde(with = "as_str")]
			bar: Option<Option<Hex>>,
			#[serde(with = "as_str")]
			buzz: Option<Option<Option<Hex>>>,
		}

		let thing = Thing {
			foo: Some("123abc".into()),
			bar: None,
			buzz: Some(Some(Some(16777215.into()))),
		};

		let thing_json = serde_json::to_string(&thing).unwrap();

		let json = r#"{"foo":"123abc","bar":null,"buzz":"ffffff"}"#;

		assert_eq!(thing_json, json);
	}

	#[test]
	fn tuple() {
		#[derive(Deserialize, Serialize, Debug, PartialEq)]
		struct Thing {
			#[serde(with = "as_str")]
			foo: (Hex, Hex),
			#[serde(with = "as_num")]
			bar: Option<Option<(Hex, Hex)>>,
		}

		let thing = Thing {
			foo: (16777215.into(), "000000".into()),
			bar: Some(None),
		};

		let thing_json = serde_json::to_string(&thing).unwrap();

		let json = r#"{"foo":["ffffff","000000"],"bar":null}"#;

		assert_eq!(thing_json, json);
	}

	#[test]
	fn vec() {
		#[derive(Deserialize, Serialize, Debug, PartialEq)]
		struct Thing {
			#[serde(with = "as_str")]
			foo: Vec<Hex>,
			#[serde(with = "as_num")]
			bar: Vec<Hex>,
		}

		let thing = Thing {
			foo: vec![16777215.into(), "000000".into()],
			bar: vec![],
		};

		let thing_json = serde_json::to_string(&thing).unwrap();

		let json = r#"{"foo":["ffffff","000000"],"bar":[]}"#;

		assert_eq!(thing_json, json);
	}
}
