use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
//// Checks weather the user is new to Neutron Launcher
use crate::launcher_log;

pub fn is_new_user() -> bool {
    let Some(config_directory) = dirs::config_dir() else {
        return false;
    };
    let launcher_directory = config_directory.join("NeutronLauncher");
    !launcher_directory.exists()
}

//// Gets the launcher directory
fn get_launcher_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().ok_or("Config directory not found")?;
    Ok(config_dir.join("NeutronLauncher"))
}

pub fn get_or_create_launcher_dir() -> Result<PathBuf, Box<dyn Error>> {
    let config_dir = dirs::config_dir().ok_or("Config directory not found")?;
    let launcher_dir = config_dir.join("NeutronLauncher");

    if !launcher_dir.exists() {
        launcher_log!(
            "Config directory not found, creating at {}",
            launcher_dir.display()
        );
        fs::create_dir_all(&launcher_dir)?;
        launcher_log!("Launcher directory created.");
    } else {
        launcher_log!("Launcher directory exists at {}", launcher_dir.display());
    }

    Ok(launcher_dir)
}
