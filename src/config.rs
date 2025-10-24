use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "s3fs")]
#[command(about = "S3 server with filesystem backend")]
pub struct Config {
    /// Configuration file path
    #[arg(short, long)]
    pub config_file: Option<PathBuf>,

    /// Host to bind to
    #[arg(short = 'H', long, default_value = "127.0.0.1", env = "S3FS_HOST")]
    pub host: String,

    /// Port to bind to
    #[arg(short, long, default_value = "9001", env = "S3FS_PORT")]
    pub port: u16,

    /// Root directory for file storage
    #[arg(short, long, default_value = "./s3-data", env = "S3FS_ROOT")]
    pub root: PathBuf,

    /// S3 Access Key ID
    #[arg(short, long, default_value = "admin", env = "S3FS_ACCESS_KEY")]
    pub access_key: String,

    /// S3 Secret Access Key
    #[arg(short, long, default_value = "admin", env = "S3FS_SECRET_KEY")]
    pub secret_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    #[serde(default)]
    pub host: Option<String>,

    #[serde(default)]
    pub port: Option<u16>,

    #[serde(default)]
    pub root: Option<PathBuf>,

    pub access_key: Option<String>,
    pub secret_key: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Config::parse();

        // If config file is specified, merge its values
        if let Some(config_path) = &config.config_file {
            tracing::info!("Loading configuration from: {}", config_path.display());
            let content = std::fs::read_to_string(config_path)?;
            let file_config: ConfigFile = serde_json::from_str(&content)?;

            // Override CLI/env values with file values only if they exist in file
            if let Some(host) = file_config.host {
                config.host = host;
            }
            if let Some(port) = file_config.port {
                config.port = port;
            }
            if let Some(root) = file_config.root {
                config.root = root;
            }
            if let Some(access_key) = file_config.access_key {
                config.access_key = access_key;
            }
            if let Some(secret_key) = file_config.secret_key {
                config.secret_key = secret_key;
            }

            tracing::info!("Configuration loaded and merged successfully");
        }

        Ok(config)
    }

    pub fn socket_addr(&self) -> Result<std::net::SocketAddr, Box<dyn std::error::Error>> {
        Ok(format!("{}:{}", self.host, self.port).parse()?)
    }
}
