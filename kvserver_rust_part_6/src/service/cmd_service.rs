use tracing::instrument;

use crate::{CmdResponse, CmdService, Get, Set};

// 为 GET 实现 execute
impl CmdService for Get {
    #[instrument(name = "service_cmd_get", skip_all)]
    fn execute(self, store: &impl crate::Storage) -> CmdResponse {
        // 从存储中获取数据，返回CmdResponse
        match store.get(&self.key) {
            Ok(Some(value)) => value.into(),
            Ok(None) => "Not found".into(),
            Err(e) => e.into(),
        }
    }
}

// 为 SET 实现 execute
impl CmdService for Set {
    // 存储数据
    #[instrument(name = "service_cmd_set", skip_all)]
    fn execute(self, store: &impl crate::Storage) -> CmdResponse {
        match store.set(&self.key, self.value) {
            Ok(Some(value)) => value.into(),
            Ok(None) => "Set fail".into(),
            Err(e) => e.into(),
        }
    }
}
