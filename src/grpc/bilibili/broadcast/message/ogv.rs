/// 房间人员变更事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomMemberChangeEvent {
    /// 房间id
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    /// 房主id
    #[prost(int64, tag = "2")]
    pub owner_id: i64,
    /// 房间成员列表
    #[prost(message, repeated, tag = "3")]
    pub members: ::prost::alloc::vec::Vec<UserInfoProto>,
    /// 提示信息
    #[prost(message, optional, tag = "4")]
    pub message: ::core::option::Option<MessageProto>,
}
/// 播放进度同步事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressSyncEvent {
    /// 房间id
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    /// 播放中的season_id
    #[prost(int64, tag = "2")]
    pub season_id: i64,
    /// 播放中的episode_id
    #[prost(int64, tag = "3")]
    pub episode_id: i64,
    /// 播放状态
    #[prost(enumeration = "PlayStatus", tag = "4")]
    pub status: i32,
    /// 房主播放进度
    #[prost(int64, tag = "5")]
    pub progress: i64,
    /// 提示信息
    #[prost(message, optional, tag = "6")]
    pub message: ::core::option::Option<MessageProto>,
}
/// 房间状态更新
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomUpdateEvent {
    /// 房间id
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    /// 房间变更状态
    #[prost(enumeration = "RoomType", tag = "2")]
    pub r#type: i32,
    /// 提示信息
    #[prost(message, optional, tag = "3")]
    pub message: ::core::option::Option<MessageProto>,
}
/// 房间销毁通知
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomDestroyEvent {
    /// 房间id
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    /// 提示信息
    #[prost(message, optional, tag = "4")]
    pub message: ::core::option::Option<MessageProto>,
}
/// 房间触发通知
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomTriggerEvent {
    /// 操作人
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 提示信息
    #[prost(message, optional, tag = "2")]
    pub message: ::core::option::Option<MessageProto>,
    /// 触发类型
    #[prost(enumeration = "TriggerType", tag = "3")]
    pub trigger: i32,
}
/// 用户信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfoProto {
    /// 用户id
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 用户头像url
    #[prost(string, tag = "2")]
    pub face: ::prost::alloc::string::String,
    /// 昵称
    #[prost(string, tag = "3")]
    pub nickname: ::prost::alloc::string::String,
    /// 等级
    #[prost(int32, tag = "4")]
    pub level: i32,
    /// 签名
    #[prost(string, tag = "5")]
    pub sign: ::prost::alloc::string::String,
    /// 大会员信息
    #[prost(message, optional, tag = "6")]
    pub vip: ::core::option::Option<VipProto>,
    /// 身份认证信息
    #[prost(message, optional, tag = "7")]
    pub official: ::core::option::Option<OfficialProto>,
    /// 挂件信息
    #[prost(message, optional, tag = "8")]
    pub pendant: ::core::option::Option<PendantProto>,
    /// 设备buvid
    #[prost(string, tag = "9")]
    pub buvid: ::prost::alloc::string::String,
}
/// 通知信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageProto {
    /// 可带占位符匹配的消息体 ep "还没有其他小伙伴，\[去邀请>\]<<https://big.bilibili.com/mobile/giftIndex?mid=123>">
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// 消息体类型
    /// 0:json格式的文本消息 1:支持全文本可点(破冰)
    #[prost(int32, tag = "2")]
    pub content_type: i32,
}
/// 大会员信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipProto {
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    #[prost(int32, tag = "2")]
    pub status: i32,
    #[prost(int64, tag = "3")]
    pub due_date: i64,
    #[prost(int32, tag = "4")]
    pub vip_pay_type: i32,
    #[prost(int32, tag = "5")]
    pub theme_type: i32,
    /// 大会员角标
    /// 0:无角标 1:粉色大会员角标 2:绿色小会员角标
    #[prost(int32, tag = "6")]
    pub avatar_subscript: i32,
    /// 昵称色值，可能为空，色值示例：#FFFB9E60
    #[prost(string, tag = "7")]
    pub nickname_color: ::prost::alloc::string::String,
}
/// 认证信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialProto {
    #[prost(int32, tag = "1")]
    pub role: i32,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub r#type: i32,
}
/// 挂件信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendantProto {
    #[prost(int32, tag = "1")]
    pub pid: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub expire: i64,
    #[prost(string, tag = "5")]
    pub image_enhance: ::prost::alloc::string::String,
}
/// 通用信息通知
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageEvent {
    /// 房间id
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    /// 消息id
    #[prost(int64, tag = "2")]
    pub msg_id: i64,
    /// 消息发送服务端时间 时间戳 单位秒
    #[prost(int64, tag = "3")]
    pub ts: i64,
    /// 信息通知发送主体id
    #[prost(int64, tag = "4")]
    pub oid: i64,
    /// 信息通知发送领域
    #[prost(enumeration = "MessageDomain", tag = "5")]
    pub domain: i32,
    /// 通知信息类型
    #[prost(enumeration = "MessageType", tag = "6")]
    pub r#type: i32,
    /// 提示信息
    #[prost(message, optional, tag = "7")]
    pub message: ::core::option::Option<MessageProto>,
    /// 消息发送用户信息
    #[prost(message, optional, tag = "8")]
    pub user: ::core::option::Option<UserInfoProto>,
    /// 消息id str类型
    #[prost(string, tag = "9")]
    pub msg_id2: ::prost::alloc::string::String,
}
/// 聊天信息清除通知
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveChatEvent {
    /// 房间id
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    /// 撤回的聊天信息id
    #[prost(int64, tag = "2")]
    pub msg_id: i64,
    /// 提示信息
    #[prost(message, optional, tag = "3")]
    pub message: ::core::option::Option<MessageProto>,
}
/// "一起看"房间事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreyaEventBody {
    /// 房间id
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    /// 接收事件消息的白名单用户
    #[prost(int64, repeated, tag = "2")]
    pub white_mid: ::prost::alloc::vec::Vec<i64>,
    /// 不处理信息的黑名单用户 优先级低于白名单 当白名单有数据时 忽略黑名单
    #[prost(int64, repeated, tag = "3")]
    pub ignore_mid: ::prost::alloc::vec::Vec<i64>,
    /// 消息序列号
    #[prost(int64, tag = "100")]
    pub sequence_id: i64,
    /// 命令类型
    #[prost(oneof = "freya_event_body::Event", tags = "4, 5, 6, 7, 8, 9, 10")]
    pub event: ::core::option::Option<freya_event_body::Event>,
}
/// Nested message and enum types in `FreyaEventBody`.
pub mod freya_event_body {
    /// 命令类型
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// 房间人员变更事件
        #[prost(message, tag = "4")]
        MemberChange(super::RoomMemberChangeEvent),
        /// 播放进度同步事件
        #[prost(message, tag = "5")]
        Progress(super::ProgressSyncEvent),
        /// 房间状态更新
        #[prost(message, tag = "6")]
        RoomUpdate(super::RoomUpdateEvent),
        /// 通用信息通知
        #[prost(message, tag = "7")]
        Message(super::MessageEvent),
        /// 聊天信息清除通知
        #[prost(message, tag = "8")]
        RemoveChat(super::RemoveChatEvent),
        /// 房间销毁通知
        #[prost(message, tag = "9")]
        RoomDestroy(super::RoomDestroyEvent),
        /// 房间触发通知
        #[prost(message, tag = "10")]
        RoomTrigger(super::RoomTriggerEvent),
    }
}
/// 播放状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayStatus {
    /// 暂停
    Pause = 0,
    /// 播放
    Play = 1,
    /// 终止
    End = 2,
}
impl PlayStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayStatus::Pause => "Pause",
            PlayStatus::Play => "Play",
            PlayStatus::End => "End",
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
/// 房间类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomType {
    /// 私密
    Private = 0,
    /// 公开
    Open = 1,
}
impl RoomType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RoomType::Private => "Private",
            RoomType::Open => "Open",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Private" => Some(Self::Private),
            "Open" => Some(Self::Open),
            _ => None,
        }
    }
}
/// 信息通知发送领域
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageDomain {
    /// 默认
    DefaultDomain = 0,
    /// 房间用户
    RoomMid = 1,
    /// 系统通知
    SystemInfo = 2,
}
impl MessageDomain {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageDomain::DefaultDomain => "DefaultDomain",
            MessageDomain::RoomMid => "RoomMid",
            MessageDomain::SystemInfo => "SystemInfo",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DefaultDomain" => Some(Self::DefaultDomain),
            "RoomMid" => Some(Self::RoomMid),
            "SystemInfo" => Some(Self::SystemInfo),
            _ => None,
        }
    }
}
/// 通知信息类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageType {
    /// 默认
    DefaultType = 0,
    /// 房间用户
    ChatMessage = 1,
    /// 系统通知
    SystemMessage = 2,
}
impl MessageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageType::DefaultType => "DefaultType",
            MessageType::ChatMessage => "ChatMessage",
            MessageType::SystemMessage => "SystemMessage",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DefaultType" => Some(Self::DefaultType),
            "ChatMessage" => Some(Self::ChatMessage),
            "SystemMessage" => Some(Self::SystemMessage),
            _ => None,
        }
    }
}
/// 触发通知类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    /// 默认
    DefaultTrigger = 0,
    /// 关注、取消关注
    Relation = 1,
}
impl TriggerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TriggerType::DefaultTrigger => "DefaultTrigger",
            TriggerType::Relation => "Relation",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DefaultTrigger" => Some(Self::DefaultTrigger),
            "Relation" => Some(Self::Relation),
            _ => None,
        }
    }
}
/// 开播事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveStartEvent {}
/// 直播中止事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveEndEvent {}
/// 在线人数事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveOnlineEvent {
    /// 在线人数
    #[prost(int64, tag = "1")]
    pub online: i64,
}
/// 变更通知
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveUpdateEvent {
    /// 直播后状态
    /// 1:下线 2:转点播
    #[prost(int32, tag = "1")]
    pub after_premiere_type: i32,
    /// 直播开始绝对时间 单位ms
    #[prost(int64, tag = "2")]
    pub start_time: i64,
    /// id
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    /// 服务端播放进度，未打散，负数表示距离开播时间，正数表示已开播时间，单位：毫秒
    /// 用户实际播放进度：progress - delay_time
    #[prost(int64, tag = "4")]
    pub progress: i64,
}
/// 直播间事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmdBody {
    /// 命令类型
    #[prost(oneof = "cmd_body::Event", tags = "1, 2, 3, 4")]
    pub event: ::core::option::Option<cmd_body::Event>,
}
/// Nested message and enum types in `CMDBody`.
pub mod cmd_body {
    /// 命令类型
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// 开播事件
        #[prost(message, tag = "1")]
        Start(super::LiveStartEvent),
        /// 直播中止事件
        #[prost(message, tag = "2")]
        Emergency(super::LiveEndEvent),
        /// 在线人数事件
        #[prost(message, tag = "3")]
        Online(super::LiveOnlineEvent),
        /// 变更通知
        #[prost(message, tag = "4")]
        Update(super::LiveUpdateEvent),
    }
}
