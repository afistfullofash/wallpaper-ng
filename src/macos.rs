use crate::unix::{get_stdout, run};
use crate::{Mode, Result};

#[cfg(feature = "from_url")]
use crate::download_image;

pub fn supports_get() -> bool {
    true
}

pub fn supports_mode() -> bool {
    false
}

pub fn supports_set() -> bool {
    true
}

#[cfg(feature = "from_url")]
pub fn supports_url() -> bool {
    true
}

#[cfg(not(feature = "from_url"))]
pub fn supports_url() -> bool {
    false
}

/// Returns the current wallpaper.
pub fn get() -> Result<String> {
    get_stdout(
        "osascript",
        &[
            "-e",
            r#"tell application "Finder" to get POSIX path of (get desktop picture as alias)"#,
        ],
    )
}

// Sets the wallpaper from a file.
pub fn set_from_path(path: &str) -> Result<()> {
    run(
        "osascript",
        &[
            "-e",
            &format!(
                r#"tell application "System Events" to tell every desktop to set picture to {}"#,
                enquote::enquote('"', path),
            ),
        ],
    )
}

#[cfg(feature = "from_url")]
// Sets the wallpaper from a URL.
pub fn set_from_url(url: &str) -> Result<()> {
    let path = download_image(url)?;
    set_from_path(&path)
}

/// No-op. Unable to change with AppleScript.
pub fn set_mode(_: Mode) -> Result<()> {
    Err("unsupported on macos".into())
}
