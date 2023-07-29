/// 推送选项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomEvent {
    /// RoomStatus 类型
    #[prost(enumeration = "RoomStatus", tag = "1")]
    pub room_status: i32,
    ///
    #[prost(string, tag = "2")]
    pub room_message: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomStatus {
    /// 暂停:
    Pause = 0,
    /// 播放:
    Play = 1,
    /// 终止:
    End = 2,
}
impl RoomStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RoomStatus::Pause => "Pause",
            RoomStatus::Play => "Play",
            RoomStatus::End => "End",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Pause" => Some(Self::Pause),
            "Play" => Some(Self::Play),
            "End" => Some(Self::End),
            _ => None,
        }
    }
}
