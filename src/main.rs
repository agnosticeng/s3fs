mod config;
mod logging;
mod server;

use config::Config;
use server::S3Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_tracing()?;

    let config = Config::load()?;

    let server = S3Server::new(config);
    server.start().await
}
