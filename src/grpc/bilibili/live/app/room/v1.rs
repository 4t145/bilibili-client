///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStudioListReq {
    ///
    #[prost(int64, tag = "1")]
    pub room_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStudioListResp {
    ///
    #[prost(int64, tag = "1")]
    pub status: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub master_list: ::prost::alloc::vec::Vec<get_studio_list_resp::StudioMaster>,
}
/// Nested message and enum types in `GetStudioListResp`.
pub mod get_studio_list_resp {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Pendants {
        ///
        #[prost(message, optional, tag = "1")]
        pub frame: ::core::option::Option<Pendant>,
        ///
        #[prost(message, optional, tag = "2")]
        pub badge: ::core::option::Option<Pendant>,
    }
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Pendant {
        ///
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        ///
        #[prost(int64, tag = "2")]
        pub position: i64,
        ///
        #[prost(string, tag = "3")]
        pub value: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "4")]
        pub desc: ::prost::alloc::string::String,
    }
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StudioMaster {
        ///
        #[prost(int64, tag = "1")]
        pub uid: i64,
        ///
        #[prost(int64, tag = "2")]
        pub room_id: i64,
        ///
        #[prost(string, tag = "3")]
        pub uname: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "4")]
        pub face: ::prost::alloc::string::String,
        ///
        #[prost(message, optional, tag = "5")]
        pub pendants: ::core::option::Option<Pendants>,
        ///
        #[prost(string, tag = "6")]
        pub tag: ::prost::alloc::string::String,
        ///
        #[prost(int64, tag = "7")]
        pub tag_type: i64,
    }
}
