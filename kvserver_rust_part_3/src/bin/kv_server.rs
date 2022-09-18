use std::{error::Error, sync::Arc};

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use kvserver_rust_part_3::{CmdRequest, CmdResponse, ServerConfig, cmd_request::ReqData, Get, Set, mem_storage::MemStorage, Storage};
use prost::Message;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let server_conf = ServerConfig::load("conf/server.conf")?;
    let listen_addr = server_conf.listen_address.addr;

    let listener = TcpListener::bind(&listen_addr).await?;
    info!("Listening on {} ......", listen_addr);

    // 初始化内存存储
    let storage = Arc::new(MemStorage::new());

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client: {:?} connected", addr);

        let stor = storage.clone();

        tokio::spawn(async move {
            // 使用Frame的LengthDelimitedCodec进行编解码操作
            let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
            while let Some(Ok(mut buf)) = stream.next().await {
                // 对客户端发来的protobuf请求命令进行拆包
                let cmd_req = CmdRequest::decode(&buf[..]).unwrap();
                info!("Receive a command: {:?}", cmd_req);
                
                // 处理请求命令
                let cmd_res = process_cmd(cmd_req, &stor).await.unwrap();

                buf.clear();

                // 对protobuf的请求响应进行封包，然后发送给客户端。
                // let cmd_res = CmdResponse::new(200, "success".to_string(), Bytes::default());
                cmd_res.encode(&mut buf).unwrap();
                stream.send(buf.freeze()).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}

// 处理请求命令，返回Response
async fn process_cmd(req: CmdRequest, storage: &MemStorage) -> Result<CmdResponse, Box<dyn Error>> {
    match req {
        // 处理 GET 命令
        CmdRequest{
            req_data:Some(ReqData::Get(Get {key})),
        } => {
            let value = storage.get(&key)?;
            Ok(CmdResponse::new(200, "get success".to_string(), value.unwrap_or_default()))
        }, 
        // 处理 SET 命令
        CmdRequest{
            req_data:Some(ReqData::Set(Set {key, value})),
        } => {
            let value = storage.set(&key, value)?;
            Ok(CmdResponse::new(200, "set success".to_string(), value.unwrap_or_default()))
        }, 
        _ => Err("Invalid command".into())
    }
}