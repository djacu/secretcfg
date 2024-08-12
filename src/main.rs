use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Interact with the master key
    Master(MasterArgs),
    /// Interact with sub keys
    Subkey(SubkeyArgs),
}

#[derive(Debug, Args)]
struct MasterArgs {
    #[command(subcommand)]
    command: MasterCommands,
}

#[derive(Debug, Subcommand)]
enum MasterCommands {
    /// Creates a master key
    Create,
    /// Extracts a master key from a backup
    Extract,
    /// Creates a backup of a master key
    Backup,
}

#[derive(Debug, Args)]
struct SubkeyArgs {
    #[command(subcommand)]
    command: SubkeyCommands,
}

#[derive(Debug, Subcommand)]
enum SubkeyCommands {
    /// Creates a sub key
    Create,
    /// Revokes a sub key
    Revoke,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Master(master) => {
            let master_cmd = &master.command;
            match master_cmd {
                MasterCommands::Create => {
                    println!("Master created!")
                }
                MasterCommands::Extract => {
                    println!("Master extracted!")
                }
                MasterCommands::Backup => {
                    println!("Master backed up!")
                }
            }
        }

        Commands::Subkey(subkey) => {
            let subkey_cmd = &subkey.command;
            match subkey_cmd {
                SubkeyCommands::Create => {
                    println!("Sub key created!");
                }
                SubkeyCommands::Revoke => {
                    println!("Sub key revoked!")
                }
            }
        }
    }
}
