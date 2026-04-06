mod gnome;
mod kde;
mod lxde;
pub(crate) mod xfce;

use crate::unix::{get_stdout, run};
use crate::{Error, Mode, Result};
use std::{env, path::Path};
use which::which;

#[cfg(feature = "from_url")]
use crate::download_image;

/// The desktops which are supported
enum Desktops {
    Gnome,
    Kde,
    Cinnamon,
    Mate,
    Xfce,
    Lxde,
    Deepin,
    /// Generic Wayland support (swaybg)
    Wayland,
    /// Generic X11 Support (feh)
    X11,
    Unsupported,
}

fn get_desktop() -> Desktops {
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    if gnome::is_compliant(&desktop) {
        return Desktops::Gnome;
    }

    match desktop.as_str() {
        "KDE" => Desktops::Kde,
        "X-Cinnamon" => Desktops::Cinnamon,
        "MATE" => Desktops::Mate,
        "XFCE" => Desktops::Xfce,
        "LXDE" => Desktops::Lxde,
        "Deepin" => Desktops::Deepin,
        _ => {
            if which("swaybg").is_ok() {
                Desktops::Wayland
            } else if which("feh").is_ok() {
                Desktops::X11
            } else {
                Desktops::Unsupported
            }
        }
    }
}

pub fn supports_mode() -> bool {
    match get_desktop() {
        Desktops::Gnome => true,
        Desktops::Kde => true,
        Desktops::Cinnamon => true,
        Desktops::Mate => true,
        Desktops::Xfce => true,
        Desktops::Lxde => true,
        Desktops::Deepin => true,
        Desktops::Wayland => false,
        Desktops::X11 => false,
        Desktops::Unsupported => false,
    }
}

pub fn supports_set() -> bool {
    match get_desktop() {
        Desktops::Gnome => true,
        Desktops::Kde => true,
        Desktops::Cinnamon => true,
        Desktops::Mate => true,
        Desktops::Xfce => true,
        Desktops::Lxde => true,
        Desktops::Deepin => true,
        Desktops::Wayland => true,
        Desktops::X11 => true,
        Desktops::Unsupported => false,
    }
}

pub fn supports_get() -> bool {
    match get_desktop() {
        Desktops::Gnome => true,
        Desktops::Kde => true,
        Desktops::Cinnamon => true,
        Desktops::Mate => true,
        Desktops::Xfce => true,
        Desktops::Lxde => true,
        Desktops::Deepin => true,
        Desktops::Wayland => false,
        Desktops::X11 => false,
        Desktops::Unsupported => false,
    }
}

#[cfg(feature = "from_url")]
pub fn supports_url() -> bool {
    true
}

#[cfg(not(feature = "from_url"))]
pub fn supports_url() -> bool {
    false
}

/// Returns the wallpaper of the current desktop.
pub fn get() -> Result<String> {
    match get_desktop() {
        Desktops::Gnome => gnome::get(),
        Desktops::Kde => kde::get(),
        Desktops::Cinnamon => parse_dconf(
            "gsettings",
            &["get", "org.cinnamon.desktop.background", "picture-uri"],
        ),
        Desktops::Mate => parse_dconf(
            "dconf",
            &["read", "/org/mate/desktop/background/picture-filename"],
        ),
        Desktops::Xfce => xfce::get(),
        Desktops::Lxde => lxde::get(),
        Desktops::Deepin => parse_dconf(
            "dconf",
            &[
                "read",
                "/com/deepin/wrap/gnome/desktop/background/picture-uri",
            ],
        ),
        Desktops::Wayland => Err(Error::UnsupportedDesktop),
        Desktops::X11 => Err(Error::UnsupportedDesktop),
        Desktops::Unsupported => Err(Error::UnsupportedDesktop),
    }
}

/// Sets the wallpaper for the current desktop from a file path.
pub fn set_from_path<P>(path: P) -> Result<()>
where
    P: AsRef<Path> + std::fmt::Display,
{
    match get_desktop() {
        Desktops::Gnome => gnome::set(&path),
        Desktops::Kde => kde::set(&path),
        Desktops::Cinnamon => run(
            "gsettings",
            &[
                "set",
                "org.cinnamon.desktop.background",
                "picture-uri",
                &enquote::enquote('"', &format!("file://{}", &path)),
            ],
        ),
        Desktops::Mate => run(
            "dconf",
            &[
                "write",
                "/org/mate/desktop/background/picture-filename",
                &enquote::enquote('"', path.as_ref().to_str().ok_or(Error::InvalidPath)?),
            ],
        ),
        Desktops::Xfce => xfce::set(path),
        Desktops::Lxde => lxde::set(path),
        Desktops::Deepin => run(
            "dconf",
            &[
                "write",
                "/com/deepin/wrap/gnome/desktop/background/picture-uri",
                &enquote::enquote('"', &format!("file://{}", &path)),
            ],
        ),
        Desktops::Wayland => run(
            "swaybg",
            &["-i", path.as_ref().to_str().ok_or(Error::InvalidPath)?],
        ),
        Desktops::X11 => run(
            "feh",
            &[
                "--bg-fill",
                path.as_ref().to_str().ok_or(Error::InvalidPath)?,
            ],
        ),
        Desktops::Unsupported => Err(Error::UnsupportedDesktop),
    }
}

#[cfg(feature = "from_url")]
/// Sets the wallpaper for the current desktop from a URL.
pub fn set_from_url(url: &str) -> Result<()> {
    let path = download_image(url)?;
    set_from_path(&path)
}

/// Sets the wallpaper style.
pub fn set_mode(mode: Mode) -> Result<()> {
    match get_desktop() {
        Desktops::Gnome => gnome::set_mode(mode),
        Desktops::Kde => kde::set_mode(mode),
        Desktops::Cinnamon => run(
            "gsettings",
            &[
                "set",
                "org.cinnamon.desktop.background",
                "picture-options",
                &mode.get_gnome_string(),
            ],
        ),
        Desktops::Mate => run(
            "dconf",
            &[
                "write",
                "/org/mate/desktop/background/picture-options",
                &mode.get_gnome_string(),
            ],
        ),
        Desktops::Xfce => xfce::set_mode(mode),
        Desktops::Lxde => lxde::set_mode(mode),
        Desktops::Deepin => run(
            "dconf",
            &[
                "write",
                "/com/deepin/wrap/gnome/desktop/background/picture-options",
                &mode.get_gnome_string(),
            ],
        ),
        Desktops::Wayland => Err(Error::UnsupportedDesktop),
        Desktops::X11 => Err(Error::UnsupportedDesktop),
        Desktops::Unsupported => Err(Error::UnsupportedDesktop),
    }
}

fn parse_dconf(command: &str, args: &[&str]) -> Result<String> {
    let mut stdout = enquote::unquote(&get_stdout(command, args)?)?;
    // removes file protocol
    if stdout.starts_with("file://") {
        stdout = stdout[7..].into();
    }
    Ok(stdout)
}
