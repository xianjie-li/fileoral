use clap::Parser;
use fileoral::{config::Config, server::server_entry::server_entry};

#[tokio::main]
async fn main() {
    let args = Config::parse();

    server_entry(&args).await;
}
