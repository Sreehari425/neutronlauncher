use clap::{Parser, ValueEnum};
use nl_core::{
    init::get_or_create_launcher_with_instances, 
    instance_util::{create_instance_file, list_instance_files, load_instance_config, remove_instance_file},
    launcher_log
};
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
            let (launcher_dir, instances_dir) = get_or_create_launcher_with_instances()?;
            launcher_log!("Launcher directory: {}", launcher_dir.display());
            
            // Create instance ID from name (replace spaces with underscores, convert to lowercase)
            let instance_id = name.to_lowercase().replace(' ', "_");
            
            let instance_type = match r#type {
                InstanceType::LinuxNative => "linux-native".to_string(),
                InstanceType::Wine => "wine".to_string(),
            };
            
            let desc = description.as_deref().unwrap_or("").to_string();
            
            // Create the individual instance file
            let instance_file_path = create_instance_file(
                &instances_dir,
                &instance_id,
                name.clone(),
                executable_path.clone(),
                instance_type.clone(),
                desc.clone(),
            )?;
            
            println!("Instance created successfully:");
            println!("  ID: {}", instance_id);
            println!("  Name: {}", name);
            println!("  Executable: {}", executable_path);
            println!("  Description: {}", desc);
            println!("  Type: {}", instance_type);
            println!("  Instance file: {}", instance_file_path.display());
        }
        Commands::ListInstances => {
            let (launcher_dir, instances_dir) = get_or_create_launcher_with_instances()?;
            launcher_log!("Launcher directory: {}", launcher_dir.display());
            
            let instance_files = list_instance_files(&instances_dir)?;
            
            if instance_files.is_empty() {
                launcher_log!("No instances found.");
            } else {
                println!("Found {} instance(s):", instance_files.len());
                for instance_file in instance_files {
                    match load_instance_config(&instance_file) {
                        Ok(config) => {
                            let instance_id = instance_file.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("unknown");
                            
                            println!("  ID: {}", instance_id);
                            println!("    Name: {}", config.instance_name);
                            println!("    Path: {}", config.instance_path);
                            println!("    Type: {}", config.instance_type);
                            if !config.description.is_empty() {
                                println!("    Description: {}", config.description);
                            }
                            println!();
                        }
                        Err(e) => {
                            launcher_log!("Error loading instance file {}: {}", instance_file.display(), e);
                        }
                    }
                }
            }
        }
        Commands::RemoveInstance { id } => {
            let (launcher_dir, instances_dir) = get_or_create_launcher_with_instances()?;
            launcher_log!("Launcher directory: {}", launcher_dir.display());
            
            remove_instance_file(&instances_dir, id)?;
        }
    }
    Ok(())
}
