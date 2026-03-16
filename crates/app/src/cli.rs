use std::path::PathBuf;

use lexopt::{Arg, Parser};
use tracing::error;
use tracing::level_filters::LevelFilter;

pub struct CliArgs {
	pub log_level: Option<LevelFilter>,
	pub config_dir: Option<PathBuf>,
}

/// returns `None` if the full app should not be run
pub fn parse_args() -> Option<CliArgs> {
	let mut cli_args = CliArgs {
		log_level: None,
		config_dir: None,
	};

	let mut parser = Parser::from_env();
	while let Ok(Some(arg)) = parser.next() {
		match arg {
			| Arg::Long("help") | Arg::Short('h') => {
				print_help();
				return None;
			},
			| Arg::Long("log-level") => {
				if let Ok(Some(Arg::Value(l))) = parser.next() {
					let level = match l.to_string_lossy().to_lowercase().as_str() {
						| "error" => LevelFilter::ERROR,
						| "warn" => LevelFilter::WARN,
						| "info" => LevelFilter::INFO,
						| "debug" => LevelFilter::DEBUG,
						| "trace" => LevelFilter::TRACE,
						| level => {
							error!(
								"Unknown log level `{level}` (options are error, warn, info, debug, and trace)",
							);
							return None;
						},
					};
					cli_args.log_level = Some(level);
				} else {
					error!("--log-level flag must be supplied with a logging level!");
					return None;
				}
			},
			| Arg::Long("config-dir") => {
				if let Ok(Some(Arg::Value(d))) = parser.next() {
					cli_args.config_dir = Some(PathBuf::from(d));
				} else {
					error!("--config-dir flag must be supplied with a path!");
					return None;
				}
			},
			| _ => {},
		}
	}

	Some(cli_args)
}

fn print_help() {
	println!(
		r#"Cowcord v{}

Usage:
  -h --help:		prints this help message
  --log-level <LEVEL>:	override the log level (eg. error, warn, info). using trace will clog your terminal so dont do that :)
  --config-dir <PATH>:	override the config directory
"#,
		env!("CARGO_PKG_VERSION"),
	);
}
