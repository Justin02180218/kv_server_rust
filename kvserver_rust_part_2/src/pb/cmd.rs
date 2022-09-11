/// 命令请求
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmdRequest {
    #[prost(oneof="cmd_request::ReqData", tags="1, 2, 3, 4, 5")]
    pub req_data: ::core::option::Option<cmd_request::ReqData>,
}
/// Nested message and enum types in `CmdRequest`.
pub mod cmd_request {
    #[derive(PartialOrd)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ReqData {
        #[prost(message, tag="1")]
        Get(super::Get),
        #[prost(message, tag="2")]
        Set(super::Set),
        #[prost(message, tag="3")]
        Publish(super::Publish),
        #[prost(message, tag="4")]
        Subscribe(super::Subscribe),
        #[prost(message, tag="5")]
        Unsubscribe(super::Unsubscribe),
    }
}
/// 服务器的响应
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmdResponse {
    #[prost(uint32, tag="1")]
    pub status: u32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(bytes="bytes", tag="3")]
    pub value: ::prost::bytes::Bytes,
}
/// 请求值命令
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
/// 存储值命令
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Set {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes="bytes", tag="2")]
    pub value: ::prost::bytes::Bytes,
    #[prost(uint32, tag="3")]
    pub expire: u32,
}
/// 向Topic发布值命令
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Publish {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(bytes="bytes", tag="2")]
    pub value: ::prost::bytes::Bytes,
}
/// 订阅Topic命令
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscribe {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
}
/// 取消订阅命令
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unsubscribe {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub id: u32,
}
