use std::error::Error;

use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use kvserver_rust_part_2::{CmdRequest, CmdResponse, ServerConfig};
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

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client: {:?} connected", addr);

        tokio::spawn(async move {
            // 使用Frame的LengthDelimitedCodec进行编解码操作
            let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
            while let Some(Ok(mut buf)) = stream.next().await {
                // 对客户端发来的protobuf请求命令进行拆包
                let cmd_req = CmdRequest::decode(&buf[..]).unwrap();
                info!("Receive a command: {:?}", cmd_req);

                buf.clear();

                // 对protobuf的请求响应进行封包，然后发送给客户端。
                let cmd_res = CmdResponse::new(200, "success".to_string(), Bytes::default());
                cmd_res.encode(&mut buf).unwrap();
                stream.send(buf.freeze()).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
