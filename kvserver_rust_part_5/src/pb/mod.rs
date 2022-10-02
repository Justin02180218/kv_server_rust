use std::error::Error;

use bytes::Bytes;

use crate::{
    cmd_request::ReqData, CmdRequest, CmdResponse, Get, Publish, Set, Subscribe, Unsubscribe,
};

pub mod cmd;

impl CmdRequest {
    // GET命令
    pub fn get(key: impl Into<String>) -> Self {
        Self {
            req_data: Some(ReqData::Get(Get { key: key.into() })),
        }
    }

    // SET命令
    pub fn set(key: impl Into<String>, value: Bytes) -> Self {
        Self {
            req_data: Some(ReqData::Set(Set {
                key: key.into(),
                value,
            })),
        }
    }

    // PUBLISH命令
    pub fn publish(topic: impl Into<String>, value: Bytes) -> Self {
        Self {
            req_data: Some(ReqData::Publish(Publish {
                topic: topic.into(),
                value,
            })),
        }
    }

    // 订阅命令
    pub fn subscribe(topic: impl Into<String>) -> Self {
        Self {
            req_data: Some(ReqData::Subscribe(Subscribe {
                topic: topic.into(),
            })),
        }
    }

    // 解除订阅命令
    pub fn unsubscribe(topic: impl Into<String>, id: u32) -> Self {
        Self {
            req_data: Some(ReqData::Unsubscribe(Unsubscribe {
                topic: topic.into(),
                id,
            })),
        }
    }
}

impl CmdResponse {
    pub fn new(status: u32, message: String, value: Bytes) -> Self {
        Self {
            status,
            message,
            value,
        }
    }
}

impl From<Bytes> for CmdResponse {
    fn from(v: Bytes) -> Self {
        Self {
            status: 200u32,
            message: "success".to_string(),
            value: v,
        }
    }
}

impl From<&str> for CmdResponse {
    fn from(s: &str) -> Self {
        Self {
            status: 400u32,
            message: s.to_string(),
            ..Default::default()
        }
    }
}

impl From<Box<dyn Error>> for CmdResponse {
    fn from(e: Box<dyn Error>) -> Self {
        Self {
            status: 500u32,
            message: e.to_string(),
            ..Default::default()
        }
    }
}
