use std::sync::Arc;

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
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            store_svc: Arc::new(StoreService { store }),
        }
    }

    // 执行命令
    pub async fn execute(&self, cmd_req: CmdRequest) -> CmdResponse {
        println!("=== Execute Command Before ===");
        let cmd_res = process_cmd(cmd_req, &self.store_svc.store).await;
        println!("=== Execute Command After ===");
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
async fn process_cmd(cmd_req: CmdRequest, store: &impl Storage) -> CmdResponse {
    match cmd_req.req_data {
        // 处理 GET 命令
        Some(ReqData::Get(cmd_get)) => cmd_get.execute(store),
        // 处理 SET 命令
        Some(ReqData::Set(cmd_set)) => cmd_set.execute(store),
        _ => "Invalid command".into(),
    }
}
