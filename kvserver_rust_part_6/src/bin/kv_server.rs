use std::error::Error;

use anyhow::Result;
use kvserver_rust_part_6::{
    rocksdb_storage::RocksDbStorage, Server, ServerConfig, Service, StoreService
};
use tokio::signal;
use tracing::{debug, span};
use tracing_subscriber::{layer::SubscriberExt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let server_conf = ServerConfig::load("conf/server.conf")?;
    let listen_addr = server_conf.listen_address.addr;
    let rocksdb_path = server_conf.rocksdb_path.path;
    let max_conns = server_conf.connects.max_conns;

    let tracer = opentelemetry_jaeger::new_pipeline().with_service_name("kv_server").install_simple()?;
    let tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_layer).init(); 
    
    let root = span!(tracing::Level::INFO, "app_start", work_units = 2); 
    let _enter = root.enter();

    // 初始化Service及存储
    let service: Service = StoreService::new(RocksDbStorage::new(rocksdb_path))
        .regist_recv_req(|req| debug!("[DEBUG] Receive req: {:?}", req))
        .regist_exec_req(|res| debug!("[DEBUG] Execute req: {:?}", res))
        .regist_before_res(|res| debug!("[DEBUG] Before res {:?}", res))
        .into();

    let server = Server::new(listen_addr, service, max_conns);
    // 监听ctrl+c信号
    server.run(signal::ctrl_c()).await
}
