use std::error::Error;

use anyhow::Result;
use kvserver_rust_part_1::ClientConfig;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_conf = ClientConfig::load("conf/client.conf")?;
    let connect_addr = client_conf.connect_address.server_addr;

    let mut stream = TcpStream::connect(&connect_addr).await?;

    let n = stream.write(b"Hello, world!").await?;
    println!("Send info successed！n = {n}");

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await.expect("从Socket读取数据失败！");
    println!("Receive info：{}, n = {n}", String::from_utf8(buf).unwrap());

    Ok(())
}
