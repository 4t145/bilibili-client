///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountInfo {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub pic_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendRelation {
    /// 用户mid
    #[prost(uint64, tag = "1")]
    pub uid: u64,
    /// 用户昵称
    #[prost(string, tag = "2")]
    pub user_name: ::prost::alloc::string::String,
    /// 头像url
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    /// vip类型
    /// 0:无 1:月度大会员 2:年度大会员
    #[prost(uint32, tag = "4")]
    pub vip_level: u32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRelation {
    ///
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
    ///
    #[prost(uint64, tag = "2")]
    pub owner_uid: u64,
    ///
    #[prost(uint32, tag = "3")]
    pub group_type: u32,
    ///
    #[prost(uint32, tag = "4")]
    pub group_level: u32,
    ///
    #[prost(string, tag = "5")]
    pub group_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub group_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub group_notice: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub status: i32,
    ///
    #[prost(int32, tag = "9")]
    pub member_role: i32,
    ///
    #[prost(string, tag = "10")]
    pub fans_medal_name: ::prost::alloc::string::String,
    ///
    #[prost(uint64, tag = "11")]
    pub room_id: u64,
}
/// 关键词高亮文本
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HighText {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    /// 表示高亮文本应该高亮第几个匹配的文本，ps：比如，“有疑问请联系客服，联系客服时，请说明具体的情况”，一共有2个匹配的文本，要高亮第一个匹配的，则index=1
    #[prost(uint32, tag = "3")]
    pub index: u32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImgInfo {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub width: i32,
    ///
    #[prost(int32, tag = "3")]
    pub height: i32,
    ///
    #[prost(string, tag = "4")]
    pub image_type: ::prost::alloc::string::String,
}
/// 关键词命中信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyHitInfos {
    ///
    #[prost(string, tag = "1")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(uint32, tag = "2")]
    pub rule_id: u32,
    ///
    ///
    ///
    #[prost(message, repeated, tag = "3")]
    pub high_text: ::prost::alloc::vec::Vec<HighText>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Medal {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub medal_id: i32,
    ///
    #[prost(int32, tag = "3")]
    pub level: i32,
    ///
    #[prost(string, tag = "4")]
    pub medal_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub score: i32,
    ///
    #[prost(int32, tag = "6")]
    pub intimacy: i32,
    ///
    #[prost(int32, tag = "7")]
    pub master_status: i32,
    ///
    #[prost(int32, tag = "8")]
    pub is_receive: i32,
    ///
    #[prost(int64, tag = "9")]
    pub medal_color_start: i64,
    ///
    #[prost(int64, tag = "10")]
    pub medal_color_end: i64,
    ///
    #[prost(int64, tag = "11")]
    pub medal_color_border: i64,
    ///
    #[prost(int64, tag = "12")]
    pub medal_color_name: i64,
    ///
    #[prost(int64, tag = "13")]
    pub medal_color_level: i64,
    ///
    #[prost(int64, tag = "14")]
    pub guard_level: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    /// 发送方mid
    #[prost(uint64, tag = "1")]
    pub sender_uid: u64,
    /// 接收方类型
    #[prost(enumeration = "RecverType", tag = "2")]
    pub receiver_type: i32,
    /// 接收方mid
    #[prost(uint64, tag = "3")]
    pub receiver_id: u64,
    /// 客户端的序列id,用于服务端去重
    #[prost(uint64, tag = "4")]
    pub cli_msg_id: u64,
    /// 消息类型
    #[prost(enumeration = "MsgType", tag = "5")]
    pub msg_type: i32,
    /// 消息内容
    #[prost(string, tag = "6")]
    pub content: ::prost::alloc::string::String,
    /// 服务端的序列号x
    #[prost(uint64, tag = "7")]
    pub msg_seqno: u64,
    /// 消息发送时间（服务端时间）
    #[prost(uint64, tag = "8")]
    pub timestamp: u64,
    /// @用户列表
    #[prost(uint64, repeated, tag = "9")]
    pub at_uids: ::prost::alloc::vec::Vec<u64>,
    /// 多人消息
    #[prost(uint64, repeated, tag = "10")]
    pub recver_ids: ::prost::alloc::vec::Vec<u64>,
    /// 消息唯一标示
    #[prost(uint64, tag = "11")]
    pub msg_key: u64,
    /// 消息状态
    #[prost(uint32, tag = "12")]
    pub msg_status: u32,
    /// 是否为系统撤销
    #[prost(bool, tag = "13")]
    pub sys_cancel: bool,
    /// 通知码
    #[prost(string, tag = "14")]
    pub notify_code: ::prost::alloc::string::String,
    /// 消息来源
    #[prost(enumeration = "MsgSource", tag = "15")]
    pub msg_source: i32,
    /// 如果msg.content有表情字符，则该参数需要置为1
    #[prost(int32, tag = "16")]
    pub new_face_version: i32,
    /// 命中关键词信息
    #[prost(message, optional, tag = "17")]
    pub key_hit_infos: ::core::option::Option<KeyHitInfos>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationLog {
    /// 操作类型
    #[prost(enumeration = "RelationLogType", tag = "1")]
    pub log_type: i32,
    /// 操作seqno
    #[prost(uint64, tag = "2")]
    pub oplog_seqno: u64,
    /// 好友信息
    #[prost(message, optional, tag = "3")]
    pub friend_relation: ::core::option::Option<FriendRelation>,
    /// 群信息
    #[prost(message, optional, tag = "4")]
    pub group_relation: ::core::option::Option<GroupRelation>,
}
/// 会话详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionInfo {
    ///
    #[prost(uint64, tag = "1")]
    pub talker_id: u64,
    ///
    #[prost(uint32, tag = "2")]
    pub session_type: u32,
    ///
    #[prost(uint64, tag = "3")]
    pub at_seqno: u64,
    ///
    #[prost(uint64, tag = "4")]
    pub top_ts: u64,
    ///
    #[prost(string, tag = "5")]
    pub group_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub group_cover: ::prost::alloc::string::String,
    ///
    #[prost(uint32, tag = "7")]
    pub is_follow: u32,
    ///
    #[prost(uint32, tag = "8")]
    pub is_dnd: u32,
    ///
    #[prost(uint64, tag = "9")]
    pub ack_seqno: u64,
    ///
    #[prost(uint64, tag = "10")]
    pub ack_ts: u64,
    ///
    #[prost(uint64, tag = "11")]
    pub session_ts: u64,
    ///
    #[prost(uint32, tag = "12")]
    pub unread_count: u32,
    ///
    #[prost(message, optional, tag = "13")]
    pub last_msg: ::core::option::Option<Msg>,
    ///
    #[prost(uint32, tag = "14")]
    pub group_type: u32,
    ///
    #[prost(uint32, tag = "15")]
    pub can_fold: u32,
    ///
    #[prost(uint32, tag = "16")]
    pub status: u32,
    ///
    #[prost(uint64, tag = "17")]
    pub max_seqno: u64,
    /// 会话是否有业务通知
    #[prost(uint32, tag = "18")]
    pub new_push_msg: u32,
    /// 接收方是否设置接受推送
    #[prost(uint32, tag = "19")]
    pub setting: u32,
    ///
    #[prost(uint32, tag = "20")]
    pub is_guardian: u32,
    ///
    #[prost(int32, tag = "21")]
    pub is_intercept: i32,
    ///
    #[prost(int32, tag = "22")]
    pub is_trust: i32,
    ///
    #[prost(int32, tag = "23")]
    pub system_msg_type: i32,
    ///
    #[prost(message, optional, tag = "24")]
    pub account_info: ::core::option::Option<AccountInfo>,
    ///
    #[prost(int32, tag = "25")]
    pub live_status: i32,
    ///
    #[prost(int32, tag = "26")]
    pub biz_msg_unread_count: i32,
    ///
    #[prost(message, optional, tag = "27")]
    pub user_label: ::core::option::Option<UserLabel>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLabel {
    ///
    #[prost(int32, tag = "1")]
    pub label_type: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub medal: ::core::option::Option<Medal>,
    ///
    #[prost(int32, tag = "3")]
    pub guardian_relation: i32,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CmdId {
    /// 非法cmd
    EnCmdIdInvalid = 0,
    /// msg_svr
    ///
    /// 发消息
    EnCmdIdSendMsg = 200001,
    /// sync_msg_svr
    ///
    /// 同步消息
    EnCmdIdSyncMsg = 500001,
    /// 同步相关链
    EnCmdIdSyncRelation = 500002,
    /// 客户端同步消息完成后，向服务器确认同步进度
    EnCmdIdSyncAck = 500003,
    /// 多端同步版本拉取消息
    EnCmdIdSyncFetchSessionMsgs = 500006,
    /// session_svr
    ///
    /// 拉会话列表
    EnCmdIdSessionSvrGetSessions = 1000001,
    /// 新消息到达时获取会话列表
    EnCmdIdSessionSvrNewSessions = 1000002,
    /// 获取已读位置有更新的会话列表
    EnCmdIdSessionSvrAckSessions = 1000003,
    /// 更新已读进度
    EnCmdIdSessionSvrUpdateAck = 1000004,
    /// 置顶/取消置顶
    EnCmdIdSessionSvrSetTop = 1000005,
    /// 删除会话
    EnCmdIdSessionSvrRemoveSession = 1000007,
    /// 单聊未读信息数
    EnCmdIdSessionSvrSingleUnread = 1000008,
    /// 我创建的应援团未读数
    EnCmdIdSessionSvrMyGroupUnread = 1000009,
    /// 未关注的人批量设置为已读
    EnCmdIdSessionSvrUpdateUnflwRead = 1000010,
    /// 应援团消息助手
    EnCmdIdSessionSvrGroupAssisMsg = 1000011,
    /// 更新应援团小助手消息已拉取进度
    EnCmdIdSessionSvrAckAssisMsg = 1000012,
    /// 拉会话详情
    EnCmdIdSessionSvrSessionDetail = 1000015,
    /// 批量拉会话详情
    EnCmdIdSessionSvrBatchSessDetail = 1000016,
    /// 批量删除会话
    EnCmdIdSessionSvrBatchRmSessions = 1000017,
}
impl CmdId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CmdId::EnCmdIdInvalid => "EN_CMD_ID_INVALID",
            CmdId::EnCmdIdSendMsg => "EN_CMD_ID_SEND_MSG",
            CmdId::EnCmdIdSyncMsg => "EN_CMD_ID_SYNC_MSG",
            CmdId::EnCmdIdSyncRelation => "EN_CMD_ID_SYNC_RELATION",
            CmdId::EnCmdIdSyncAck => "EN_CMD_ID_SYNC_ACK",
            CmdId::EnCmdIdSyncFetchSessionMsgs => "EN_CMD_ID_SYNC_FETCH_SESSION_MSGS",
            CmdId::EnCmdIdSessionSvrGetSessions => "EN_CMD_ID_SESSION_SVR_GET_SESSIONS",
            CmdId::EnCmdIdSessionSvrNewSessions => "EN_CMD_ID_SESSION_SVR_NEW_SESSIONS",
            CmdId::EnCmdIdSessionSvrAckSessions => "EN_CMD_ID_SESSION_SVR_ACK_SESSIONS",
            CmdId::EnCmdIdSessionSvrUpdateAck => "EN_CMD_ID_SESSION_SVR_UPDATE_ACK",
            CmdId::EnCmdIdSessionSvrSetTop => "EN_CMD_ID_SESSION_SVR_SET_TOP",
            CmdId::EnCmdIdSessionSvrRemoveSession => {
                "EN_CMD_ID_SESSION_SVR_REMOVE_SESSION"
            }
            CmdId::EnCmdIdSessionSvrSingleUnread => "EN_CMD_ID_SESSION_SVR_SINGLE_UNREAD",
            CmdId::EnCmdIdSessionSvrMyGroupUnread => {
                "EN_CMD_ID_SESSION_SVR_MY_GROUP_UNREAD"
            }
            CmdId::EnCmdIdSessionSvrUpdateUnflwRead => {
                "EN_CMD_ID_SESSION_SVR_UPDATE_UNFLW_READ"
            }
            CmdId::EnCmdIdSessionSvrGroupAssisMsg => {
                "EN_CMD_ID_SESSION_SVR_GROUP_ASSIS_MSG"
            }
            CmdId::EnCmdIdSessionSvrAckAssisMsg => "EN_CMD_ID_SESSION_SVR_ACK_ASSIS_MSG",
            CmdId::EnCmdIdSessionSvrSessionDetail => {
                "EN_CMD_ID_SESSION_SVR_SESSION_DETAIL"
            }
            CmdId::EnCmdIdSessionSvrBatchSessDetail => {
                "EN_CMD_ID_SESSION_SVR_BATCH_SESS_DETAIL"
            }
            CmdId::EnCmdIdSessionSvrBatchRmSessions => {
                "EN_CMD_ID_SESSION_SVR_BATCH_RM_SESSIONS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EN_CMD_ID_INVALID" => Some(Self::EnCmdIdInvalid),
            "EN_CMD_ID_SEND_MSG" => Some(Self::EnCmdIdSendMsg),
            "EN_CMD_ID_SYNC_MSG" => Some(Self::EnCmdIdSyncMsg),
            "EN_CMD_ID_SYNC_RELATION" => Some(Self::EnCmdIdSyncRelation),
            "EN_CMD_ID_SYNC_ACK" => Some(Self::EnCmdIdSyncAck),
            "EN_CMD_ID_SYNC_FETCH_SESSION_MSGS" => {
                Some(Self::EnCmdIdSyncFetchSessionMsgs)
            }
            "EN_CMD_ID_SESSION_SVR_GET_SESSIONS" => {
                Some(Self::EnCmdIdSessionSvrGetSessions)
            }
            "EN_CMD_ID_SESSION_SVR_NEW_SESSIONS" => {
                Some(Self::EnCmdIdSessionSvrNewSessions)
            }
            "EN_CMD_ID_SESSION_SVR_ACK_SESSIONS" => {
                Some(Self::EnCmdIdSessionSvrAckSessions)
            }
            "EN_CMD_ID_SESSION_SVR_UPDATE_ACK" => Some(Self::EnCmdIdSessionSvrUpdateAck),
            "EN_CMD_ID_SESSION_SVR_SET_TOP" => Some(Self::EnCmdIdSessionSvrSetTop),
            "EN_CMD_ID_SESSION_SVR_REMOVE_SESSION" => {
                Some(Self::EnCmdIdSessionSvrRemoveSession)
            }
            "EN_CMD_ID_SESSION_SVR_SINGLE_UNREAD" => {
                Some(Self::EnCmdIdSessionSvrSingleUnread)
            }
            "EN_CMD_ID_SESSION_SVR_MY_GROUP_UNREAD" => {
                Some(Self::EnCmdIdSessionSvrMyGroupUnread)
            }
            "EN_CMD_ID_SESSION_SVR_UPDATE_UNFLW_READ" => {
                Some(Self::EnCmdIdSessionSvrUpdateUnflwRead)
            }
            "EN_CMD_ID_SESSION_SVR_GROUP_ASSIS_MSG" => {
                Some(Self::EnCmdIdSessionSvrGroupAssisMsg)
            }
            "EN_CMD_ID_SESSION_SVR_ACK_ASSIS_MSG" => {
                Some(Self::EnCmdIdSessionSvrAckAssisMsg)
            }
            "EN_CMD_ID_SESSION_SVR_SESSION_DETAIL" => {
                Some(Self::EnCmdIdSessionSvrSessionDetail)
            }
            "EN_CMD_ID_SESSION_SVR_BATCH_SESS_DETAIL" => {
                Some(Self::EnCmdIdSessionSvrBatchSessDetail)
            }
            "EN_CMD_ID_SESSION_SVR_BATCH_RM_SESSIONS" => {
                Some(Self::EnCmdIdSessionSvrBatchRmSessions)
            }
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EnumBizMsgType {
    ///
    BizMsgTypeNormal = 0,
    ///
    BizMsgTypeCardVideo = 1,
}
impl EnumBizMsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EnumBizMsgType::BizMsgTypeNormal => "BIZ_MSG_TYPE_NORMAL",
            EnumBizMsgType::BizMsgTypeCardVideo => "BIZ_MSG_TYPE_CARD_VIDEO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BIZ_MSG_TYPE_NORMAL" => Some(Self::BizMsgTypeNormal),
            "BIZ_MSG_TYPE_CARD_VIDEO" => Some(Self::BizMsgTypeCardVideo),
            _ => None,
        }
    }
}
/// 消息来源
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgSource {
    ///
    EnMsgSourceUnkonw = 0,
    ///
    EnMsgSourceIos = 1,
    ///
    EnMsgSourceAndriod = 2,
    ///
    EnMsgSourceH5 = 3,
    ///
    EnMsgSourcePc = 4,
    ///
    EnMsgSourceBackstage = 5,
    ///
    EnMsgSourceBiz = 6,
    ///
    EnMsgSourceWeb = 7,
    ///
    EnMsgSourceAutoreplyByFollowed = 8,
    ///
    EnMsgSourceAutoreplyByReceiveMsg = 9,
    ///
    EnMsgSourceAutoreplyByKeywords = 10,
    ///
    EnMsgSourceAutoreplyByVoyage = 11,
    ///
    EnMsgSourceVcAttachMsg = 12,
}
impl MsgSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MsgSource::EnMsgSourceUnkonw => "EN_MSG_SOURCE_UNKONW",
            MsgSource::EnMsgSourceIos => "EN_MSG_SOURCE_IOS",
            MsgSource::EnMsgSourceAndriod => "EN_MSG_SOURCE_ANDRIOD",
            MsgSource::EnMsgSourceH5 => "EN_MSG_SOURCE_H5",
            MsgSource::EnMsgSourcePc => "EN_MSG_SOURCE_PC",
            MsgSource::EnMsgSourceBackstage => "EN_MSG_SOURCE_BACKSTAGE",
            MsgSource::EnMsgSourceBiz => "EN_MSG_SOURCE_BIZ",
            MsgSource::EnMsgSourceWeb => "EN_MSG_SOURCE_WEB",
            MsgSource::EnMsgSourceAutoreplyByFollowed => {
                "EN_MSG_SOURCE_AUTOREPLY_BY_FOLLOWED"
            }
            MsgSource::EnMsgSourceAutoreplyByReceiveMsg => {
                "EN_MSG_SOURCE_AUTOREPLY_BY_RECEIVE_MSG"
            }
            MsgSource::EnMsgSourceAutoreplyByKeywords => {
                "EN_MSG_SOURCE_AUTOREPLY_BY_KEYWORDS"
            }
            MsgSource::EnMsgSourceAutoreplyByVoyage => {
                "EN_MSG_SOURCE_AUTOREPLY_BY_VOYAGE"
            }
            MsgSource::EnMsgSourceVcAttachMsg => "EN_MSG_SOURCE_VC_ATTACH_MSG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EN_MSG_SOURCE_UNKONW" => Some(Self::EnMsgSourceUnkonw),
            "EN_MSG_SOURCE_IOS" => Some(Self::EnMsgSourceIos),
            "EN_MSG_SOURCE_ANDRIOD" => Some(Self::EnMsgSourceAndriod),
            "EN_MSG_SOURCE_H5" => Some(Self::EnMsgSourceH5),
            "EN_MSG_SOURCE_PC" => Some(Self::EnMsgSourcePc),
            "EN_MSG_SOURCE_BACKSTAGE" => Some(Self::EnMsgSourceBackstage),
            "EN_MSG_SOURCE_BIZ" => Some(Self::EnMsgSourceBiz),
            "EN_MSG_SOURCE_WEB" => Some(Self::EnMsgSourceWeb),
            "EN_MSG_SOURCE_AUTOREPLY_BY_FOLLOWED" => {
                Some(Self::EnMsgSourceAutoreplyByFollowed)
            }
            "EN_MSG_SOURCE_AUTOREPLY_BY_RECEIVE_MSG" => {
                Some(Self::EnMsgSourceAutoreplyByReceiveMsg)
            }
            "EN_MSG_SOURCE_AUTOREPLY_BY_KEYWORDS" => {
                Some(Self::EnMsgSourceAutoreplyByKeywords)
            }
            "EN_MSG_SOURCE_AUTOREPLY_BY_VOYAGE" => {
                Some(Self::EnMsgSourceAutoreplyByVoyage)
            }
            "EN_MSG_SOURCE_VC_ATTACH_MSG" => Some(Self::EnMsgSourceVcAttachMsg),
            _ => None,
        }
    }
}
/// 消息类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    /// 基础消息类型
    ///
    /// 空空的~
    EnInvalidMsgType = 0,
    /// 文本消息
    EnMsgTypeText = 1,
    /// 图片消息
    EnMsgTypePic = 2,
    /// 语音消息
    EnMsgTypeAudio = 3,
    /// 分享消息
    EnMsgTypeShare = 4,
    /// 撤回消息
    EnMsgTypeDrawBack = 5,
    /// 自定义表情
    EnMsgTypeCustomFace = 6,
    /// 分享v2消息
    EnMsgTypeShareV2 = 7,
    /// 系统撤销
    EnMsgTypeSysCancel = 8,
    /// 小程序
    EnMsgTypeMiniProgram = 9,
    /// 扩展消息类型
    ///
    /// 业务通知
    EnMsgTypeNotifyMsg = 10,
    /// 视频卡片
    EnMsgTypeVideoCard = 11,
    /// 专栏卡片
    EnMsgTypeArticleCard = 12,
    /// 图片卡
    EnMsgTypePictureCard = 13,
    /// 异形卡
    EnMsgTypeCommonShareCard = 14,
    ///
    EnMsgTypeBizMsgType = 50,
    ///
    EnMsgTypeModifyMsgType = 51,
    /// 功能类系统消息类型
    ///
    /// 群成员变更
    EnMsgTypeGroupMemberChanged = 101,
    /// 群状态变更
    EnMsgTypeGroupStatusChanged = 102,
    /// 群动态变更
    EnMsgTypeGroupDynamicChanged = 103,
    /// 群列表变更
    EnMsgTypeGroupListChanged = 104,
    /// 好友列表变更
    EmMsgTypeFriendListChanged = 105,
    /// 群详情发生变化
    EnMsgTypeGroupDetailChanged = 106,
    /// 群成员角色发生变化
    EnMsgTypeGroupMemberRoleChanged = 107,
    ///
    EnMsgTypeNoticeWatchList = 108,
    /// 消息系统，收到新的reply
    EnMsgTypeNotifyNewReplyRecieved = 109,
    ///
    EnMsgTypeNotifyNewAtRecieved = 110,
    ///
    EnMsgTypeNotifyNewPraiseRecieved = 111,
    ///
    EnMsgTypeNotifyNewUpRecieved = 112,
    ///
    EnMsgTypeNotifyNewReplyRecievedV2 = 113,
    ///
    EnMsgTypeNotifyNewAtRecievedV2 = 114,
    ///
    EnMsgTypeNotifyNewPraiseRecievedV2 = 115,
    /// 群详情发生变化,多端同步版本需要即时消息，无需落地
    EnMsgTypeGroupDetailChangedMulti = 116,
    /// 群成员角色发生变化,多端同步版本需要即时消息，无需落地
    EnMsgTypeGroupMemberRoleChangedMulti = 117,
    ///
    EnMsgTypeNotifyAntiDisturb = 118,
    /// 系统通知栏消息类型
    ///
    /// 群解散
    EnMsgTypeSysGroupDissolved = 201,
    /// 入群
    EnMsgTypeSysGroupJoined = 202,
    /// 成员主动退群
    EnMsgTypeSysGroupMemberExited = 203,
    /// 房管被撤
    EnMsgTypeSysGroupAdminFired = 204,
    /// 成员被T
    EnMsgTypeSysGroupMemberKicked = 205,
    /// 管理T人
    EnMsgTypeSysGroupAdminKickOff = 206,
    /// 管理上任
    EnMsgTypeSysGroupAdminDuty = 207,
    /// 自动创建
    EnMsgTypeSysGroupAutoCreated = 208,
    /// 好友申请
    EnMsgTypeSysFriendApply = 210,
    /// 好友申请通过
    EnMsgTypeSysFriendApplyAck = 211,
    /// 用户加群申请
    EnMsgTypeSysGroupApplyForJoining = 212,
    /// 通知管理员,有其他管理员已经同意用户加群
    EnMsgTypeSysGroupAdminAcceptedUserApply = 213,
    /// 聊天窗口通知消息类型
    ///
    /// 入群
    EnMsgTypeChatMemberJoined = 301,
    /// 退群
    EnMsgTypeChatMemberExited = 302,
    /// 冻结
    EnMsgTypeChatGroupFreezed = 303,
    /// 解散
    EnMsgTypeChatGroupDissolved = 304,
    /// 开通应援团
    EnMsgTypeChatGroupCreated = 305,
    /// 弹出会话
    EnMsgTypeChatPopupSession = 306,
}
impl MsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MsgType::EnInvalidMsgType => "EN_INVALID_MSG_TYPE",
            MsgType::EnMsgTypeText => "EN_MSG_TYPE_TEXT",
            MsgType::EnMsgTypePic => "EN_MSG_TYPE_PIC",
            MsgType::EnMsgTypeAudio => "EN_MSG_TYPE_AUDIO",
            MsgType::EnMsgTypeShare => "EN_MSG_TYPE_SHARE",
            MsgType::EnMsgTypeDrawBack => "EN_MSG_TYPE_DRAW_BACK",
            MsgType::EnMsgTypeCustomFace => "EN_MSG_TYPE_CUSTOM_FACE",
            MsgType::EnMsgTypeShareV2 => "EN_MSG_TYPE_SHARE_V2",
            MsgType::EnMsgTypeSysCancel => "EN_MSG_TYPE_SYS_CANCEL",
            MsgType::EnMsgTypeMiniProgram => "EN_MSG_TYPE_MINI_PROGRAM",
            MsgType::EnMsgTypeNotifyMsg => "EN_MSG_TYPE_NOTIFY_MSG",
            MsgType::EnMsgTypeVideoCard => "EN_MSG_TYPE_VIDEO_CARD",
            MsgType::EnMsgTypeArticleCard => "EN_MSG_TYPE_ARTICLE_CARD",
            MsgType::EnMsgTypePictureCard => "EN_MSG_TYPE_PICTURE_CARD",
            MsgType::EnMsgTypeCommonShareCard => "EN_MSG_TYPE_COMMON_SHARE_CARD",
            MsgType::EnMsgTypeBizMsgType => "EN_MSG_TYPE_BIZ_MSG_TYPE",
            MsgType::EnMsgTypeModifyMsgType => "EN_MSG_TYPE_MODIFY_MSG_TYPE",
            MsgType::EnMsgTypeGroupMemberChanged => "EN_MSG_TYPE_GROUP_MEMBER_CHANGED",
            MsgType::EnMsgTypeGroupStatusChanged => "EN_MSG_TYPE_GROUP_STATUS_CHANGED",
            MsgType::EnMsgTypeGroupDynamicChanged => "EN_MSG_TYPE_GROUP_DYNAMIC_CHANGED",
            MsgType::EnMsgTypeGroupListChanged => "EN_MSG_TYPE_GROUP_LIST_CHANGED",
            MsgType::EmMsgTypeFriendListChanged => "EM_MSG_TYPE_FRIEND_LIST_CHANGED",
            MsgType::EnMsgTypeGroupDetailChanged => "EN_MSG_TYPE_GROUP_DETAIL_CHANGED",
            MsgType::EnMsgTypeGroupMemberRoleChanged => {
                "EN_MSG_TYPE_GROUP_MEMBER_ROLE_CHANGED"
            }
            MsgType::EnMsgTypeNoticeWatchList => "EN_MSG_TYPE_NOTICE_WATCH_LIST",
            MsgType::EnMsgTypeNotifyNewReplyRecieved => {
                "EN_MSG_TYPE_NOTIFY_NEW_REPLY_RECIEVED"
            }
            MsgType::EnMsgTypeNotifyNewAtRecieved => "EN_MSG_TYPE_NOTIFY_NEW_AT_RECIEVED",
            MsgType::EnMsgTypeNotifyNewPraiseRecieved => {
                "EN_MSG_TYPE_NOTIFY_NEW_PRAISE_RECIEVED"
            }
            MsgType::EnMsgTypeNotifyNewUpRecieved => "EN_MSG_TYPE_NOTIFY_NEW_UP_RECIEVED",
            MsgType::EnMsgTypeNotifyNewReplyRecievedV2 => {
                "EN_MSG_TYPE_NOTIFY_NEW_REPLY_RECIEVED_V2"
            }
            MsgType::EnMsgTypeNotifyNewAtRecievedV2 => {
                "EN_MSG_TYPE_NOTIFY_NEW_AT_RECIEVED_V2"
            }
            MsgType::EnMsgTypeNotifyNewPraiseRecievedV2 => {
                "EN_MSG_TYPE_NOTIFY_NEW_PRAISE_RECIEVED_V2"
            }
            MsgType::EnMsgTypeGroupDetailChangedMulti => {
                "EN_MSG_TYPE_GROUP_DETAIL_CHANGED_MULTI"
            }
            MsgType::EnMsgTypeGroupMemberRoleChangedMulti => {
                "EN_MSG_TYPE_GROUP_MEMBER_ROLE_CHANGED_MULTI"
            }
            MsgType::EnMsgTypeNotifyAntiDisturb => "EN_MSG_TYPE_NOTIFY_ANTI_DISTURB",
            MsgType::EnMsgTypeSysGroupDissolved => "EN_MSG_TYPE_SYS_GROUP_DISSOLVED",
            MsgType::EnMsgTypeSysGroupJoined => "EN_MSG_TYPE_SYS_GROUP_JOINED",
            MsgType::EnMsgTypeSysGroupMemberExited => {
                "EN_MSG_TYPE_SYS_GROUP_MEMBER_EXITED"
            }
            MsgType::EnMsgTypeSysGroupAdminFired => "EN_MSG_TYPE_SYS_GROUP_ADMIN_FIRED",
            MsgType::EnMsgTypeSysGroupMemberKicked => {
                "EN_MSG_TYPE_SYS_GROUP_MEMBER_KICKED"
            }
            MsgType::EnMsgTypeSysGroupAdminKickOff => {
                "EN_MSG_TYPE_SYS_GROUP_ADMIN_KICK_OFF"
            }
            MsgType::EnMsgTypeSysGroupAdminDuty => "EN_MSG_TYPE_SYS_GROUP_ADMIN_DUTY",
            MsgType::EnMsgTypeSysGroupAutoCreated => "EN_MSG_TYPE_SYS_GROUP_AUTO_CREATED",
            MsgType::EnMsgTypeSysFriendApply => "EN_MSG_TYPE_SYS_FRIEND_APPLY",
            MsgType::EnMsgTypeSysFriendApplyAck => "EN_MSG_TYPE_SYS_FRIEND_APPLY_ACK",
            MsgType::EnMsgTypeSysGroupApplyForJoining => {
                "EN_MSG_TYPE_SYS_GROUP_APPLY_FOR_JOINING"
            }
            MsgType::EnMsgTypeSysGroupAdminAcceptedUserApply => {
                "EN_MSG_TYPE_SYS_GROUP_ADMIN_ACCEPTED_USER_APPLY"
            }
            MsgType::EnMsgTypeChatMemberJoined => "EN_MSG_TYPE_CHAT_MEMBER_JOINED",
            MsgType::EnMsgTypeChatMemberExited => "EN_MSG_TYPE_CHAT_MEMBER_EXITED",
            MsgType::EnMsgTypeChatGroupFreezed => "EN_MSG_TYPE_CHAT_GROUP_FREEZED",
            MsgType::EnMsgTypeChatGroupDissolved => "EN_MSG_TYPE_CHAT_GROUP_DISSOLVED",
            MsgType::EnMsgTypeChatGroupCreated => "EN_MSG_TYPE_CHAT_GROUP_CREATED",
            MsgType::EnMsgTypeChatPopupSession => "EN_MSG_TYPE_CHAT_POPUP_SESSION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EN_INVALID_MSG_TYPE" => Some(Self::EnInvalidMsgType),
            "EN_MSG_TYPE_TEXT" => Some(Self::EnMsgTypeText),
            "EN_MSG_TYPE_PIC" => Some(Self::EnMsgTypePic),
            "EN_MSG_TYPE_AUDIO" => Some(Self::EnMsgTypeAudio),
            "EN_MSG_TYPE_SHARE" => Some(Self::EnMsgTypeShare),
            "EN_MSG_TYPE_DRAW_BACK" => Some(Self::EnMsgTypeDrawBack),
            "EN_MSG_TYPE_CUSTOM_FACE" => Some(Self::EnMsgTypeCustomFace),
            "EN_MSG_TYPE_SHARE_V2" => Some(Self::EnMsgTypeShareV2),
            "EN_MSG_TYPE_SYS_CANCEL" => Some(Self::EnMsgTypeSysCancel),
            "EN_MSG_TYPE_MINI_PROGRAM" => Some(Self::EnMsgTypeMiniProgram),
            "EN_MSG_TYPE_NOTIFY_MSG" => Some(Self::EnMsgTypeNotifyMsg),
            "EN_MSG_TYPE_VIDEO_CARD" => Some(Self::EnMsgTypeVideoCard),
            "EN_MSG_TYPE_ARTICLE_CARD" => Some(Self::EnMsgTypeArticleCard),
            "EN_MSG_TYPE_PICTURE_CARD" => Some(Self::EnMsgTypePictureCard),
            "EN_MSG_TYPE_COMMON_SHARE_CARD" => Some(Self::EnMsgTypeCommonShareCard),
            "EN_MSG_TYPE_BIZ_MSG_TYPE" => Some(Self::EnMsgTypeBizMsgType),
            "EN_MSG_TYPE_MODIFY_MSG_TYPE" => Some(Self::EnMsgTypeModifyMsgType),
            "EN_MSG_TYPE_GROUP_MEMBER_CHANGED" => Some(Self::EnMsgTypeGroupMemberChanged),
            "EN_MSG_TYPE_GROUP_STATUS_CHANGED" => Some(Self::EnMsgTypeGroupStatusChanged),
            "EN_MSG_TYPE_GROUP_DYNAMIC_CHANGED" => {
                Some(Self::EnMsgTypeGroupDynamicChanged)
            }
            "EN_MSG_TYPE_GROUP_LIST_CHANGED" => Some(Self::EnMsgTypeGroupListChanged),
            "EM_MSG_TYPE_FRIEND_LIST_CHANGED" => Some(Self::EmMsgTypeFriendListChanged),
            "EN_MSG_TYPE_GROUP_DETAIL_CHANGED" => Some(Self::EnMsgTypeGroupDetailChanged),
            "EN_MSG_TYPE_GROUP_MEMBER_ROLE_CHANGED" => {
                Some(Self::EnMsgTypeGroupMemberRoleChanged)
            }
            "EN_MSG_TYPE_NOTICE_WATCH_LIST" => Some(Self::EnMsgTypeNoticeWatchList),
            "EN_MSG_TYPE_NOTIFY_NEW_REPLY_RECIEVED" => {
                Some(Self::EnMsgTypeNotifyNewReplyRecieved)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_AT_RECIEVED" => {
                Some(Self::EnMsgTypeNotifyNewAtRecieved)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_PRAISE_RECIEVED" => {
                Some(Self::EnMsgTypeNotifyNewPraiseRecieved)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_UP_RECIEVED" => {
                Some(Self::EnMsgTypeNotifyNewUpRecieved)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_REPLY_RECIEVED_V2" => {
                Some(Self::EnMsgTypeNotifyNewReplyRecievedV2)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_AT_RECIEVED_V2" => {
                Some(Self::EnMsgTypeNotifyNewAtRecievedV2)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_PRAISE_RECIEVED_V2" => {
                Some(Self::EnMsgTypeNotifyNewPraiseRecievedV2)
            }
            "EN_MSG_TYPE_GROUP_DETAIL_CHANGED_MULTI" => {
                Some(Self::EnMsgTypeGroupDetailChangedMulti)
            }
            "EN_MSG_TYPE_GROUP_MEMBER_ROLE_CHANGED_MULTI" => {
                Some(Self::EnMsgTypeGroupMemberRoleChangedMulti)
            }
            "EN_MSG_TYPE_NOTIFY_ANTI_DISTURB" => Some(Self::EnMsgTypeNotifyAntiDisturb),
            "EN_MSG_TYPE_SYS_GROUP_DISSOLVED" => Some(Self::EnMsgTypeSysGroupDissolved),
            "EN_MSG_TYPE_SYS_GROUP_JOINED" => Some(Self::EnMsgTypeSysGroupJoined),
            "EN_MSG_TYPE_SYS_GROUP_MEMBER_EXITED" => {
                Some(Self::EnMsgTypeSysGroupMemberExited)
            }
            "EN_MSG_TYPE_SYS_GROUP_ADMIN_FIRED" => {
                Some(Self::EnMsgTypeSysGroupAdminFired)
            }
            "EN_MSG_TYPE_SYS_GROUP_MEMBER_KICKED" => {
                Some(Self::EnMsgTypeSysGroupMemberKicked)
            }
            "EN_MSG_TYPE_SYS_GROUP_ADMIN_KICK_OFF" => {
                Some(Self::EnMsgTypeSysGroupAdminKickOff)
            }
            "EN_MSG_TYPE_SYS_GROUP_ADMIN_DUTY" => Some(Self::EnMsgTypeSysGroupAdminDuty),
            "EN_MSG_TYPE_SYS_GROUP_AUTO_CREATED" => {
                Some(Self::EnMsgTypeSysGroupAutoCreated)
            }
            "EN_MSG_TYPE_SYS_FRIEND_APPLY" => Some(Self::EnMsgTypeSysFriendApply),
            "EN_MSG_TYPE_SYS_FRIEND_APPLY_ACK" => Some(Self::EnMsgTypeSysFriendApplyAck),
            "EN_MSG_TYPE_SYS_GROUP_APPLY_FOR_JOINING" => {
                Some(Self::EnMsgTypeSysGroupApplyForJoining)
            }
            "EN_MSG_TYPE_SYS_GROUP_ADMIN_ACCEPTED_USER_APPLY" => {
                Some(Self::EnMsgTypeSysGroupAdminAcceptedUserApply)
            }
            "EN_MSG_TYPE_CHAT_MEMBER_JOINED" => Some(Self::EnMsgTypeChatMemberJoined),
            "EN_MSG_TYPE_CHAT_MEMBER_EXITED" => Some(Self::EnMsgTypeChatMemberExited),
            "EN_MSG_TYPE_CHAT_GROUP_FREEZED" => Some(Self::EnMsgTypeChatGroupFreezed),
            "EN_MSG_TYPE_CHAT_GROUP_DISSOLVED" => Some(Self::EnMsgTypeChatGroupDissolved),
            "EN_MSG_TYPE_CHAT_GROUP_CREATED" => Some(Self::EnMsgTypeChatGroupCreated),
            "EN_MSG_TYPE_CHAT_POPUP_SESSION" => Some(Self::EnMsgTypeChatPopupSession),
            _ => None,
        }
    }
}
/// 接收方类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RecverType {
    ///
    EnNoMeaning = 0,
    /// 单人
    EnRecverTypePeer = 1,
    /// 群
    EnRecverTypeGroup = 2,
    /// 多人
    EnRecverTypePeers = 3,
}
impl RecverType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RecverType::EnNoMeaning => "EN_NO_MEANING",
            RecverType::EnRecverTypePeer => "EN_RECVER_TYPE_PEER",
            RecverType::EnRecverTypeGroup => "EN_RECVER_TYPE_GROUP",
            RecverType::EnRecverTypePeers => "EN_RECVER_TYPE_PEERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EN_NO_MEANING" => Some(Self::EnNoMeaning),
            "EN_RECVER_TYPE_PEER" => Some(Self::EnRecverTypePeer),
            "EN_RECVER_TYPE_GROUP" => Some(Self::EnRecverTypeGroup),
            "EN_RECVER_TYPE_PEERS" => Some(Self::EnRecverTypePeers),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RelationLogType {
    ///
    EnInvalidLogType = 0,
    /// 添加好友
    EnAddFriend = 1,
    /// 删除好友
    EnRemoveFriend = 2,
    /// 加入群
    EnJoinGroup = 3,
    /// 退出群
    EnExitGroup = 4,
}
impl RelationLogType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RelationLogType::EnInvalidLogType => "EN_INVALID_LOG_TYPE",
            RelationLogType::EnAddFriend => "EN_ADD_FRIEND",
            RelationLogType::EnRemoveFriend => "EN_REMOVE_FRIEND",
            RelationLogType::EnJoinGroup => "EN_JOIN_GROUP",
            RelationLogType::EnExitGroup => "EN_EXIT_GROUP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EN_INVALID_LOG_TYPE" => Some(Self::EnInvalidLogType),
            "EN_ADD_FRIEND" => Some(Self::EnAddFriend),
            "EN_REMOVE_FRIEND" => Some(Self::EnRemoveFriend),
            "EN_JOIN_GROUP" => Some(Self::EnJoinGroup),
            "EN_EXIT_GROUP" => Some(Self::EnExitGroup),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionType {
    ///
    InvalidSessionType = 0,
    ///
    UnFoldSession = 1,
    ///
    UnFollowSingleSession = 2,
    ///
    MyGroupSession = 3,
    ///
    AllSession = 4,
}
impl SessionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SessionType::InvalidSessionType => "INVALID_SESSION_TYPE",
            SessionType::UnFoldSession => "UN_FOLD_SESSION",
            SessionType::UnFollowSingleSession => "UN_FOLLOW_SINGLE_SESSION",
            SessionType::MyGroupSession => "MY_GROUP_SESSION",
            SessionType::AllSession => "ALL_SESSION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_SESSION_TYPE" => Some(Self::InvalidSessionType),
            "UN_FOLD_SESSION" => Some(Self::UnFoldSession),
            "UN_FOLLOW_SINGLE_SESSION" => Some(Self::UnFollowSingleSession),
            "MY_GROUP_SESSION" => Some(Self::MyGroupSession),
            "ALL_SESSION" => Some(Self::AllSession),
            _ => None,
        }
    }
}
