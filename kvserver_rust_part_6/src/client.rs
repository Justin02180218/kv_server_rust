use std::error::Error;

use bytes::BytesMut;
use clap::Parser;
use futures::{SinkExt, StreamExt};
use prost::Message;
use tokio::net::TcpStream;
use tokio_util::codec::{LengthDelimitedCodec, Framed};
use tracing::info;

use crate::{ClientArgs, CmdRequest, CmdResponse};


pub struct Client;

impl Client {
    pub async fn run(addr: String) -> Result<(), Box<dyn Error>> {
        let stream = TcpStream::connect(addr).await?;

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

        while let Some(Ok(buf)) = stream.next().await {
            let cmd_res = CmdResponse::decode(&buf[..]).unwrap();
            info!("Receive a response: {:?}", cmd_res);
        }

        Ok(())
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