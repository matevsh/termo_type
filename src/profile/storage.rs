use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use super::models::Profile;

/// Get the path to the profile file
/// Uses ~/.config/termotype/profile.json on Linux/Mac
/// Uses %APPDATA%/termotype/profile.json on Windows
fn get_profile_path() -> Result<PathBuf> {
    let config_dir = if cfg!(target_os = "windows") {
        // Windows: use APPDATA
        std::env::var("APPDATA")
            .context("Failed to get APPDATA environment variable")?
            .into()
    } else {
        // Linux/Mac: use ~/.config
        let home = std::env::var("HOME")
            .context("Failed to get HOME environment variable")?;
        PathBuf::from(home).join(".config")
    };

    let termotype_dir = config_dir.join("termotype");

    // Create directory if it doesn't exist
    if !termotype_dir.exists() {
        fs::create_dir_all(&termotype_dir)
            .context("Failed to create termotype config directory")?;
    }

    Ok(termotype_dir.join("profile.json"))
}

/// Save profile to disk
pub fn save_profile(profile: &Profile) -> Result<()> {
    let path = get_profile_path()?;

    let json = serde_json::to_string_pretty(profile)
        .context("Failed to serialize profile")?;

    fs::write(&path, json)
        .with_context(|| format!("Failed to write profile to {:?}", path))?;

    Ok(())
}

/// Load profile from disk
/// Returns a new empty profile if the file doesn't exist
pub fn load_profile() -> Result<Profile> {
    let path = get_profile_path()?;

    if !path.exists() {
        // Return empty profile if file doesn't exist
        return Ok(Profile::new());
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read profile from {:?}", path))?;

    let profile: Profile = serde_json::from_str(&content)
        .context("Failed to parse profile JSON")?;

    Ok(profile)
}

/// Get the profile file path (for display purposes)
pub fn get_profile_path_display() -> String {
    get_profile_path()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_path() {
        let path = get_profile_path();
        assert!(path.is_ok());
    }
}
