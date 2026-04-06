use Result;

pub fn supports_get() -> bool {
    false
}

pub fn supports_mode() -> bool {
    false
}

pub fn supports_set() -> bool {
    false
}

pub fn supports_url() -> bool {
    false
}

pub fn get() -> Result<String> {
    Err("unsupported operating system".into())
}

pub fn set_from_path(_: &str) -> Result<()> {
    Err("unsupported operating system".into())
}

#[cfg(feature = "from_url")]
pub fn set_from_url(_: &str) -> Result<()> {
    Err("unsupported operating system".into())
}

pub fn set_mode(_: Mode) -> Result<()> {
    Err("unsupported operating system".into())
}
