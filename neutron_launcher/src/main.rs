use clap::{Parser, ValueEnum};

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
}

#[derive(ValueEnum, Clone, Debug)]
enum InstanceType {
    LinuxNative,
    Wine,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateInstance {
            name,
            executable_path,
            description,
            r#type,
        } => {
            println!("Creating instance:");
            println!("  Name: {}", name);
            println!("  Executable: {}", executable_path);
            println!(
                "  Description: {}",
                description.as_deref().unwrap_or("None")
            );
            println!("  Type: {:?}", r#type);
        }
    }
}
