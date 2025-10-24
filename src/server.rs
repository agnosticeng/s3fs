use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use s3s::auth::SimpleAuth;
use s3s::service::S3ServiceBuilder;
use s3s_fs::FileSystem;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{debug, info, warn};

use crate::config::Config;

pub struct S3Server {
    config: Config,
}

impl S3Server {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create root directory if it doesn't exist
        tokio::fs::create_dir_all(&self.config.root).await?;
        info!("Using root directory: {}", self.config.root.display());

        // Create filesystem backend
        let fs = FileSystem::new(self.config.root.clone())
            .map_err(|e| format!("Failed to create filesystem: {:?}", e))?;

        info!("FileSystem backend created successfully");

        // Create auth and S3 service
        let auth = SimpleAuth::from_single(
            self.config.access_key.clone(),
            self.config.secret_key.clone(),
        );

        let mut builder = S3ServiceBuilder::new(fs);
        builder.set_auth(auth);
        let service = builder.build().into_shared();

        info!("S3 service built successfully");

        let addr = self.config.socket_addr()?;
        let listener = TcpListener::bind(addr).await?;

        self.log_server_info(&addr);

        // Accept connections
        loop {
            let (stream, peer_addr) = match listener.accept().await {
                Ok(conn) => conn,
                Err(e) => {
                    warn!("Failed to accept connection: {}", e);
                    continue;
                }
            };

            let service_clone = service.clone();

            tokio::spawn(async move {
                debug!("New connection from {}", peer_addr);
                let io = TokioIo::new(stream);

                if let Err(e) = http1::Builder::new()
                    .serve_connection(io, service_clone)
                    .await
                {
                    warn!("Error serving connection from {}: {}", peer_addr, e);
                } else {
                    debug!("Connection from {} closed successfully", peer_addr);
                }
            });
        }
    }

    fn log_server_info(&self, addr: &SocketAddr) {
        info!("S3 server starting on http://{}", addr);
        info!("You can now use this server with S3 clients by setting:");
        info!("  - Endpoint: http://{}", addr);
        info!("  - Access Key: {}", self.config.access_key);
        info!("  - Secret Key: {}", self.config.secret_key);
        info!("  - Region: us-east-1 (or any region)");
        info!("  - Force path style: true (for some clients like ClickHouse)");
        info!("Authentication: Simple auth configured with specified credentials");
    }
}
