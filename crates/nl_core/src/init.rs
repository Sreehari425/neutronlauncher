use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use crate::{error, launcher_log, Config, GameInstance};

/// Represents a game configuration

pub fn is_new_user() -> bool {
    let Some(config_directory) = dirs::config_dir() else {
        return false;
    };
    let launcher_directory = config_directory.join("NeutronLauncher");
    !launcher_directory.exists()
}

// Todo : use anyhow
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

/// Creates a config.json file with the basic structure
pub fn create_config_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let json_content = serde_json::to_string_pretty(&config)?;
    
    // Create directory if it doesn't exist
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(path, json_content)?;
    launcher_log!("Config file created successfully!");
    Ok(())
}

/// Loads the config from a JSON file
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

/// Saves the config to a JSON file
pub fn save_config<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = serde_json::to_string_pretty(config)?;
    fs::write(path, json_content)?;
    Ok(())
}

/// Adds a new game instance to the config file
pub fn add_game_to_config<P: AsRef<Path>>(
    config_path: P,
    instance_id: String,
    game_name: String,
    game_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load existing config or create new one if file doesn't exist
    let mut config = match load_config(&config_path) {
        Ok(config) => config,
        Err(_) => {
            launcher_log!("Config file not found, creating new one...");
            Config::new()
        }
    };

    // Create new game instance
    let game_instance = GameInstance {
        game_name: game_name.clone(),
        game_path: game_path.clone(),
    };

    // Check if instance ID already exists
    if config.instances.contains_key(&instance_id) {
        launcher_log!("Instance '{}' already exists, updating...", instance_id);
    } else {
        launcher_log!("Adding new game instance: '{}'", instance_id);
    }

    // Add or update the game instance
    config.instances.insert(instance_id.clone(), game_instance);

    // Save the updated config
    save_config(&config, config_path)?;
    launcher_log!("Game '{}' added successfully as '{}'!", game_name, instance_id);
    
    Ok(())
}

/// Removes a game instance from the config file
pub fn remove_game_from_config<P: AsRef<Path>>(
    config_path: P,
    instance_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config(&config_path)?;
    
    match config.instances.remove(instance_id) {
        Some(removed_game) => {
            save_config(&config, config_path)?;
            launcher_log!("Game '{}' (ID: '{}') removed successfully!", removed_game.game_name, instance_id);
        }
        None => {
            launcher_log!("Game instance '{}' not found in config!", instance_id);
        }
    }
    
    Ok(())
}
