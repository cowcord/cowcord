use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub enum Locale {
	ar,
	bg,
	cs,
	da,
	de,
	el,
	#[serde(rename = "en-GB")]
	en_GB,
	#[default]
	#[serde(rename = "en-US")]
	en_US,
	#[serde(rename = "es-ES")]
	es_ES,
	#[serde(rename = "es-419")]
	es_419,
	fi,
	fr,
	hi,
	hr,
	hu,
	id,
	it,
	ja,
	ko,
	lt,
	nl,
	no,
	pl,
	#[serde(rename = "pt-BR")]
	pt_BR,
	ro,
	ru,
	#[serde(rename = "sv-SE")]
	sv_SE,
	th,
	tr,
	uk,
	vi,
	#[serde(rename = "zh-CN")]
	zh_CN,
	#[serde(rename = "zh-TW")]
	zh_TW,
}

impl Display for Locale {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		match self {
			| Locale::ar => write!(f, "العربية"),
			| Locale::bg => write!(f, "български"),
			| Locale::cs => write!(f, "Čeština"),
			| Locale::da => write!(f, "Dansk"),
			| Locale::de => write!(f, "Deutsch"),
			| Locale::el => write!(f, "Ελληνικά"),
			| Locale::en_GB => write!(f, "English, UK"),
			| Locale::en_US => write!(f, "English, US"),
			| Locale::es_ES => write!(f, "Español"),
			| Locale::es_419 => write!(f, "Español de América Latina"),
			| Locale::fi => write!(f, "Suomi"),
			| Locale::fr => write!(f, "Français"),
			| Locale::hi => write!(f, "हिन्दी"),
			| Locale::hr => write!(f, "Hrvatski"),
			| Locale::hu => write!(f, "Magyar"),
			| Locale::id => write!(f, "Bahasa Indonesia"),
			| Locale::it => write!(f, "Italiano"),
			| Locale::ja => write!(f, "日本語"),
			| Locale::ko => write!(f, "한국어"),
			| Locale::lt => write!(f, "Lietuviškai"),
			| Locale::nl => write!(f, "Nederlands"),
			| Locale::no => write!(f, "Norsk"),
			| Locale::pl => write!(f, "Polski"),
			| Locale::pt_BR => write!(f, "Português do Brasil"),
			| Locale::ro => write!(f, "Română"),
			| Locale::ru => write!(f, "Pусский"),
			| Locale::sv_SE => write!(f, "Svenska"),
			| Locale::th => write!(f, "ไทย"),
			| Locale::tr => write!(f, "Türkçe"),
			| Locale::uk => write!(f, "Українська"),
			| Locale::vi => write!(f, "Tiếng Việt"),
			| Locale::zh_CN => write!(f, "中文"),
			| Locale::zh_TW => write!(f, "繁體中文"),
		}
	}
}
