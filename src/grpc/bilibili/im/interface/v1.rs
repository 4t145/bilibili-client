/// 空请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DummyReq {
    ///
    #[prost(uint32, tag = "1")]
    pub idl: u32,
}
/// 空响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DummyRsp {}
/// 表情资源信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmotionInfo {
    /// 表情
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 表情url
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    /// 表情大小
    /// 0:未知 1:min 2:max
    #[prost(int32, tag = "3")]
    pub size: i32,
    /// gif url
    #[prost(string, tag = "4")]
    pub gif_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDetail {
    ///
    #[prost(int64, tag = "1")]
    pub msg_key: i64,
    ///
    #[prost(int64, tag = "2")]
    pub seqno: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFeedUnreadRsp {
    ///
    #[prost(map = "string, int64", tag = "1")]
    pub unread: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
/// 更新应援团小助手消息已拉取进度-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqAckAssisMsg {
    ///
    #[prost(uint64, tag = "1")]
    pub ack_seqno: u64,
}
/// 拉取已读消息-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqAckSessions {
    ///
    #[prost(uint64, tag = "1")]
    pub begin_ts: u64,
    ///
    #[prost(uint32, tag = "2")]
    pub end_ts: u32,
    ///
    #[prost(uint32, tag = "3")]
    pub size: u32,
}
/// 批量删除会话-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqBatRmSess {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqCloseClearUnreadUi {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqGetMsg {
    ///
    #[prost(int64, tag = "1")]
    pub talker_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub session_type: i32,
    ///
    #[prost(message, repeated, tag = "3")]
    pub msg_detail: ::prost::alloc::vec::Vec<MsgDetail>,
}
/// 拉取会话记录列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqGetSessions {
    ///
    #[prost(uint64, tag = "1")]
    pub begin_ts: u64,
    ///
    #[prost(uint64, tag = "2")]
    pub end_ts: u64,
    ///
    #[prost(uint32, tag = "3")]
    pub size: u32,
    ///
    #[prost(uint32, tag = "4")]
    pub session_type: u32,
    ///
    #[prost(uint32, tag = "5")]
    pub unfollow_fold: u32,
    ///
    #[prost(uint32, tag = "6")]
    pub group_fold: u32,
    ///
    #[prost(uint32, tag = "7")]
    pub sort_rule: u32,
    /// 青少年模式
    #[prost(uint32, tag = "8")]
    pub teenager_mode: u32,
    /// 课堂模式
    #[prost(uint32, tag = "9")]
    pub lessons_mode: u32,
}
/// -请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqGetSpecificSessions {
    /// 具体会话详情
    #[prost(message, repeated, tag = "1")]
    pub talker_sessions: ::prost::alloc::vec::Vec<SimpleSession>,
}
/// 应援团消息助手-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqGroupAssisMsg {
    ///
    #[prost(uint64, tag = "1")]
    pub client_seqno: u64,
    ///
    #[prost(uint32, tag = "2")]
    pub size: u32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqLiveInfo {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub talker_id: i64,
}
/// 拉取新消息-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqNewSessions {
    ///
    #[prost(uint64, tag = "1")]
    pub begin_ts: u64,
    ///
    #[prost(uint32, tag = "2")]
    pub size: u32,
    ///
    #[prost(uint32, tag = "3")]
    pub teenager_mode: u32,
    /// 课堂模式
    #[prost(uint32, tag = "4")]
    pub lessons_mode: u32,
}
/// 同步关系-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqRelationSync {
    /// 客户端当前seqno
    #[prost(uint64, tag = "1")]
    pub client_relation_oplog_seqno: u64,
}
/// 删除会话记录-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqRemoveSession {
    ///
    #[prost(uint64, tag = "1")]
    pub talker_id: u64,
    ///
    #[prost(uint32, tag = "2")]
    pub session_type: u32,
}
/// 发送消息-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSendMsg {
    /// 消息内容
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<super::super::r#type::Msg>,
    ///
    #[prost(string, tag = "2")]
    pub cookie: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cookie2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub error_code: i32,
    ///
    #[prost(string, tag = "5")]
    pub dev_id: ::prost::alloc::string::String,
}
/// 拉取会话详情-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSessionDetail {
    ///
    #[prost(uint64, tag = "1")]
    pub talker_id: u64,
    ///
    #[prost(uint32, tag = "2")]
    pub session_type: u32,
    ///
    #[prost(uint64, tag = "3")]
    pub uid: u64,
}
/// 批量拉取会话详情-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSessionDetails {
    /// 会话详情请求列表
    #[prost(message, repeated, tag = "1")]
    pub sess_ids: ::prost::alloc::vec::Vec<ReqSessionDetail>,
}
/// 同步版本拉取消息-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSessionMsg {
    ///
    #[prost(uint64, tag = "1")]
    pub talker_id: u64,
    ///
    #[prost(int32, tag = "2")]
    pub session_type: i32,
    ///
    #[prost(uint64, tag = "3")]
    pub end_seqno: u64,
    ///
    #[prost(uint64, tag = "4")]
    pub begin_seqno: u64,
    ///
    #[prost(int32, tag = "5")]
    pub size: i32,
    ///
    #[prost(int32, tag = "6")]
    pub order: i32,
    ///
    #[prost(string, tag = "7")]
    pub dev_id: ::prost::alloc::string::String,
}
/// 置顶聊天-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSetTop {
    ///
    #[prost(uint64, tag = "1")]
    pub talker_id: u64,
    ///
    #[prost(uint32, tag = "2")]
    pub session_type: u32,
    ///
    /// 0:置顶 1:取消置顶
    #[prost(uint32, tag = "3")]
    pub op_type: u32,
}
/// 拉取最近私信分享列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqShareList {
    /// 分页大小 最大20
    #[prost(int32, tag = "1")]
    pub size: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqShowClearUnreadUi {
    ///
    #[prost(int32, tag = "1")]
    pub unread_type: i32,
    ///
    #[prost(int32, tag = "2")]
    pub show_unfollow_list: i32,
    ///
    #[prost(int32, tag = "4")]
    pub show_dustbin: i32,
}
/// 未读私信数-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSingleUnread {
    ///
    #[prost(int32, tag = "1")]
    pub unread_type: i32,
    ///
    #[prost(int32, tag = "2")]
    pub show_unfollow_list: i32,
    ///
    #[prost(int64, tag = "3")]
    pub uid: i64,
    ///
    #[prost(int32, tag = "4")]
    pub show_dustbin: i32,
}
/// -请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSpecificSingleUnread {
    /// 具体会话详情
    #[prost(message, repeated, tag = "1")]
    pub talker_sessions: ::prost::alloc::vec::Vec<SimpleSession>,
}
/// 确认同步进度-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSyncAck {
    ///
    #[prost(uint64, tag = "1")]
    pub client_seqno: u64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqTotalUnread {
    ///
    #[prost(int32, tag = "1")]
    pub unread_type: i32,
    ///
    #[prost(int32, tag = "2")]
    pub show_unfollow_list: i32,
    ///
    #[prost(int64, tag = "3")]
    pub uid: i64,
    ///
    #[prost(int32, tag = "4")]
    pub show_dustbin: i32,
    ///
    #[prost(int32, tag = "5")]
    pub singleunread_on: i32,
    ///
    #[prost(int32, tag = "6")]
    pub msgfeed_on: i32,
    ///
    #[prost(int32, tag = "7")]
    pub sysup_on: i32,
}
/// 更新已读进度-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqUpdateAck {
    /// 聊天对象uid，可以为用户id或者为群id
    #[prost(uint64, tag = "1")]
    pub talker_id: u64,
    /// 会话类型
    #[prost(uint32, tag = "2")]
    pub session_type: u32,
    /// 已读的最大seqno
    #[prost(uint64, tag = "3")]
    pub ack_seqno: u64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqUpdateIntercept {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub talker_id: i64,
    ///
    #[prost(int32, tag = "3")]
    pub status: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqUpdateTotalUnread {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspCloseClearUnreadUi {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspGetMsg {
    ///
    #[prost(message, repeated, tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<super::super::r#type::Msg>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspLiveInfo {
    ///
    #[prost(int64, tag = "1")]
    pub live_status: i64,
    ///
    #[prost(string, tag = "2")]
    pub jump_url: ::prost::alloc::string::String,
}
/// 我创建的应援团未读数-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspMyGroupUnread {
    /// 未读消息数
    #[prost(uint32, tag = "1")]
    pub unread_count: u32,
}
/// 同步关系-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspRelationSync {
    ///
    #[prost(int32, tag = "1")]
    pub full: i32,
    /// 增量日志
    #[prost(message, repeated, tag = "2")]
    pub relation_logs: ::prost::alloc::vec::Vec<super::super::r#type::RelationLog>,
    /// 全量列表
    #[prost(message, repeated, tag = "3")]
    pub friend_list: ::prost::alloc::vec::Vec<super::super::r#type::FriendRelation>,
    /// 服务器端最大的relation seqno
    #[prost(uint64, tag = "4")]
    pub server_relation_oplog_seqno: u64,
    /// 全量列表
    #[prost(message, repeated, tag = "5")]
    pub group_list: ::prost::alloc::vec::Vec<super::super::r#type::GroupRelation>,
}
/// 发送消息-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSendMsg {
    ///
    #[prost(uint64, tag = "1")]
    pub msg_key: u64,
    /// 表情资源信息
    #[prost(message, repeated, tag = "2")]
    pub e_infos: ::prost::alloc::vec::Vec<EmotionInfo>,
    ///
    #[prost(string, tag = "3")]
    pub msg_content: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub key_hit_infos: ::core::option::Option<super::super::r#type::KeyHitInfos>,
}
/// 批量拉取会话详情-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSessionDetails {
    /// 会话详情列表
    #[prost(message, repeated, tag = "1")]
    pub sess_infos: ::prost::alloc::vec::Vec<super::super::r#type::SessionInfo>,
}
/// 同步版本拉取消息-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSessionMsg {
    ///
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<super::super::r#type::Msg>,
    ///
    #[prost(int32, tag = "2")]
    pub has_more: i32,
    ///
    #[prost(uint64, tag = "3")]
    pub min_seqno: u64,
    ///
    #[prost(uint64, tag = "4")]
    pub max_seqno: u64,
    /// 表情资源信息
    #[prost(message, repeated, tag = "5")]
    pub e_infos: ::prost::alloc::vec::Vec<EmotionInfo>,
}
/// 拉取消息-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSessions {
    ///
    #[prost(message, repeated, tag = "1")]
    pub session_list: ::prost::alloc::vec::Vec<super::super::r#type::SessionInfo>,
    ///
    #[prost(uint32, tag = "2")]
    pub has_more: u32,
    /// 标记反垃圾会话是否在清理中
    #[prost(bool, tag = "3")]
    pub anti_disturb_cleaning: bool,
    /// 当session_list为空时，会返回该字段用于判断通讯录是否为空，1表示空，0表示非空
    #[prost(int32, tag = "4")]
    pub is_address_list_empty: i32,
    ///
    #[prost(map = "int32, int64", tag = "5")]
    pub system_msg: ::std::collections::HashMap<i32, i64>,
    ///
    #[prost(bool, tag = "6")]
    pub show_level: bool,
}
/// 拉取最近私信分享列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspShareList {
    /// 最近会话列表
    #[prost(message, repeated, tag = "1")]
    pub session_list: ::prost::alloc::vec::Vec<ShareSessionInfo>,
    ///
    #[prost(int32, tag = "2")]
    pub is_address_list_empty: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspShowClearUnreadUi {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// 未读私信数-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSingleUnread {
    /// 未关注用户私信数
    #[prost(uint64, tag = "1")]
    pub unfollow_unread: u64,
    /// 已关注用户私信数
    #[prost(uint64, tag = "2")]
    pub follow_unread: u64,
    /// 未关注人列表是否有新业务通知
    #[prost(uint32, tag = "3")]
    pub unfollow_push_msg: u32,
    ///
    #[prost(int32, tag = "4")]
    pub dustbin_push_msg: i32,
    ///
    #[prost(int64, tag = "5")]
    pub dustbin_unread: i64,
    ///
    #[prost(int64, tag = "6")]
    pub biz_msg_unfollow_unread: i64,
    ///
    #[prost(int64, tag = "7")]
    pub biz_msg_follow_unread: i64,
}
/// -响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSpecificSingleUnread {
    /// key -> 用户uid, value ->未读数
    #[prost(map = "uint64, uint64", tag = "1")]
    pub talker_unread_cnt: ::std::collections::HashMap<u64, u64>,
    /// 总未读数
    #[prost(uint64, tag = "2")]
    pub all_unread_cnt: u64,
}
/// 确认同步进度-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSyncAck {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspTotalUnread {
    ///
    #[prost(message, optional, tag = "1")]
    pub session_single_unread: ::core::option::Option<SessionSingleUnreadRsp>,
    ///
    #[prost(message, optional, tag = "2")]
    pub msg_feed_unread: ::core::option::Option<MsgFeedUnreadRsp>,
    ///
    #[prost(message, optional, tag = "3")]
    pub sys_msg_interface_last_msg: ::core::option::Option<SysMsgInterfaceLastMsgRsp>,
    ///
    #[prost(int32, tag = "4")]
    pub total_unread: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspUpdateTotalUnread {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionSingleUnreadRsp {
    ///
    #[prost(int64, tag = "1")]
    pub unfollow_unread: i64,
    ///
    #[prost(int64, tag = "2")]
    pub follow_unread: i64,
    ///
    #[prost(int32, tag = "3")]
    pub unfollow_push_msg: i32,
    ///
    #[prost(int32, tag = "4")]
    pub dustbin_push_msg: i32,
    ///
    #[prost(int64, tag = "5")]
    pub dustbin_unread: i64,
}
/// 会话信息，用于私信分享
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareSessionInfo {
    ///
    #[prost(uint64, tag = "1")]
    pub talker_id: u64,
    ///
    #[prost(string, tag = "2")]
    pub talker_uname: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub talker_icon: ::prost::alloc::string::String,
    /// 认证信息
    /// -1: 无认证 0:个人认证 1:机构认证
    #[prost(int32, tag = "4")]
    pub official_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleSession {
    /// 聊天对象uid，可以为用户id或者为群id
    #[prost(uint64, tag = "1")]
    pub talker_id: u64,
    /// 会话类型
    #[prost(uint32, tag = "2")]
    pub session_type: u32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SysMsgInterfaceLastMsgRsp {
    ///
    #[prost(int32, tag = "1")]
    pub unread: i32,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub time: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub id: i64,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EnumFold {
    ///
    FoldNo = 0,
    ///
    FoldYes = 1,
    ///
    FoldUnknown = 2,
}
impl EnumFold {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EnumFold::FoldNo => "FOLD_NO",
            EnumFold::FoldYes => "FOLD_YES",
            EnumFold::FoldUnknown => "FOLD_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FOLD_NO" => Some(Self::FoldNo),
            "FOLD_YES" => Some(Self::FoldYes),
            "FOLD_UNKNOWN" => Some(Self::FoldUnknown),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EnumUnreadType {
    ///
    UnreadTypeAll = 0,
    ///
    UnreadTypeFollow = 1,
    ///
    UnreadTypeUnfollow = 2,
    ///
    UnreadTypeDustbin = 3,
}
impl EnumUnreadType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EnumUnreadType::UnreadTypeAll => "UNREAD_TYPE_ALL",
            EnumUnreadType::UnreadTypeFollow => "UNREAD_TYPE_FOLLOW",
            EnumUnreadType::UnreadTypeUnfollow => "UNREAD_TYPE_UNFOLLOW",
            EnumUnreadType::UnreadTypeDustbin => "UNREAD_TYPE_DUSTBIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNREAD_TYPE_ALL" => Some(Self::UnreadTypeAll),
            "UNREAD_TYPE_FOLLOW" => Some(Self::UnreadTypeFollow),
            "UNREAD_TYPE_UNFOLLOW" => Some(Self::UnreadTypeUnfollow),
            "UNREAD_TYPE_DUSTBIN" => Some(Self::UnreadTypeDustbin),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionType {
    ///
    Unknown = 0,
    ///
    UnFoldSession = 1,
    ///
    UnFollowSingleSession = 2,
    ///
    MyGroupSession = 3,
    ///
    AllSession = 4,
    ///
    DustbinSession = 5,
}
impl SessionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SessionType::Unknown => "UNKNOWN",
            SessionType::UnFoldSession => "UN_FOLD_SESSION",
            SessionType::UnFollowSingleSession => "UN_FOLLOW_SINGLE_SESSION",
            SessionType::MyGroupSession => "MY_GROUP_SESSION",
            SessionType::AllSession => "ALL_SESSION",
            SessionType::DustbinSession => "DUSTBIN_SESSION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "UN_FOLD_SESSION" => Some(Self::UnFoldSession),
            "UN_FOLLOW_SINGLE_SESSION" => Some(Self::UnFollowSingleSession),
            "MY_GROUP_SESSION" => Some(Self::MyGroupSession),
            "ALL_SESSION" => Some(Self::AllSession),
            "DUSTBIN_SESSION" => Some(Self::DustbinSession),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod im_interface_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 私信
    #[derive(Debug, Clone)]
    pub struct ImInterfaceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ImInterfaceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ImInterfaceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ImInterfaceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ImInterfaceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// 发送消息
        pub async fn send_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqSendMsg>,
        ) -> std::result::Result<tonic::Response<super::RspSendMsg>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/SendMsg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.im.interface.v1.ImInterface", "SendMsg"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 同步关系
        pub async fn sync_relation(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqRelationSync>,
        ) -> std::result::Result<
            tonic::Response<super::RspRelationSync>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/SyncRelation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "SyncRelation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 确认同步进度
        pub async fn sync_ack(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqSyncAck>,
        ) -> std::result::Result<tonic::Response<super::RspSyncAck>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/SyncAck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.im.interface.v1.ImInterface", "SyncAck"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 同步版本拉取消息
        pub async fn sync_fetch_session_msgs(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqSessionMsg>,
        ) -> std::result::Result<tonic::Response<super::RspSessionMsg>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/SyncFetchSessionMsgs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "SyncFetchSessionMsgs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 拉取会话记录列表
        pub async fn get_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqGetSessions>,
        ) -> std::result::Result<tonic::Response<super::RspSessions>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/GetSessions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "GetSessions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 拉取新消息
        pub async fn new_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqNewSessions>,
        ) -> std::result::Result<tonic::Response<super::RspSessions>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/NewSessions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "NewSessions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 拉取已读消息
        pub async fn ack_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqAckSessions>,
        ) -> std::result::Result<tonic::Response<super::RspSessions>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/AckSessions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "AckSessions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 更新已读进度
        pub async fn update_ack(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqUpdateAck>,
        ) -> std::result::Result<tonic::Response<super::DummyRsp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/UpdateAck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.im.interface.v1.ImInterface", "UpdateAck"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 置顶聊天
        pub async fn set_top(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqSetTop>,
        ) -> std::result::Result<tonic::Response<super::DummyRsp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/SetTop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.im.interface.v1.ImInterface", "SetTop"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 删除会话记录
        pub async fn remove_session(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqRemoveSession>,
        ) -> std::result::Result<tonic::Response<super::DummyRsp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/RemoveSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "RemoveSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 未读私信数
        pub async fn single_unread(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqSingleUnread>,
        ) -> std::result::Result<
            tonic::Response<super::RspSingleUnread>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/SingleUnread",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "SingleUnread",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 我创建的应援团未读数
        pub async fn my_group_unread(
            &mut self,
            request: impl tonic::IntoRequest<super::DummyReq>,
        ) -> std::result::Result<
            tonic::Response<super::RspMyGroupUnread>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/MyGroupUnread",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "MyGroupUnread",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 未关注的人批量设置为已读
        pub async fn update_unflw_read(
            &mut self,
            request: impl tonic::IntoRequest<super::DummyReq>,
        ) -> std::result::Result<tonic::Response<super::DummyRsp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/UpdateUnflwRead",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "UpdateUnflwRead",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 应援团消息助手
        pub async fn group_assis_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqGroupAssisMsg>,
        ) -> std::result::Result<tonic::Response<super::RspSessionMsg>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/GroupAssisMsg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "GroupAssisMsg",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 更新应援团小助手消息已拉取进度
        pub async fn ack_assis_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqAckAssisMsg>,
        ) -> std::result::Result<tonic::Response<super::DummyRsp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/AckAssisMsg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "AckAssisMsg",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 拉取会话详情
        pub async fn session_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqSessionDetail>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::r#type::SessionInfo>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/SessionDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "SessionDetail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 批量拉取会话详情
        pub async fn batch_sess_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqSessionDetails>,
        ) -> std::result::Result<
            tonic::Response<super::RspSessionDetails>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/BatchSessDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "BatchSessDetail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 批量删除会话
        pub async fn batch_rm_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqBatRmSess>,
        ) -> std::result::Result<tonic::Response<super::DummyRsp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/BatchRmSessions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "BatchRmSessions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 拉取最近私信分享列表
        pub async fn share_list(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqShareList>,
        ) -> std::result::Result<tonic::Response<super::RspShareList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/ShareList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.im.interface.v1.ImInterface", "ShareList"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn specific_single_unread(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqSpecificSingleUnread>,
        ) -> std::result::Result<
            tonic::Response<super::RspSpecificSingleUnread>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/SpecificSingleUnread",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "SpecificSingleUnread",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn get_specific_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqGetSpecificSessions>,
        ) -> std::result::Result<tonic::Response<super::RspSessions>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/GetSpecificSessions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "GetSpecificSessions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn get_live_info(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqLiveInfo>,
        ) -> std::result::Result<tonic::Response<super::RspLiveInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/GetLiveInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "GetLiveInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn get_total_unread(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqTotalUnread>,
        ) -> std::result::Result<tonic::Response<super::RspTotalUnread>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/GetTotalUnread",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "GetTotalUnread",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn show_clear_unread_ui(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqShowClearUnreadUi>,
        ) -> std::result::Result<
            tonic::Response<super::RspShowClearUnreadUi>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/ShowClearUnreadUI",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "ShowClearUnreadUI",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn close_clear_unread_ui(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqCloseClearUnreadUi>,
        ) -> std::result::Result<
            tonic::Response<super::RspCloseClearUnreadUi>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/CloseClearUnreadUI",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "CloseClearUnreadUI",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn update_total_unread(
            &mut self,
            request: impl tonic::IntoRequest<super::ReqUpdateTotalUnread>,
        ) -> std::result::Result<
            tonic::Response<super::RspUpdateTotalUnread>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.im.interface.v1.ImInterface/UpdateTotalUnread",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.im.interface.v1.ImInterface",
                        "UpdateTotalUnread",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
