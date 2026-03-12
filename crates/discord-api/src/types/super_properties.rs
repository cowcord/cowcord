use std::env;
#[cfg(all(unix, not(target_os = "macos")))]
use std::fs;
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
use const_format::formatc;
use cowcord_macros::crossplatform_const;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::locale::Locale;

/// Due to the nature of client properties, the structure of this object is not well-defined, and no field is truly required.
/// Additionally, types of fields are not verified, and all documented enums are merely conventions.
/// Fields are marked as required if it's observed that they are sent in all official client properties.
///
/// will fully type fields as they get used but until then if its set to `None` in `ClientProperties::new()` then its probably just being ignored
#[derive(Serialize, Deserialize)]
pub struct ClientProperties {
	/// The operating system of the client
	pub os: OperatingSystemType,
	/// The operating system version (kernel version for Linux, SDK version for Android)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub os_version: Option<String>,
	/// The operating system SDK version
	#[serde(skip_serializing_if = "Option::is_none")]
	pub os_sdk_version: Option<String>,
	/// The architecture of the operating system
	#[serde(skip_serializing_if = "Option::is_none")]
	pub os_arch: Option<String>,
	/// The architecture of the desktop application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub app_arch: Option<String>,
	/// The browser the client is using
	pub browser: BrowserType,
	/// The user-agent of the client's browser, may be blank on mobile clients
	pub browser_user_agent: String,
	/// The version of the client's browser, may be blank on mobile clients
	pub browser_version: String,
	/// The build number of the client
	pub client_build_number: u32,
	/// The native metadata version of the desktop client, if using the new update system
	#[serde(skip_serializing_if = "Option::is_none")]
	pub native_build_number: Option<Option<u32>>,
	/// The mobile client version
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_version: Option<String>,
	/// The alternate event source this request originated from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_event_source: Option<Option<ClientEventSource>>,
	/// The focus state of the client
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_app_state: Option<ClientAppState>,
	/// A client-generated UUID used to identify the client launch
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_launch_id: Option<Uuid>,
	/// A client-generated UUID representing the current persisted analytics heartbeat, regenerated every 30 minutes
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_heartbeat_session_id: Option<Option<Uuid>>,
	/// The total CPU utilization of the mobile device (in percent)
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_performance_cpu: Option<u32>,
	/// The total memory utilization of the mobile device (in kilobytes)
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_performance_memory: Option<u32>,
	/// The number of CPU cores available to the mobile device
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cpu_core_count: Option<u8>,
	/// The release channel of the client
	pub release_channel: ReleaseChannel,
	/// The primary system locale
	pub system_locale: Locale,
	/// The model of the mobile device the client is running on
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<String>,
	/// A unique identifier for the mobile device (UUID on Android, IdentifierForVendor on iOS)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_vendor_id: Option<String>,
	/// The advertiser ID of the mobile device
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_advertiser_id: Option<String>,
	/// The design ID of the client
	#[serde(skip_serializing_if = "Option::is_none")]
	pub design_id: Option<u32>,
	/// Whether accessibility support is enabled
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub accessibility_support_enabled: Option<bool>,
	/// The accessibility features enabled on the client
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub accessibility_features: Option<u32>,
	/// The Linux window manager
	#[serde(skip_serializing_if = "Option::is_none")]
	pub window_manager: Option<String>,
	/// The Linux distribution (output of lsb_release -ds )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub distro: Option<String>,
	/// The Linux runtime environment
	#[serde(skip_serializing_if = "Option::is_none")]
	pub runtime_environment: Option<RuntimeEnvironment>,
	/// The Linux display server
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_server: Option<String>,
	/// The URL that originally referred the user to Discord
	#[serde(skip_serializing_if = "Option::is_none")]
	pub referrer: Option<String>,
	/// Same as referrer but for the current session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub referrer_current: Option<String>,
	/// The domain of the URL that originally referred the user to Discord
	#[serde(skip_serializing_if = "Option::is_none")]
	pub referring_domain: Option<String>,
	/// Same as referring_domain but for the current session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub referring_domain_current: Option<String>,
	/// The search engine that originally referred the user to Discord, parsed from the referrer URL
	#[serde(skip_serializing_if = "Option::is_none")]
	pub search_engine: Option<String>,
	/// Same as search_engine but for the current session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub search_engine_current: Option<String>,
	/// The search engine query that originally referred the user to Discord, parsed from the q or p parameter of the referrer URL
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mp_keyword: Option<String>,
	/// Same as mp_keyword but for the current session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mp_keyword_current: Option<String>,
	/// The UTM campaign that originally referred the user to Discord, parsed from the utm_campaign parameter of the referrer URL
	#[serde(skip_serializing_if = "Option::is_none")]
	pub utm_campaign: Option<String>,
	/// Same as utm_campaign but for the current session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub utm_campaign_current: Option<String>,
	/// The UTM content that originally referred the user to Discord, parsed from the utm_content parameter of the referrer URL
	#[serde(skip_serializing_if = "Option::is_none")]
	pub utm_content: Option<String>,
	/// Same as utm_content but for the current session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub utm_content_current: Option<String>,
	/// The UTM medium that originally referred the user to Discord, parsed from the utm_medium parameter of the referrer URL
	#[serde(skip_serializing_if = "Option::is_none")]
	pub utm_medium: Option<String>,
	/// Same as utm_medium but for the current session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub utm_medium_current: Option<String>,
	/// The UTM source that originally referred the user to Discord, parsed from the utm_source parameter of the referrer URL
	#[serde(skip_serializing_if = "Option::is_none")]
	pub utm_source: Option<String>,
	/// Same as utm_source but for the current session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub utm_source_current: Option<String>,
	/// The UTM term that originally referred the user to Discord, parsed from the utm_term parameter of the referrer URL
	#[serde(skip_serializing_if = "Option::is_none")]
	pub utm_term: Option<String>,
	/// Same as utm_term but for the current session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub utm_term_current: Option<String>,
	/// Whether the connecting client has modifications (e.g. BetterDiscord)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub has_client_mods: Option<bool>,
	/// The launch signature of the client
	#[serde(skip_serializing_if = "Option::is_none")]
	pub launch_signature: Option<Uuid>,
	/// The client's existing installation ID (see the experiments documentation for more information)
	///
	/// only sent when identifying with the Gateway and are not included in the `X-Super-Properties` header.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub installation_id: Option<String>,
	/// Whether the Gateway session sent the identified using fast connect (a preload script ran before the client was loaded)
	///
	/// only sent when identifying with the Gateway and are not included in the `X-Super-Properties` header.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_fast_connect: Option<bool>,
	/// The version of the client properties protocol
	///
	/// only sent when identifying with the Gateway and are not included in the `X-Super-Properties` header.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub version: Option<String>,
}

impl ClientProperties {
	pub fn new() -> Self {
		let os_arch = OS_ARCH.map(|i| i.to_owned());

		#[allow(deprecated)]
		Self {
			os: OperatingSystemType::default(),
			os_version: os_version(),
			os_sdk_version: os_sdk_version(),
			app_arch: os_arch.clone(),
			os_arch,

			window_manager: os_window_manager(),
			distro: os_distro(),
			display_server: os_display_server(),
			runtime_environment: Some(RuntimeEnvironment::Native),

			cpu_core_count: None,
			client_performance_cpu: None,
			client_performance_memory: None,

			browser: BrowserType::default(),
			browser_user_agent: BROWSER_USER_AGENT.to_owned(),
			browser_version: BROWSER_VERSION.to_owned(),

			release_channel: ReleaseChannel::Stable,

			client_app_state: Some(ClientAppState::Focused),
			client_build_number: DISCORD_BUILD_NUMBER,
			native_build_number: None,
			client_version: Some(CLIENT_VERSION.to_owned()),
			client_event_source: Some(None),

			client_heartbeat_session_id: Some(client_heartbeat_session_id()),
			client_launch_id: Some(*CLIENT_LAUNCH_ID.get_or_init(|| Uuid::new_v4())),
			launch_signature: Some(*LAUNCH_SIGNATURE.get_or_init(|| gen_launch_signature())),
			has_client_mods: Some(false), // https://i.ytimg.com/vi/bIsp1K8eJG0/mqdefault.jpg

			system_locale: Locale::en_US,
			accessibility_features: None,
			accessibility_support_enabled: None,

			device: None,
			device_vendor_id: None,
			device_advertiser_id: None,
			design_id: None,

			referrer: None,
			referrer_current: None,
			referring_domain: None,
			referring_domain_current: None,

			search_engine: None,
			search_engine_current: None,

			mp_keyword: None,
			mp_keyword_current: None,

			utm_campaign: None,
			utm_campaign_current: None,
			utm_content: None,
			utm_content_current: None,
			utm_medium: None,
			utm_medium_current: None,
			utm_source: None,
			utm_source_current: None,
			utm_term: None,
			utm_term_current: None,

			installation_id: None,
			is_fast_connect: None,
			version: None,
		}
	}
}

#[derive(Serialize, Deserialize, Default)]
pub enum OperatingSystemType {
	/// The client is running on Android
	#[cfg_attr(target_os = "android", default)]
	Android,
	/// The client is running on BlackBerry OS
	BlackBerry,
	/// The client is running on macOS
	#[cfg_attr(target_os = "macos", default)]
	#[serde(rename = "Mac OS X")]
	MacOsX,
	/// The client is running on iOS
	iOS,
	/// The client is running on a Linux distribution
	#[cfg_attr(target_os = "linux", default)]
	Linux,
	/// The client is running on Windows Mobile
	#[serde(rename = "Windows Mobile")]
	WindowsMobile,
	/// The client is running on Windows
	#[cfg_attr(target_os = "windows", default)]
	Windows,
	/// The client is running on PlayStation
	Playstation,
	/// The client is running on Xbox
	Xbox,
	/// The client is running on an unknown OS
	#[cfg_attr(
		not(any(
			target_os = "android",
			target_os = "macos",
			target_os = "linux",
			target_os = "windows"
		)),
		default
	)]
	Unknown,
}

/// get os version (kernel version for macos, linux, and bsds)
// todo: ios/android
pub fn os_version() -> Option<String> {
	#[cfg(any(
		target_os = "linux",
		target_os = "macos",
		target_os = "freebsd",
		target_os = "openbsd",
		target_os = "netbsd",
	))]
	return Some(
		String::from_utf8_lossy(
			&Command::new("uname")
				.arg("-r")
				.output()
				.expect("failed to run `uname` for system os version")
				.stdout,
		)
		.trim()
		.to_owned(),
	);

	#[cfg(windows)]
	return Some(
		String::from_utf8_lossy(
			&Command::new("cmd")
				.args(["/c", "ver"])
				.output()
				.expect("failed to run `ver` for system os version")
				.stdout,
		)
		.split("[Version ")
		.nth(1)
		.expect("failed to parse version from `ver` comamnd output")
		.trim_end_matches("]")
		.to_owned(),
	);

	#[cfg(not(any(windows, unix)))]
	return None;
}

/// get the os sdk version
pub fn os_sdk_version() -> Option<String> {
	#[cfg(target_os = "macos")]
	const NTH: usize = 0; // uses major kernel version on mac
	#[cfg(target_os = "windows")]
	const NTH: usize = 2; // uses the 3rd thingy in the os version on windows (eg. 10.0.26100 -> 26100)

	#[cfg(any(target_os = "macos", target_os = "windows"))]
	return os_version()
		.map(|v| v.split('.').nth(NTH).map(|i| i.to_owned()))
		.flatten();

	#[cfg(not(any(target_os = "macos", target_os = "windows")))]
	return None;
}

#[cfg(target_arch = "x86_64")]
pub const OS_ARCH: Option<&str> = Some("x64");
#[cfg(target_arch = "x86")]
pub const OS_ARCH: Option<&str> = Some("x86");
#[cfg(target_arch = "aarch64")]
pub const OS_ARCH: Option<&str> = Some("arm64");
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
pub const OS_ARCH: Option<&str> = None;

/// returns `None` if not linux/bsd or `PRETTY_NAME` was not found in `/etc/os-release`
pub fn os_distro() -> Option<String> {
	#[cfg(all(unix, not(target_os = "macos")))]
	{
		let os_release = fs::read_to_string("/etc/os-release")
			.expect("error occured whilea ttempting to read os release file");

		return os_release
			.split('\n')
			.filter(|l| l.starts_with("PRETTY_NAME="))
			.nth(0)
			.map(|d| d.replace("PRETTY_NAME=", "").replace("\"", ""));
	}
	#[cfg(not(all(unix, not(target_os = "macos"))))]
	return None;
}

/// returns `None` if not linux/bsd
pub fn os_window_manager() -> Option<String> {
	#[cfg(all(unix, not(target_os = "macos")))]
	{
		let xdg_desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or("unknown".to_owned());
		let gdm_session = env::var("GDMSESSION").unwrap_or("unknown".to_owned());
		return Some(format!("{xdg_desktop},{gdm_session}"));
	}
	#[cfg(not(all(unix, not(target_os = "macos"))))]
	return None;
}

/// returns `None` if not linux/bsd
pub fn os_display_server() -> Option<String> {
	#[cfg(all(unix, not(target_os = "macos")))]
	return Some(env::var("XDG_SESSION_TYPE").unwrap_or("unknown".to_owned()));
	#[cfg(not(all(unix, not(target_os = "macos"))))]
	return None;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeEnvironment {
	/// Client is running natively
	Native,
	/// Client is running in Flatpak
	Flatpak,
	/// Client is running in Snap
	Snap,
	/// Client is running in AppImage
	AppImage,
}

#[derive(Serialize, Deserialize, Default)]
pub enum BrowserType {
	/// Desktop client
	#[cfg_attr(not(any(target_os = "android", target_os = "ios")), default)]
	#[serde(rename = "Discord Client")]
	DiscordClient,
	/// Android client
	#[cfg_attr(target_os = "android", default)]
	#[serde(rename = "Discord Android")]
	DiscordAndroid,
	/// iOS client
	#[cfg_attr(target_os = "ios", default)]
	#[serde(rename = "Discord iOS")]
	DiscordiOS,
	/// Embedded (e.g. Xbox) client
	#[serde(rename = "Discord Embedded")]
	DiscordEmbedded,
	/// VR (e.g. Meta Quest) client
	#[serde(rename = "Discord VR")]
	DiscordVR,
	/// Google Chrome Android
	#[serde(rename = "Android Chrome")]
	AndroidChrome,
	/// Generic Android browser
	#[serde(rename = "Android Mobile")]
	AndroidMobile,
	/// BlackBerry browser
	BlackBerry,
	/// Google Chrome desktop
	Chrome,
	/// Google Chrome iOS
	#[serde(rename = "Chrome iOS")]
	ChromeiOS,
	/// Facebook mobile browser
	#[serde(rename = "Facebook Mobile")]
	Facebook,
	/// Mozilla Firefox
	Firefox,
	/// Microsoft Internet Explorer
	#[serde(rename = "Internet Explorer")]
	InternetExplorer,
	/// KDE Konqueror
	Konqueror,
	/// Safari iOS
	#[serde(rename = "Mobile Safari")]
	MobileSafari,
	/// Generic Mozilla-like browser
	Mozilla,
	/// Opera
	Opera,
	/// Opera Mini
	#[serde(rename = "Opera Mini")]
	OperaMini,
	/// Safari desktop
	Safari,
}

pub const ELECTRON_VERSION: &str = "37.6.0";
pub const WEBKIT_VERSION: &str = "537.36";
pub const CHROMIUM_VERSION: &str = "140.0.0.0";

#[cfg(any(windows, unix))]
pub const BROWSER_VERSION: &str = ELECTRON_VERSION;
#[cfg(any(target_family = "wasm"))]
pub const BROWSER_VERSION: &str = CHROMIUM_VERSION;

crossplatform_const!(
	DISCORD_BUILD_NUMBER,
	u32,
	509303,
	485097,
	397417,
	91102,
	4025
);

crossplatform_const!(
	BROWSER_USER_AGENT,
	&str,
	formatc!(
		"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/{WEBKIT_VERSION} (KHTML, like Gecko) discord/{CLIENT_VERSION} Chrome/{CHROMIUM_VERSION} Electron/{ELECTRON_VERSION} Safari/{WEBKIT_VERSION}"
	),
	formatc!(
		"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/{WEBKIT_VERSION} (KHTML, like Gecko) discord/{CLIENT_VERSION} Chrome/{CHROMIUM_VERSION} Electron/{ELECTRON_VERSION} Safari/{WEBKIT_VERSION}"
	),
	formatc!(
		"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/{WEBKIT_VERSION} (KHTML, like Gecko) discord/{CLIENT_VERSION} Chrome/{CHROMIUM_VERSION} Electron/{ELECTRON_VERSION} Safari/{WEBKIT_VERSION}"
	),
	"Discord/{DISCORD_BUILD_NUMBER} CFNetwork/1494.0.7 Darwin/23.4.0",
	"Discord-Android/280202;RNA"
);

crossplatform_const!(
	CLIENT_VERSION,
	&str,
	"0.0.125",
	"0.0.372",
	"1.0.328",
	"310.3",
	"280.2 - rn"
);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientAppState {
	/// The app is active (mobile only)
	Active,
	/// The app is inactive (mobile only)
	Inactive,
	/// The app has been backgrounded (mobile only)
	Background,
	/// The client is focused (in the foreground)
	#[cfg(not(target_os = "ios"))]
	Focused,
	/// The client is unfocused (in the background)
	#[cfg(not(target_os = "ios"))]
	Unfocused,
}

#[derive(Serialize, Deserialize)]
pub enum ClientEventSource {
	/// The request originated from the Discord Overlay
	OVERLAY,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReleaseChannel {
	/// Stable
	Stable,
	/// PTB
	Ptb,
	/// Canary
	Canary,
	/// Staging
	Staging,
	/// Internal
	Internal,
	/// Google Play Store stable
	GoogleRelease,
	/// Android billing (unknown)
	BillingRelease,
	/// Android beta
	BetaRelease,
	/// Android alpha
	CanaryRelease,
	/// Internal employee-only release
	InternalRelease,
	/// Internal developer release
	DeveloperRelease,
	/// iOS ad-hoc release
	AdhocRelease,
	/// Not applicable release (unknown)
	#[serde(rename = "N/A")]
	NA,
	/// Unknown release
	Unknown,
}

// todo: take bitflags input param?
/// generates a random uuidv4 from a base set of bits
///
/// note: mobile clients just use the current unix timestamp, not this
pub fn gen_launch_signature() -> Uuid {
	let bits = 0b00000000100000000001000000010000000010000001000000001000000000000010000010000001000000000100000000000001000000000000100000000000u128;
	let uuid = Uuid::new_v4().as_u128() & (!bits & u128::MAX);
	return Uuid::from_u128(uuid);
}

pub static CLIENT_LAUNCH_ID: OnceLock<Uuid> = OnceLock::new();
pub static LAUNCH_SIGNATURE: OnceLock<Uuid> = OnceLock::new();

pub struct HeartbeatSession {
	pub last_generated: Option<Instant>,
	pub cached: Option<Uuid>,
}

pub static HEARTBEAT_SESSION: Mutex<HeartbeatSession> = Mutex::new(HeartbeatSession {
	last_generated: None,
	cached: None,
});

pub fn client_heartbeat_session_id() -> Option<Uuid> {
	let mut state = HEARTBEAT_SESSION.lock().ok()?;

	let needs_refresh = match state.last_generated {
		| None => true,
		| Some(last) => last.elapsed() > Duration::from_secs(30 * 60),
	};

	if needs_refresh {
		state.last_generated = Some(Instant::now());
		state.cached = Some(Uuid::new_v4());
	}

	state.cached
}
