use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod commands;
mod config;

#[derive(Parser)]
#[command(name = "homelab")]
#[command(about = "CLI tool for managing Homelab NixOS configurations")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// API endpoint URL for the Homelab Lambda
    #[arg(long)]
    pub api_url: Option<String>,
    
    /// Configuration file path
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new server configuration
    Add {
        /// Server name
        #[arg(long)]
        server: String,
        /// Path to NixOS configuration file
        #[arg(long)]
        config_path: String,
        /// Server description
        #[arg(long)]
        description: Option<String>,
    },
    /// Update an existing server configuration
    Update {
        /// Server ID
        #[arg(long)]
        id: String,
        /// New path to NixOS configuration file
        #[arg(long)]
        config_path: Option<String>,
        /// New server description
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a server configuration
    Delete {
        /// Server ID
        #[arg(long)]
        id: String,
    },
    /// List all server configurations
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Load configuration
    let app_config = config::load_config(cli.config.as_deref()).await?;
    
    // Use API URL from command line, config file, or default
    let api_url = cli.api_url
        .or(app_config.api_url)
        .unwrap_or_else(|| "https://api.example.com".to_string());
    
    match cli.command {
        Commands::Add { server, config_path, description } => {
            commands::add_server::execute(&api_url, server, config_path, description).await?;
        }
        Commands::Update { id, config_path, description } => {
            commands::update_config::execute(&api_url, id, config_path, description).await?;
        }
        Commands::Delete { id } => {
            commands::delete_config::execute(&api_url, id).await?;
        }
        Commands::List => {
            commands::list_servers::execute(&api_url).await?;
        }
    }
    
    Ok(())
}