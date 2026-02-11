//! Basic library for discord user api endpoints and their typings
//!
//! # Endpoints
//!
//! Each endpoint will have a const unless it requires params then it will be a function that returns a `String`
//!
//! If the endpoint takes request body data, a typed struct with correct se/deserialization will be included
//!
//! If the endpoint returns response body data, a typed struct with correct se/deserialization will be included
//!
//! If a struct value is of type `Option<Option<T>>` thats because `None` will not send the field at all
//! while `Some(None)` will send the field with value of `null`
//! because Discord for whatever reason changes behavior on some endpoints depending on which is sent
//! and also sends back data in this format

#![allow(non_camel_case_types)]

pub mod endpoints;
pub mod types;

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
