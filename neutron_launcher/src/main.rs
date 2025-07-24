use clap::{Parser, ValueEnum};
use nl_core::{init::{get_or_create_launcher_dir, add_game_to_config, create_config_file, load_config, remove_game_from_config}, launcher_log};
#[derive(Parser, Debug)]
#[command(name = "neutron-launcher")]
#[command(about = "Neutron Launcher CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Create a new instance
    CreateInstance {
        /// Instance name
        #[arg(long)]
        name: String,

        /// Executable path
        #[arg(long, value_name = "PATH")]
        executable_path: String,

        /// Instance description
        #[arg(long)]
        description: Option<String>,

        /// Type of instance
        #[arg(long, value_enum)]
        r#type: InstanceType,
    },
    /// List all instances
    ListInstances,
    /// Remove an instance
    RemoveInstance {
        /// Instance ID to remove
        #[arg(long)]
        id: String,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum InstanceType {
    LinuxNative,
    Wine,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateInstance {
            name,
            executable_path,
            description,
            r#type,
        } => {
            let launcher_dir = get_or_create_launcher_dir()?;
            launcher_log!("Default launcher directory: {}", launcher_dir.display());
            
            // Create config file path
            let config_path = launcher_dir.join("config.json");
            
            // Ensure config file exists
            if !config_path.exists() {
                launcher_log!("Creating config file...");
                create_config_file(&config_path)?;
            }
            
            // Create instance ID from name (replace spaces with underscores, convert to lowercase)
            let instance_id = name.to_lowercase().replace(' ', "_");
            
            // Add the game instance to config
            add_game_to_config(
                &config_path,
                instance_id.clone(),
                name.clone(),
                executable_path.clone(),
            )?;
            
            println!("Instance created successfully:");
            println!("  ID: {}", instance_id);
            println!("  Name: {}", name);
            println!("  Executable: {}", executable_path);
            println!(
                "  Description: {}",
                description.as_deref().unwrap_or("None")
            );
            println!("  Type: {:?}", r#type);
            println!("  Config saved to: {}", config_path.display());
        }
        Commands::ListInstances => {
            let launcher_dir = get_or_create_launcher_dir()?;
            let config_path = launcher_dir.join("config.json");
            
            if !config_path.exists() {
                launcher_log!("No config file found. Create an instance first!");
                return Ok(());
            }
            
            match load_config(&config_path) {
                Ok(config) => {
                    if config.instances.is_empty() {
                        launcher_log!("No instances found.");
                    } else {
                        println!("Found {} instance(s):", config.instances.len());
                        for (id, instance) in &config.instances {
                            println!("  ID: {}", id);
                            println!("    Name: {}", instance.game_name);
                            println!("    Path: {}", instance.game_path);
                            println!();
                        }
                    }
                }
                Err(e) => {
                    launcher_log!("Error loading config: {}", e);
                }
            }
        }
        Commands::RemoveInstance { id } => {
            let launcher_dir = get_or_create_launcher_dir()?;
            let config_path = launcher_dir.join("config.json");
            
            if !config_path.exists() {
                launcher_log!("No config file found!");
                return Ok(());
            }
            
            remove_game_from_config(&config_path, id)?;
        }
    }
    Ok(())
}
