#![allow(non_camel_case_types)]

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Locale {
	ar,
	bg,
	cs,
	da,
	de,
	el,
	en_GB,
	en_US,
	es_ES,
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
	pt_BR,
	ro,
	ru,
	sv_SE,
	th,
	tr,
	uk,
	vi,
	zh_CN,
	zh_TW,
}

impl fmt::Display for Locale {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			| Locale::ar => write!(f, "Arabic"),
			| Locale::bg => write!(f, "Bulgarian"),
			| Locale::cs => write!(f, "Czech"),
			| Locale::da => write!(f, "Danish"),
			| Locale::de => write!(f, "German"),
			| Locale::el => write!(f, "Greek"),
			| Locale::en_GB => write!(f, "English"),
			| Locale::en_US => write!(f, "English"),
			| Locale::es_ES => write!(f, "Spanish"),
			| Locale::es_419 => write!(f, "Spanish"),
			| Locale::fi => write!(f, "Finnish"),
			| Locale::fr => write!(f, "French"),
			| Locale::hi => write!(f, "Hindi"),
			| Locale::hr => write!(f, "Croatian"),
			| Locale::hu => write!(f, "Hungarian"),
			| Locale::id => write!(f, "Indonesian"),
			| Locale::it => write!(f, "Italian"),
			| Locale::ja => write!(f, "Japanese"),
			| Locale::ko => write!(f, "Korean"),
			| Locale::lt => write!(f, "Lithuanian"),
			| Locale::nl => write!(f, "Dutch"),
			| Locale::no => write!(f, "Norwegian"),
			| Locale::pl => write!(f, "Polish"),
			| Locale::pt_BR => write!(f, "Portuguese"),
			| Locale::ro => write!(f, "Romanian"),
			| Locale::ru => write!(f, "Russian"),
			| Locale::sv_SE => write!(f, "Swedish"),
			| Locale::th => write!(f, "Thai"),
			| Locale::tr => write!(f, "Turkish"),
			| Locale::uk => write!(f, "Ukrainian"),
			| Locale::vi => write!(f, "Vietnamese"),
			| Locale::zh_CN => write!(f, "Chinese"),
			| Locale::zh_TW => write!(f, "Chinese"),
		}
	}
}
