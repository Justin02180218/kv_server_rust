use std::error::Error;

use anyhow::Result;
use kvserver_rust_part_5::{
    rocksdb_storage::RocksDbStorage, Server, ServerConfig, Service, StoreService,
};
use tokio::signal;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let server_conf = ServerConfig::load("conf/server.conf")?;
    let listen_addr = server_conf.listen_address.addr;
    let rocksdb_path = server_conf.rocksdb_path.path;

    // 初始化Service及存储
    let service: Service = StoreService::new(RocksDbStorage::new(rocksdb_path))
        .regist_recv_req(|req| debug!("[DEBUG] Receive req: {:?}", req))
        .regist_exec_req(|res| debug!("[DEBUG] Execute req: {:?}", res))
        .regist_before_res(|res| debug!("[DEBUG] Before res {:?}", res))
        .into();

    let server = Server::new(listen_addr, service);
    // 监听ctrl+c信号
    server.run(signal::ctrl_c()).await
}
