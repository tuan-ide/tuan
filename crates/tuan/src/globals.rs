use std::sync::LazyLock;

#[cfg(target_os = "macos")]
pub static CONFIG_DIR: LazyLock<String> = LazyLock::new(|| {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| "/".to_string());
    option_env!("TUAN_CONFIG_DIR")
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}/Library/Application Support/tuan-editor", home))
});

#[cfg(target_os = "linux")]
pub static CONFIG_DIR: LazyLock<&'static str> = LazyLock::new(|| {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| "/".to_string());
    option_env!("TUAN_CONFIG_DIR")
        .unwrap_or_else(|| format!("{}/.config/tuan-editor", home).as_str())
});

#[cfg(target_os = "windows")]
pub static CONFIG_DIR: LazyLock<&'static str> =
    LazyLock::new(|| option_env!("TUAN_CONFIG_DIR").unwrap_or(r"%APPDATA%\tuan-editor"));
