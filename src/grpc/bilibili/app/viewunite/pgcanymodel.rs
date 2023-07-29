///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewPgcAny {
    ///
    #[prost(uint64, tag = "1")]
    pub season_id: u64,
    ///
    #[prost(int32, tag = "2")]
    pub season_type: i32,
}
