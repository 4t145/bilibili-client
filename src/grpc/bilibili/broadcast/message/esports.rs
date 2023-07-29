#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notify {
    /// cid
    #[prost(int64, tag = "1")]
    pub cid: i64,
}
