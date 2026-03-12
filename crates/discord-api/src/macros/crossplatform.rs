/// Create the same constant with different values for different platforms, linxu and bsds are groups together
///
/// # Examples
///
/// ```rust
/// crossplatform_const!(
/// 	PLATFORM_NAME,
/// 	&str,
/// 	"linux/bsd",
/// 	"macos",
/// 	"windows",
/// 	"ios",
/// 	"android"
/// );
/// ```
macro_rules! crossplatform_const {
	($name:ident, $type:ty, $linux:expr, $macos:expr, $windows:expr, $ios:expr, $android:expr) => {
		#[cfg(any(
			target_os = "linux",
			target_os = "freebsd",
			target_os = "openbsd",
			target_os = "netbsd"
		))]
		pub const $name: $type = $linux;

		#[cfg(target_os = "macos")]
		pub const $name: $type = $macos;

		#[cfg(target_os = "windows")]
		pub const $name: $type = $windows;

		#[cfg(target_os = "ios")]
		pub const $name: $type = $ios;

		#[cfg(target_os = "android")]
		pub const $name: $type = $android;
	};
}
