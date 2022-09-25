use std::error::Error;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use kvserver_rust_part_4::{rocksdb_storage::RocksDbStorage, CmdRequest, ServerConfig, Service};
use prost::Message;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let server_conf = ServerConfig::load("conf/server.conf")?;
    let listen_addr = server_conf.listen_address.addr;
    let rocksdb_path = server_conf.rocksdb_path.path;

    let listener = TcpListener::bind(&listen_addr).await?;
    info!("Listening on {} ......", listen_addr);

    // 初始化Service及存储
    let service = Service::new(RocksDbStorage::new(rocksdb_path));

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client: {:?} connected", addr);

        let svc = service.clone();

        tokio::spawn(async move {
            // 使用Frame的LengthDelimitedCodec进行编解码操作
            let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
            while let Some(Ok(mut buf)) = stream.next().await {
                // 对客户端发来的protobuf请求命令进行拆包
                let cmd_req = CmdRequest::decode(&buf[..]).unwrap();
                info!("Receive a command: {:?}", cmd_req);

                // 执行请求命令
                let cmd_res = svc.execute(cmd_req).await;

                buf.clear();

                // 对protobuf的请求响应进行封包，然后发送给客户端。
                cmd_res.encode(&mut buf).unwrap();
                stream.send(buf.freeze()).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
