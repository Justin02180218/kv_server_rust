use std::error::Error;

use anyhow::Result;
use kvserver_rust_part_1::ServerConfig;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_conf = ServerConfig::load("conf/server.conf")?;
    let listen_addr = server_conf.listen_address.addr;

    let listener = TcpListener::bind(&listen_addr).await?;
    println!("Listening on {} ......", listen_addr);

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("Client: {:?} connected", addr);

        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];

            loop {
                let n = stream.read(&mut buf).await.expect("从Socket读取数据失败！");

                if n == 0 {
                    return;
                }

                stream
                    .write_all(&buf[0..n])
                    .await
                    .expect("向Socket写入数据失败！");
            }
        });
    }
}
