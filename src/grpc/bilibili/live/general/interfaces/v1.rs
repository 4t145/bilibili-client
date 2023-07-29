///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOnlineRankReq {
    ///
    #[prost(int64, tag = "1")]
    pub ruid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub room_id: i64,
    ///
    #[prost(int64, tag = "3")]
    pub page: i64,
    ///
    #[prost(int64, tag = "4")]
    pub page_size: i64,
    ///
    #[prost(string, tag = "5")]
    pub platform: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOnlineRankResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<get_online_rank_resp::OnlineRankItem>,
    ///
    #[prost(int64, tag = "2")]
    pub online_num: i64,
}
/// Nested message and enum types in `GetOnlineRankResp`.
pub mod get_online_rank_resp {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OnlineRankItem {
        ///
        #[prost(int64, tag = "1")]
        pub uid: i64,
        ///
        #[prost(string, tag = "2")]
        pub uname: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "3")]
        pub face: ::prost::alloc::string::String,
        ///
        #[prost(int64, tag = "4")]
        pub continue_watch: i64,
        ///
        #[prost(message, optional, tag = "5")]
        pub medal_info: ::core::option::Option<super::MedalInfo>,
        ///
        #[prost(int64, tag = "6")]
        pub guard_level: i64,
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MedalInfo {
    ///
    #[prost(int64, tag = "1")]
    pub guard_level: i64,
    ///
    #[prost(int64, tag = "2")]
    pub medal_color_start: i64,
    ///
    #[prost(int64, tag = "3")]
    pub medal_color_end: i64,
    ///
    #[prost(int64, tag = "4")]
    pub medal_color_border: i64,
    ///
    #[prost(string, tag = "5")]
    pub medal_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub level: i64,
    ///
    #[prost(int64, tag = "7")]
    pub target_id: i64,
    ///
    #[prost(int64, tag = "8")]
    pub is_light: i64,
}
