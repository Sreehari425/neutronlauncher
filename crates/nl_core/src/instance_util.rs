use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::launcher_log;

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceConfig {
    pub instance_name: String,
    pub instance_path: String,
    pub instance_type: String,
    pub description: String,
}

impl InstanceConfig {
    pub fn new(name: String, path: String, instance_type: String, description: String) -> Self {
        InstanceConfig {
            instance_name: name,
            instance_path: path,
            instance_type,
            description,
        }
    }
}

/// Creates an instances directory inside the launcher directory
pub fn create_instances_dir<P: AsRef<Path>>(launcher_dir: P) -> Result<PathBuf, Box<dyn Error>> {
    let instances_dir = launcher_dir.as_ref().join("instances");
    
    if !instances_dir.exists() {
        launcher_log!("Creating instances directory at {}", instances_dir.display());
        fs::create_dir_all(&instances_dir)?;
    } else {
        launcher_log!("Instances directory exists at {}", instances_dir.display());
    }
    
    Ok(instances_dir)
}

/// Creates an individual instance JSON file
pub fn create_instance_file<P: AsRef<Path>>(
    instances_dir: P,
    instance_id: &str,
    instance_name: String,
    instance_path: String,
    instance_type: String,
    description: String,
) -> Result<PathBuf, Box<dyn Error>> {
    let instance_config = InstanceConfig::new(instance_name, instance_path, instance_type, description);
    let json_content = serde_json::to_string_pretty(&instance_config)?;
    
    let instance_file_path = instances_dir.as_ref().join(format!("{}.json", instance_id));
    
    fs::write(&instance_file_path, json_content)?;
    launcher_log!("Instance file created at {}", instance_file_path.display());
    
    Ok(instance_file_path)
}

/// Loads an instance configuration from a JSON file
pub fn load_instance_config<P: AsRef<Path>>(instance_file_path: P) -> Result<InstanceConfig, Box<dyn Error>> {
    let content = fs::read_to_string(instance_file_path)?;
    let config: InstanceConfig = serde_json::from_str(&content)?;
    Ok(config)
}

/// Lists all instance files in the instances directory
pub fn list_instance_files<P: AsRef<Path>>(instances_dir: P) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut instance_files = Vec::new();
    
    if instances_dir.as_ref().exists() {
        for entry in fs::read_dir(instances_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                instance_files.push(path);
            }
        }
    }
    
    Ok(instance_files)
}

/// Removes an instance file
pub fn remove_instance_file<P: AsRef<Path>>(instances_dir: P, instance_id: &str) -> Result<(), Box<dyn Error>> {
    let instance_file_path = instances_dir.as_ref().join(format!("{}.json", instance_id));
    
    if instance_file_path.exists() {
        fs::remove_file(&instance_file_path)?;
        launcher_log!("Instance file '{}' removed successfully!", instance_file_path.display());
    } else {
        launcher_log!("Instance file for '{}' not found!", instance_id);
    }
    
    Ok(())
}
