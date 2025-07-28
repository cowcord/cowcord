//! ISC License (ISC)
//! Copyright (c) 2016, Serenity Contributors
//!
//! Permission to use, copy, modify, and/or distribute this software for any purpose
//! with or without fee is hereby granted, provided that the above copyright notice
//! and this permission notice appear in all copies.
//!
//! THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
//! REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
//! FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
//! INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
//! OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
//! TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
//! THIS SOFTWARE.

use std::fmt;
use std::fmt::Write;
use std::str::FromStr;

use arrayvec::ArrayString;

/// Hides the implementation detail of ImageHash as an enum.
#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum ImageHashInner {
	Normal { hash: [u8; 16], is_animated: bool },
	Clyde,
}

/// An image hash returned from the Discord API.
///
/// This type can be constructed via it's [`FromStr`] implementation, and can be turned into it's
/// cannonical representation via [`std::fmt::Display`] or [`serde::Serialize`].
///
/// # Example
/// ```rust
/// use serenity::model::misc::ImageHash;
///
/// let image_hash: ImageHash = "f1eff024d9c85339c877985229ed8fec".parse().unwrap();
/// assert_eq!(
/// 	image_hash.to_string(),
/// 	String::from("f1eff024d9c85339c877985229ed8fec")
/// );
/// ```
#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ImageHash(ImageHashInner);

impl ImageHash {
	/// Returns if the linked image is animated, which means the hash starts with `a_`.
	///
	/// # Example
	/// ```rust
	/// use serenity::model::misc::ImageHash;
	///
	/// let animated_hash: ImageHash = "a_e3c0db7f38777778fb43081f8746ebc9".parse().unwrap();
	/// assert!(animated_hash.is_animated());
	/// ```
	#[must_use]
	pub fn is_animated(&self) -> bool {
		match &self.0 {
			| ImageHashInner::Normal {
				is_animated, ..
			} => *is_animated,
			| ImageHashInner::Clyde => true,
		}
	}

	#[must_use]
	fn into_arraystring(self) -> ArrayString<34> {
		let ImageHashInner::Normal {
			hash,
			is_animated,
		} = &self.0
		else {
			return ArrayString::from_str("clyde").expect("the string clyde is less than 34 chars");
		};

		let mut out = ArrayString::new();
		if *is_animated {
			out.push_str("a_");
		}

		for byte in hash {
			write!(out, "{byte:02x}").expect("ImageHash should fit into 34 char ArrayString");
		}

		out
	}
}

impl std::fmt::Debug for ImageHash {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		f.write_str("\"")?;
		<Self as std::fmt::Display>::fmt(self, f)?;
		f.write_str("\"")
	}
}

impl serde::Serialize for ImageHash {
	fn serialize<S: serde::Serializer>(
		&self,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		self.into_arraystring().serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for ImageHash {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let helper = ArrayString::<34>::deserialize(deserializer)?;
		Self::from_str(&helper).map_err(serde::de::Error::custom)
	}
}

impl std::fmt::Display for ImageHash {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.into_arraystring().fmt(f)
	}
}

/// An error returned when [`ImageHash`] is passed an erronous value.
#[derive(Debug, Clone)]
pub enum ImageHashParseError {
	/// The given hash was not a valid [`ImageHash`] length, containing the invalid length.
	InvalidLength(usize),
}

impl std::error::Error for ImageHashParseError {}

impl std::fmt::Display for ImageHashParseError {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			| Self::InvalidLength(length) => {
				write!(f, "Invalid length {length}, expected 32 or 34 characters")
			},
		}
	}
}

impl std::str::FromStr for ImageHash {
	type Err = ImageHashParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (hex, is_animated) = if s.len() == 34 && s.starts_with("a_") {
			(&s[2..], true)
		} else if s.len() == 32 {
			(s, false)
		} else if s == "clyde" {
			return Ok(Self(ImageHashInner::Clyde));
		} else {
			return Err(Self::Err::InvalidLength(s.len()));
		};

		let mut hash = [0u8; 16];
		for i in (0..hex.len()).step_by(2) {
			let hex_byte = &hex[i..i + 2];
			hash[i / 2] = u8::from_str_radix(hex_byte, 16).unwrap_or_else(|_| 0);
		}

		Ok(Self(ImageHashInner::Normal {
			is_animated,
			hash,
		}))
	}
}
