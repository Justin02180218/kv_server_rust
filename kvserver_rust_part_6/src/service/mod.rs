use std::sync::Arc;

use tracing::{info, instrument};

use crate::{
    cmd_request::ReqData, rocksdb_storage::RocksDbStorage, CmdRequest, CmdResponse, Storage,
};

pub mod cmd_service;

pub trait CmdService {
    // 解析命令，返回Response
    fn execute(self, store: &impl Storage) -> CmdResponse;
}

// 设置默认存储为RocksDB
pub struct Service<S = RocksDbStorage> {
    store_svc: Arc<StoreService<S>>,
}

// 在多线程中进行clone
pub struct StoreService<Store> {
    store: Store,
    on_recv_req: Vec<fn(&CmdRequest)>,
    on_exec_req: Vec<fn(&CmdResponse)>,
    on_before_res: Vec<fn(&mut CmdResponse)>,
}

impl<Store: Storage> StoreService<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            on_recv_req: Vec::new(),
            on_exec_req: Vec::new(),
            on_before_res: Vec::new(),
        }
    }

    // 注册收到命令时的通知函数
    pub fn regist_recv_req(mut self, f: fn(&CmdRequest)) -> Self {
        self.on_recv_req.push(f);
        self
    }
    // 注册执行命令时的通知函数
    pub fn regist_exec_req(mut self, f: fn(&CmdResponse)) -> Self {
        self.on_exec_req.push(f);
        self
    }
    // 注册返回结果前的通知函数
    pub fn regist_before_res(mut self, f: fn(&mut CmdResponse)) -> Self {
        self.on_before_res.push(f);
        self
    }

    // 执行注册的通知函数
    pub async fn notify_recv_req(&self, cmd_req: &CmdRequest) {
        self.on_recv_req.iter().for_each(|f| f(cmd_req))
    }
    pub async fn notify_exec_req(&self, cmd_res: &CmdResponse) {
        self.on_exec_req.iter().for_each(|f| f(cmd_res))
    }
    pub async fn notify_before_res(&self, cmd_res: &mut CmdResponse) {
        self.on_before_res.iter().for_each(|f| f(cmd_res))
    }
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            store_svc: Arc::new(StoreService::new(store)),
        }
    }

    // 执行命令
    #[instrument(name = "service_execute", skip_all)]
    pub async fn execute(&self, cmd_req: CmdRequest) -> CmdResponse {
        info!("Receive command request: {:?}", cmd_req);
        self.store_svc.notify_recv_req(&cmd_req).await;

        let mut cmd_res = process_cmd(cmd_req, &self.store_svc.store).await;

        info!("Execute command, response: {:?}", cmd_res);
        self.store_svc.notify_exec_req(&cmd_res).await;

        info!("Response CmdResponse before");
        self.store_svc.notify_before_res(&mut cmd_res).await;

        cmd_res
    }
}

// 实现Clone trait
impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            store_svc: self.store_svc.clone(),
        }
    }
}

// 处理请求命令，返回Response
#[instrument(name = "service_process_cmd", skip_all)]
async fn process_cmd(cmd_req: CmdRequest, store: &impl Storage) -> CmdResponse {
    match cmd_req.req_data {
        // 处理 GET 命令
        Some(ReqData::Get(cmd_get)) => cmd_get.execute(store),
        // 处理 SET 命令
        Some(ReqData::Set(cmd_set)) => cmd_set.execute(store),
        _ => "Invalid command".into(),
    }
}

// 从 StoreService<Store> 转换为 Service<Store>
impl<Store: Storage> From<StoreService<Store>> for Service<Store> {
    fn from(store: StoreService<Store>) -> Self {
        Self {
            store_svc: Arc::new(store),
        }
    }
}
