#![allow(non_camel_case_types)]

use std::fmt;

use hex::{decode, encode};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct Hex(Vec<u8>);

impl Hex {
	pub fn inner(&self) -> &Vec<u8> {
		&self.0
	}

	pub fn to_alphanumeric(&self) -> String {
		encode(&self.0)
	}
}

impl fmt::Display for Hex {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "{}", self.to_alphanumeric())
	}
}

impl<'de> Deserialize<'de> for Hex {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct HexVisitor;

		impl<'de> Visitor<'de> for HexVisitor {
			type Value = Hex;

			fn expecting(
				&self,
				formatter: &mut fmt::Formatter,
			) -> fmt::Result {
				formatter.write_str("a hexadecimal string or number")
			}

			fn visit_str<E>(
				self,
				value: &str,
			) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				Ok(Hex::from(value))
			}

			fn visit_u64<E>(
				self,
				value: u64,
			) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				Ok(Hex::from(format!("{:x}", value)))
			}
		}

		deserializer.deserialize_any(HexVisitor)
	}
}

impl From<&str> for Hex {
	fn from(data: &str) -> Hex {
		Hex(decode(data).unwrap())
	}
}

impl From<String> for Hex {
	fn from(data: String) -> Hex {
		Hex(decode(&data).unwrap())
	}
}

impl From<u64> for Hex {
	fn from(data: u64) -> Hex {
		Hex(decode(format!("{:x}", data)).unwrap())
	}
}

/// serializes/deserialzies hex to/from the encoded, string format
pub mod as_str {
	use serde::{self, Deserialize, Deserializer, Serializer};

	use super::Hex;

	pub fn serialize<'a, S>(
		hex: impl Into<Option<&'a Hex>>,
		serializer: S,
	) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		if let Some(hex) = hex.into() {
			let hex_str = hex.to_alphanumeric();
			serializer.serialize_str(&hex_str)
		} else {
			serializer.serialize_none()
		}
	}

	pub fn deserialize<'de, D, H>(deserializer: D) -> Result<H, D::Error>
	where
		D: Deserializer<'de>,
		H: From<Hex>,
	{
		let hex_str: String = Deserialize::deserialize(deserializer)?;
		Ok(Hex::from(hex_str).into())
	}
}

/// serializes/deserialzies hex to/from the decoded, numerical format
pub mod as_num {
	use serde::{self, Deserialize, Deserializer, Serializer};

	use super::Hex;

	pub fn serialize<'a, S>(
		hex: impl Into<Option<&'a Hex>>,
		serializer: S,
	) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		if let Some(hex) = hex.into() {
			let hex_str = hex.to_alphanumeric();
			let num = u64::from_str_radix(&hex_str, 16).map_err(serde::ser::Error::custom)?;
			serializer.serialize_u64(num)
		} else {
			serializer.serialize_none()
		}
	}

	pub fn deserialize<'de, D, H>(deserializer: D) -> Result<H, D::Error>
	where
		D: Deserializer<'de>,
		H: From<Hex>,
	{
		let num: u64 = Deserialize::deserialize(deserializer)?;
		Ok(Hex::from(format!("{:x}", num)).into())
	}
}
