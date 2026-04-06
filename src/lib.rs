//! This library gets and sets the desktop wallpaper/background.
//!
//! The supported desktops are:
//! * Windows
//! * macOS
//! * GNOME
//! * KDE
//! * Cinnamon
//! * Unity
//! * Budgie
//! * XFCE
//! * LXDE
//! * MATE
//! * Deepin
//! * Most Wayland compositors (set only, requires swaybg)
//! * i3 (set only, requires feh)
//!
//! # Example
//! ```
//!  if wallpaper_bce::supports_get() {
//!    println!("{:?}", wallpaper_bce::get());
//!  }
//!
//!  if wallpaper_bce::supports_set() {
//!    let path = "./tests/wallpapers/rust-logo.png";
//!    wallpaper_bce::set_from_path(path).unwrap();
//!    if wallpaper_bce::supports_get() {
//!       assert!(wallpaper_bce::get().unwrap() == path, "Wallpaper was not set correctly")
//!    }
//!  }
//!
//!  if wallpaper_bce::supports_mode() {
//!    wallpaper_bce::set_mode(wallpaper_bce::Mode::Crop).unwrap();
//!  }
//!
//!  if wallpaper_bce::supports_get() {
//!    println!("{:?}", wallpaper_bce::get());
//!  }
//! ```

mod error;
pub use error::Error;

#[cfg(unix)]
mod unix;

#[cfg(all(unix, not(target_os = "macos")))]
mod linux;

#[cfg(all(unix, not(target_os = "macos")))]
use crate::linux as platform;

// macos
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos as platform;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
use windows as platform;

// unsupported
#[cfg(not(any(unix, windows)))]
mod unsupported;

#[cfg(not(any(unix, windows)))]
use unsupported as platform;

// from_url feature
#[cfg(feature = "from_url")]
mod from_url;

#[cfg(feature = "from_url")]
pub(crate) use from_url::download_image;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug)]
pub enum Mode {
    Center,
    Crop,
    Fit,
    Span,
    Stretch,
    Tile,
}

/// Returns whether the current target/platform implementation can read wallpapers.
pub fn supports_get() -> bool {
    platform::supports_get()
}

/// Returns whether the current target/platform implementation can set wallpaper mode.
pub fn supports_mode() -> bool {
    platform::supports_mode()
}

/// Returns whether the current target/platform implementation can set wallpapers.
pub fn supports_set() -> bool {
    platform::supports_set()
}

/// Returns whether URL-based wallpaper setting is available.
pub fn supports_url() -> bool {
    platform::supports_url()
}

/// Returns the current wallpaper path.
pub fn get() -> Result<String> {
    platform::get()
}

/// Sets the wallpaper from a local file path.
pub fn set_from_path(path: &str) -> Result<()> {
    platform::set_from_path(path)
}

#[cfg(feature = "from_url")]
/// Sets the wallpaper from a URL.
pub fn set_from_url(url: &str) -> Result<()> {
    platform::set_from_url(url)
}

/// Sets the wallpaper display mode.
pub fn set_mode(mode: Mode) -> Result<()> {
    platform::set_mode(mode)
}
