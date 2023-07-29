///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoReply {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialReply {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub uid: i64,
    ///
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub scheme: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub rcmd: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub ios_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub android_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub button: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub deleted: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub mtime: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialRequest {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoMall {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub night_img: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub is_activated: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoMallListReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<PhotoMall>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoMallListReq {
    ///
    #[prost(string, tag = "1")]
    pub mobiapp: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(string, tag = "3")]
    pub device: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivacyReply {
    ///
    #[prost(map = "string, int64", tag = "1")]
    pub privacy: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivacyRequest {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTopPhotoReq {
    ///
    #[prost(string, tag = "1")]
    pub mobiapp: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub i_d: i64,
    ///
    #[prost(int64, tag = "3")]
    pub mid: i64,
    ///
    #[prost(int32, tag = "4")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpaceSettingReply {
    ///
    #[prost(int64, tag = "1")]
    pub channel: i64,
    ///
    #[prost(int64, tag = "2")]
    pub fav_video: i64,
    ///
    #[prost(int64, tag = "3")]
    pub coins_video: i64,
    ///
    #[prost(int64, tag = "4")]
    pub likes_video: i64,
    ///
    #[prost(int64, tag = "5")]
    pub bangumi: i64,
    ///
    #[prost(int64, tag = "6")]
    pub played_game: i64,
    ///
    #[prost(int64, tag = "7")]
    pub groups: i64,
    ///
    #[prost(int64, tag = "8")]
    pub comic: i64,
    ///
    #[prost(int64, tag = "9")]
    pub b_bq: i64,
    ///
    #[prost(int64, tag = "10")]
    pub dress_up: i64,
    ///
    #[prost(int64, tag = "11")]
    pub disable_following: i64,
    ///
    #[prost(int64, tag = "12")]
    pub live_playback: i64,
    ///
    #[prost(int64, tag = "13")]
    pub close_space_medal: i64,
    ///
    #[prost(int64, tag = "14")]
    pub only_show_wearing: i64,
    ///
    #[prost(int64, tag = "15")]
    pub disable_show_school: i64,
    ///
    #[prost(int64, tag = "16")]
    pub disable_show_nft: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpaceSettingReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopPhoto {
    ///
    #[prost(string, tag = "1")]
    pub img_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub night_img_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub sid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopPhotoArc {
    ///
    #[prost(bool, tag = "1")]
    pub show: bool,
    ///
    #[prost(int64, tag = "2")]
    pub aid: i64,
    ///
    #[prost(string, tag = "3")]
    pub pic: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopPhotoArcCancelReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopPhotoReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub top_photo: ::core::option::Option<TopPhoto>,
    ///
    #[prost(message, optional, tag = "2")]
    pub top_photo_arc: ::core::option::Option<TopPhotoArc>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopPhotoReq {
    ///
    #[prost(string, tag = "1")]
    pub mobiapp: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(int32, tag = "3")]
    pub build: i32,
    ///
    #[prost(string, tag = "4")]
    pub device: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub login_mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpActivityTabReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub state: i32,
    ///
    #[prost(int64, tag = "3")]
    pub tab_cont: i64,
    ///
    #[prost(string, tag = "4")]
    pub tab_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpActivityTabResp {
    ///
    #[prost(bool, tag = "1")]
    pub success: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpRcmdBlackListReply {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserTabReply {
    ///
    #[prost(int32, tag = "1")]
    pub tab_type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(string, tag = "3")]
    pub tab_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub tab_order: i32,
    ///
    #[prost(int64, tag = "5")]
    pub tab_cont: i64,
    ///
    #[prost(int32, tag = "6")]
    pub is_default: i32,
    ///
    #[prost(string, tag = "7")]
    pub h5_link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserTabReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub plat: i32,
    ///
    #[prost(int32, tag = "3")]
    pub build: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhitelistAddReply {
    ///
    #[prost(bool, tag = "1")]
    pub add_ok: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhitelistAddReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub stime: i64,
    ///
    #[prost(int64, tag = "3")]
    pub etime: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhitelistReply {
    ///
    #[prost(bool, tag = "1")]
    pub is_white: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhitelistReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhitelistUpReply {
    ///
    #[prost(bool, tag = "1")]
    pub up_ok: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhitelistValidTimeReply {
    ///
    #[prost(bool, tag = "1")]
    pub is_white: bool,
    ///
    #[prost(int64, tag = "2")]
    pub stime: i64,
    ///
    #[prost(int64, tag = "3")]
    pub etime: i64,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TopPhotoType {
    ///
    Unknown = 0,
    ///
    Pic = 1,
    ///
    Archive = 2,
}
impl TopPhotoType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TopPhotoType::Unknown => "UNKNOWN",
            TopPhotoType::Pic => "PIC",
            TopPhotoType::Archive => "ARCHIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "PIC" => Some(Self::Pic),
            "ARCHIVE" => Some(Self::Archive),
            _ => None,
        }
    }
}
