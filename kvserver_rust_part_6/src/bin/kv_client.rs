use std::error::Error;

use anyhow::Result;
use kvserver_rust_part_6::{ClientConfig, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let client_conf = ClientConfig::load("conf/client.conf")?;
    let connect_addr = client_conf.connect_address.server_addr;

    Client::run(connect_addr).await?;

    Ok(())
}
