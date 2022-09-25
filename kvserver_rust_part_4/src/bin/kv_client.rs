use std::error::Error;

use anyhow::Result;
use bytes::BytesMut;
use clap::Parser;
use futures::{SinkExt, StreamExt};
use kvserver_rust_part_4::{ClientArgs, ClientConfig, CmdRequest, CmdResponse};
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

    let client_args = ClientArgs::parse();

    // 解析命令行参数，生成命令
    let cmd = process_args(client_args).await?;
    // 命令编码
    cmd.encode(&mut buf).unwrap();
    // 发送命令
    stream.send(buf.freeze()).await.unwrap();
    info!("Send command successed！");

    loop {
        tokio::select! {
            Some(Ok(buf)) = stream.next() => {
                let cmd_res = CmdResponse::decode(&buf[..]).unwrap();
                info!("Receive a response: {:?}", cmd_res);
            }
        }
    }
}

// 生成CmdRequest命令
async fn process_args(client_args: ClientArgs) -> Result<CmdRequest, Box<dyn Error>> {
    match client_args {
        // 生成 GET 命令
        ClientArgs::Get { key } => Ok(CmdRequest::get(key)),
        // 生成 SET 命令
        ClientArgs::Set { key, value } => Ok(CmdRequest::set(key, value.into())),
        // 生成 PUBLISH 命令
        ClientArgs::Publish { topic, value } => Ok(CmdRequest::publish(topic, value.into())),
        // 生成 SUBSCRIBE 命令
        ClientArgs::Subscribe { topic } => Ok(CmdRequest::subscribe(topic)),
        // // 生成 UNSUBSCRIBE 命令
        ClientArgs::Unsubscribe { topic, id } => Ok(CmdRequest::unsubscribe(topic, id)),
    }
}
