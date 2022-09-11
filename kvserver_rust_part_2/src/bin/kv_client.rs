use std::error::Error;

use anyhow::Result;
use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use kvserver_rust_part_2::{ClientConfig, CmdRequest, CmdResponse};
use prost::Message;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let client_conf = ClientConfig::load("conf/client.conf")?;
    let connect_addr = client_conf.connect_address.server_addr;

    let stream = TcpStream::connect(&connect_addr).await?;

    // 使用Frame的LengthDelimitedCodec进行编解码操作
    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
    let mut buf = BytesMut::new();

    // 创建GET命令
    let cmd_get = CmdRequest::get("mykey");
    cmd_get.encode(&mut buf).unwrap();

    // 发送GET命令
    stream.send(buf.freeze()).await.unwrap();
    info!("Send info successed！");

    // 接收服务器返回的响应
    while let Some(Ok(buf)) = stream.next().await {
        let cmd_res = CmdResponse::decode(&buf[..]).unwrap();
        info!("Receive a response: {:?}", cmd_res);
    }

    Ok(())
}
