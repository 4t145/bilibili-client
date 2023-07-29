///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sync {
    /// 笔记id
    #[prost(int64, tag = "1")]
    pub note_id: i64,
    /// 唯一标示
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
}
