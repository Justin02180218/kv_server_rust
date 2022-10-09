use std::{error::Error, sync::Arc};

use futures::{Future, SinkExt, StreamExt};
use prost::Message;
use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc, Semaphore},
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{error, info, instrument};

use crate::{CmdRequest, Service};

pub struct Server {
    listen_addr: String,        // 服务器监听地址
    service: Service,           // 业务逻辑service
    max_conns: Arc<Semaphore>,  // 最大连接数
}

impl Server {
    pub fn new(listen_addr: String, service: Service, max_conns: usize) -> Self {
        Self {
            listen_addr,
            service,
            max_conns: Arc::new(Semaphore::new(max_conns)),
        }
    }

    // 监听 SIGINT 信号
    #[instrument(name = "server_run", skip_all)]
    pub async fn run(&self, shutdown: impl Future) -> Result<(), Box<dyn Error>> {
        // 广播channel，用于给各子线程发送关闭信息
        let (notify_shutdown, _) = broadcast::channel(1);
        // mpsc channel，用于通知主线程，各子线程执行完成。
        let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel::<()>(1);
        
        tokio::select! {
            res = self.execute(&notify_shutdown, &shutdown_complete_tx) => {
                if let Err(err) = res {
                    error!(cause = %err, "failed to accept");
                }
            },
            // 接收Ctrl+c SIGINT
            _ = shutdown => {
                info!("KV Server is shutting down!!!");
            }
        }
        
        drop(notify_shutdown);
        drop(shutdown_complete_tx);

        let _ = shutdown_complete_rx.recv().await;

        Ok(())
    }

    // 与客户端建立连接
    #[instrument(name = "server_execute", skip_all)]
    async fn execute(&self, notify_shutdown: &broadcast::Sender<()>, shutdown_complete_tx: &mpsc::Sender<()>) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(&self.listen_addr).await?;
        info!("Listening on {} ......", self.listen_addr);

        loop {
            let permit = self.max_conns.clone().acquire_owned().await.unwrap();

            let (stream, addr) = listener.accept().await?;
            info!("Client: {:?} connected", addr);

            let svc = self.service.clone();
            let mut shutdown = notify_shutdown.subscribe();
            let shutdown_complete = shutdown_complete_tx.clone();
            
            tokio::spawn(async move {
                // 使用Frame的LengthDelimitedCodec进行编解码操作
                let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
                drop(permit);

                loop {
                    let mut buf = tokio::select! {
                        Some(Ok(buf)) = stream.next() => {
                            buf
                        },
                        // 接收boardcast的关闭信息
                        _ = shutdown.recv() => {
                            // 清理工作
                            info!("Process resource release before shutdown ......");
                            // 通知主线程处理完成
                            let _ = shutdown_complete.send(());
                            info!("Process resource release completed ......");
                            return;
                        }
                    };
                    
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
            });
        }
    }
}
