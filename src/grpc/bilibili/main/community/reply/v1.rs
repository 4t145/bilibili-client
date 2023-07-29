/// 活动
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Activity {
    /// 活动id
    #[prost(int64, tag = "1")]
    pub activity_id: i64,
    /// 活动状态
    /// -1:待审 1:上线
    #[prost(int64, tag = "2")]
    pub activity_state: i64,
    /// 参与活动的输入框文案
    #[prost(string, tag = "3")]
    pub activity_placeholder: ::prost::alloc::string::String,
}
/// 文章项目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArticleSearchItem {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// UP主昵称
    #[prost(string, tag = "2")]
    pub up_nickname: ::prost::alloc::string::String,
    /// 封面
    #[prost(string, repeated, tag = "3")]
    pub covers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 评论at用户搜索组
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtGroup {
    /// 组类型
    #[prost(int32, tag = "1")]
    pub group_type: i32,
    /// 组标题
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    /// 评论at用户搜索列表
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<AtItem>,
}
/// 评论at用户搜索条目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtItem {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 用户名
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 用户头像url
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    /// 用户是否关注
    #[prost(int32, tag = "4")]
    pub fans: i32,
    /// 用户认证类型
    #[prost(int32, tag = "5")]
    pub official_verify_type: i32,
}
/// 评论at用户搜索-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtSearchReply {
    /// 评论at用户搜索组
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<AtGroup>,
}
/// 评论at用户搜索-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtSearchReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 关键字
    #[prost(string, tag = "2")]
    pub keyword: ::prost::alloc::string::String,
}
/// 广告
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cm {
    /// 广告数据(需要解包)
    #[prost(message, optional, tag = "1")]
    pub source_content: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Any,
    >,
}
/// 评论主体信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    /// 评论文本
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// 需要渲染的用户转义
    #[prost(map = "string, message", tag = "2")]
    pub menber: ::std::collections::HashMap<::prost::alloc::string::String, Member>,
    /// 需要渲染的表情转义
    #[prost(map = "string, message", tag = "3")]
    pub emote: ::std::collections::HashMap<::prost::alloc::string::String, Emote>,
    /// 需要高亮的话题转义
    #[prost(map = "string, message", tag = "4")]
    pub topic: ::std::collections::HashMap<::prost::alloc::string::String, Topic>,
    /// 需要高亮的超链转义
    #[prost(map = "string, message", tag = "5")]
    pub url: ::std::collections::HashMap<::prost::alloc::string::String, Url>,
    /// 投票信息
    #[prost(message, optional, tag = "6")]
    pub vote: ::core::option::Option<Vote>,
    /// at到的用户mid列表
    #[prost(map = "string, int64", tag = "7")]
    pub at_name_to_mid: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    /// 富文本
    #[prost(message, optional, tag = "8")]
    pub rich_text: ::core::option::Option<RichText>,
    /// 评论图片
    #[prost(message, repeated, tag = "9")]
    pub pictures: ::prost::alloc::vec::Vec<Picture>,
}
/// 图片信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Picture {
    /// 图片URL
    #[prost(string, tag = "1")]
    pub img_src: ::prost::alloc::string::String,
    /// 图片宽度
    #[prost(double, tag = "2")]
    pub img_width: f64,
    /// 图片高度
    #[prost(double, tag = "3")]
    pub img_height: f64,
    /// 图片大小，单位KB
    #[prost(double, tag = "4")]
    pub img_size: f64,
}
/// 页面游标回复
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CursorReply {
    /// 下页数据
    #[prost(int64, tag = "1")]
    pub next: i64,
    /// 上页数据
    #[prost(int64, tag = "2")]
    pub prev: i64,
    /// 是否到顶
    #[prost(bool, tag = "3")]
    pub is_begin: bool,
    /// 是否到底
    #[prost(bool, tag = "4")]
    pub is_end: bool,
    /// 排序方式
    /// 2:时间 3:热度
    #[prost(enumeration = "Mode", tag = "5")]
    pub mode: i32,
    /// 当前排序mode在切换按钮上的展示文案
    #[prost(string, tag = "6")]
    pub mode_text: ::prost::alloc::string::String,
}
/// 页面游标请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CursorReq {
    /// 下页数据
    #[prost(int64, tag = "1")]
    pub next: i64,
    /// 上页数据
    #[prost(int64, tag = "2")]
    pub prev: i64,
    /// 排序方式
    #[prost(enumeration = "Mode", tag = "4")]
    pub mode: i32,
}
/// 二级评论明细-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailListReply {
    /// 页面游标
    #[prost(message, optional, tag = "1")]
    pub cursor: ::core::option::Option<CursorReply>,
    /// 评论区显示控制字段
    #[prost(message, optional, tag = "2")]
    pub subject_control: ::core::option::Option<SubjectControl>,
    /// 根评论信息(带二级评论)
    #[prost(message, optional, tag = "3")]
    pub root: ::core::option::Option<ReplyInfo>,
    /// 评论区的活动
    #[prost(message, optional, tag = "4")]
    pub activity: ::core::option::Option<Activity>,
    ///
    #[prost(message, optional, tag = "5")]
    pub likes: ::core::option::Option<LikeInfo>,
}
/// 二级评论明细-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailListReq {
    /// 目标评论区id
    #[prost(int64, tag = "1")]
    pub oid: i64,
    /// 目标评论区业务type
    #[prost(int64, tag = "2")]
    pub r#type: i64,
    /// 根评论rpid
    #[prost(int64, tag = "3")]
    pub root: i64,
    /// 目标评论rpid
    #[prost(int64, tag = "4")]
    pub rpid: i64,
    /// 页面游标
    #[prost(message, optional, tag = "5")]
    pub cursor: ::core::option::Option<CursorReq>,
    /// 来源标识
    #[prost(enumeration = "DetailListScene", tag = "6")]
    pub scene: i32,
}
/// 对话评论树-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DialogListReply {
    /// 页面游标
    #[prost(message, optional, tag = "1")]
    pub cursor: ::core::option::Option<CursorReply>,
    /// 评论区显示控制字段
    #[prost(message, optional, tag = "2")]
    pub subject_control: ::core::option::Option<SubjectControl>,
    /// 子评论列表
    #[prost(message, repeated, tag = "3")]
    pub replies: ::prost::alloc::vec::Vec<ReplyInfo>,
    /// 评论区的活动
    #[prost(message, optional, tag = "4")]
    pub activity: ::core::option::Option<Activity>,
}
/// 对话评论树-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DialogListReq {
    /// 目标评论区id
    #[prost(int64, tag = "1")]
    pub oid: i64,
    /// 目标评论区业务type
    #[prost(int64, tag = "2")]
    pub r#type: i64,
    /// 根评论rpid
    #[prost(int64, tag = "3")]
    pub root: i64,
    /// 对话评论rpid
    #[prost(int64, tag = "4")]
    pub rpid: i64,
    /// 页面游标
    #[prost(message, optional, tag = "5")]
    pub cursor: ::core::option::Option<CursorReq>,
}
/// 特效
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Effects {
    ///
    #[prost(string, tag = "1")]
    pub preloading: ::prost::alloc::string::String,
}
/// 表情项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Emote {
    /// 表情大小
    /// 1:小 2:大
    #[prost(int64, tag = "1")]
    pub size: i64,
    /// 表情url
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub jump_title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub id: i64,
    ///
    #[prost(int64, tag = "6")]
    pub package_id: i64,
    ///
    #[prost(string, tag = "7")]
    pub gif_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub text: ::prost::alloc::string::String,
}
/// 商品项目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoodsSearchItem {
    /// 商品id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 商品名
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 价钱
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    /// 收入
    #[prost(string, tag = "4")]
    pub income: ::prost::alloc::string::String,
    /// 图片
    #[prost(string, tag = "5")]
    pub img: ::prost::alloc::string::String,
    /// 标签
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeInfo {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<like_info::Item>,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LikeInfo`.
pub mod like_info {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        ///
        #[prost(message, optional, tag = "1")]
        pub member: ::core::option::Option<super::Member>,
    }
}
/// 抽奖
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lottery {
    /// 抽奖id
    #[prost(int64, tag = "1")]
    pub lottery_id: i64,
    /// 抽奖状态
    /// 0:未开奖 1:开奖中 2:已开奖
    #[prost(int64, tag = "2")]
    pub lottery_status: i64,
    /// 抽奖人mid
    #[prost(int64, tag = "3")]
    pub lottery_mid: i64,
    /// 开奖时间
    #[prost(int64, tag = "4")]
    pub lottery_time: i64,
    ///
    #[prost(int64, tag = "5")]
    pub oid: i64,
    ///
    #[prost(int64, tag = "6")]
    pub r#type: i64,
    /// 发送时间
    #[prost(int64, tag = "7")]
    pub ctime: i64,
    /// 抽奖评论正文
    #[prost(message, optional, tag = "8")]
    pub content: ::core::option::Option<Content>,
    /// 用户信息
    #[prost(message, optional, tag = "9")]
    pub member: ::core::option::Option<Member>,
    /// 评论条目控制字段
    #[prost(message, optional, tag = "10")]
    pub reply_control: ::core::option::Option<ReplyControl>,
}
/// 主评论列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MainListReply {
    /// 页面游标
    #[prost(message, optional, tag = "1")]
    pub cursor: ::core::option::Option<CursorReply>,
    /// 评论列表
    #[prost(message, repeated, tag = "2")]
    pub replies: ::prost::alloc::vec::Vec<ReplyInfo>,
    /// 评论区显示控制字段
    #[prost(message, optional, tag = "3")]
    pub subject_control: ::core::option::Option<SubjectControl>,
    /// UP置顶评论
    #[prost(message, optional, tag = "4")]
    pub up_top: ::core::option::Option<ReplyInfo>,
    /// 管理员置顶评论
    #[prost(message, optional, tag = "5")]
    pub admin_top: ::core::option::Option<ReplyInfo>,
    /// 投票置顶评论
    #[prost(message, optional, tag = "6")]
    pub vote_top: ::core::option::Option<ReplyInfo>,
    /// 评论区提示
    #[prost(message, optional, tag = "7")]
    pub notice: ::core::option::Option<Notice>,
    /// 抽奖评论
    #[prost(message, optional, tag = "8")]
    pub lottery: ::core::option::Option<Lottery>,
    /// 活动
    #[prost(message, optional, tag = "9")]
    pub activity: ::core::option::Option<Activity>,
    /// 精选评论区筛选后台信息
    #[prost(message, optional, tag = "10")]
    pub up_selection: ::core::option::Option<UpSelection>,
    /// 广告
    #[prost(message, optional, tag = "11")]
    pub cm: ::core::option::Option<Cm>,
    /// 特效
    #[prost(message, optional, tag = "12")]
    pub effects: ::core::option::Option<Effects>,
    ///
    #[prost(message, optional, tag = "13")]
    pub operation: ::core::option::Option<Operation>,
    ///
    #[prost(message, repeated, tag = "14")]
    pub top_replies: ::prost::alloc::vec::Vec<ReplyInfo>,
    ///
    #[prost(message, optional, tag = "15")]
    pub qoe: ::core::option::Option<QoeInfo>,
    ///
    #[prost(map = "string, int32", tag = "16")]
    pub callbacks: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
/// 主评论列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MainListReq {
    /// 目标评论区id
    #[prost(int64, tag = "1")]
    pub oid: i64,
    /// 目标评论区业务type
    #[prost(int64, tag = "2")]
    pub r#type: i64,
    /// 页面游标
    #[prost(message, optional, tag = "3")]
    pub cursor: ::core::option::Option<CursorReq>,
    /// 扩展数据json
    #[prost(string, tag = "4")]
    pub extra: ::prost::alloc::string::String,
    /// 广告扩展
    #[prost(string, tag = "5")]
    pub ad_extra: ::prost::alloc::string::String,
    /// 目标评论rpid
    #[prost(int64, tag = "6")]
    pub rpid: i64,
    ///
    #[prost(int64, tag = "7")]
    pub seek_rpid: i64,
    /// 评论区筛选类型 取值可为: ["全部" "粉丝评论" "笔记长评"]
    #[prost(string, tag = "8")]
    pub filter_tag_name: ::prost::alloc::string::String,
}
/// 用户信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Member {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 性别
    #[prost(string, tag = "3")]
    pub sex: ::prost::alloc::string::String,
    /// 头像url
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    /// 等级
    #[prost(int64, tag = "5")]
    pub level: i64,
    /// 认证类型
    ///
    /// *********VIP相关*********
    #[prost(int64, tag = "6")]
    pub official_verify_type: i64,
    /// 会员类型
    /// 0:不是大会员 1:月度会员 2:年度大会员
    #[prost(int64, tag = "7")]
    pub vip_type: i64,
    /// 会员状态
    #[prost(int64, tag = "8")]
    pub vip_status: i64,
    /// 会员样式
    #[prost(int64, tag = "9")]
    pub vip_theme_type: i64,
    /// 会员铭牌样式url
    ///
    /// *********装扮相关*********
    #[prost(string, tag = "10")]
    pub vip_label_path: ::prost::alloc::string::String,
    /// 头像框url
    #[prost(string, tag = "11")]
    pub garb_pendant_image: ::prost::alloc::string::String,
    /// 装扮卡url
    #[prost(string, tag = "12")]
    pub garb_card_image: ::prost::alloc::string::String,
    /// 有关注按钮时的装扮卡url
    #[prost(string, tag = "13")]
    pub garb_card_image_with_focus: ::prost::alloc::string::String,
    /// 专属装扮页面url
    #[prost(string, tag = "14")]
    pub garb_card_jump_url: ::prost::alloc::string::String,
    /// 专属装扮id
    #[prost(string, tag = "15")]
    pub garb_card_number: ::prost::alloc::string::String,
    /// 专属装扮id显示颜色
    #[prost(string, tag = "16")]
    pub garb_card_fan_color: ::prost::alloc::string::String,
    /// 是否为专属装扮卡
    ///
    /// *********粉丝勋章相关*********
    #[prost(bool, tag = "17")]
    pub garb_card_is_fan: bool,
    /// 粉丝勋章名
    #[prost(string, tag = "18")]
    pub fans_medal_name: ::prost::alloc::string::String,
    /// 粉丝勋章等级
    #[prost(int64, tag = "19")]
    pub fans_medal_level: i64,
    /// 粉丝勋章显示颜色
    #[prost(int64, tag = "20")]
    pub fans_medal_color: i64,
    /// 会员昵称颜色
    #[prost(string, tag = "21")]
    pub vip_nickname_color: ::prost::alloc::string::String,
    /// 会员角标
    /// 0:无角标 1:粉色大会员角标 2:绿色小会员角标
    #[prost(int32, tag = "22")]
    pub vip_avatar_subscript: i32,
    /// 会员标签文
    #[prost(string, tag = "23")]
    pub vip_label_text: ::prost::alloc::string::String,
    /// 会员标颜色
    #[prost(string, tag = "24")]
    pub vip_label_theme: ::prost::alloc::string::String,
    /// 粉丝勋章底色
    #[prost(int64, tag = "25")]
    pub fans_medal_color_end: i64,
    /// 粉丝勋章边框颜色
    #[prost(int64, tag = "26")]
    pub fans_medal_color_border: i64,
    /// 粉丝勋章名颜色
    #[prost(int64, tag = "27")]
    pub fans_medal_color_name: i64,
    /// 粉丝勋章等级颜色
    #[prost(int64, tag = "28")]
    pub fans_medal_color_level: i64,
    ///
    #[prost(int64, tag = "29")]
    pub fans_guard_level: i64,
    ///
    #[prost(int32, tag = "30")]
    pub face_nft: i32,
    /// 是否NFT头像
    #[prost(int32, tag = "31")]
    pub face_nft_new: i32,
    /// 是否为硬核会员
    #[prost(int32, tag = "32")]
    pub is_senior_member: i32,
    /// NFT信息
    #[prost(message, optional, tag = "33")]
    pub nft_interaction: ::core::option::Option<member::NftInteraction>,
    ///
    #[prost(string, tag = "34")]
    pub fans_guard_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "35")]
    pub fans_honor_icon: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Member`.
pub mod member {
    /// NFT地区
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Region {
        /// 地区类型
        #[prost(enumeration = "RegionType", tag = "1")]
        pub r#type: i32,
        /// 角标url
        #[prost(string, tag = "2")]
        pub icon: ::prost::alloc::string::String,
        ///
        #[prost(enumeration = "ShowStatus", tag = "3")]
        pub show_status: i32,
    }
    /// NFT信息
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NftInteraction {
        ///
        #[prost(string, tag = "1")]
        pub itype: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "2")]
        pub metadata_url: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "3")]
        pub nft_id: ::prost::alloc::string::String,
        /// NFT地区
        #[prost(message, optional, tag = "4")]
        pub region: ::core::option::Option<Region>,
    }
    /// 地区类型
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RegionType {
        /// 默认
        Default = 0,
        /// 大陆地区
        Mainland = 1,
        ///
        Gat = 2,
    }
    impl RegionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RegionType::Default => "DEFAULT",
                RegionType::Mainland => "MAINLAND",
                RegionType::Gat => "GAT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "MAINLAND" => Some(Self::Mainland),
                "GAT" => Some(Self::Gat),
                _ => None,
            }
        }
    }
    ///
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ShowStatus {
        /// 默认
        Showdefault = 0,
        ///
        Zoominmainland = 1,
        ///
        Raw = 2,
    }
    impl ShowStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ShowStatus::Showdefault => "SHOWDEFAULT",
                ShowStatus::Zoominmainland => "ZOOMINMAINLAND",
                ShowStatus::Raw => "RAW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SHOWDEFAULT" => Some(Self::Showdefault),
                "ZOOMINMAINLAND" => Some(Self::Zoominmainland),
                "RAW" => Some(Self::Raw),
                _ => None,
            }
        }
    }
}
/// 用户信息V2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberV2 {
    /// 基本信息
    #[prost(message, optional, tag = "1")]
    pub basic: ::core::option::Option<member_v2::Basic>,
    /// 认证信息
    #[prost(message, optional, tag = "2")]
    pub official: ::core::option::Option<member_v2::Official>,
    /// 大会员信息
    #[prost(message, optional, tag = "3")]
    pub vip: ::core::option::Option<member_v2::Vip>,
    /// 装扮信息
    #[prost(message, optional, tag = "4")]
    pub garb: ::core::option::Option<member_v2::Garb>,
    /// 粉丝勋章信息
    #[prost(message, optional, tag = "5")]
    pub medal: ::core::option::Option<member_v2::Medal>,
    /// NFT信息
    #[prost(message, optional, tag = "6")]
    pub nft: ::core::option::Option<member_v2::Nft>,
    /// 硬核会员信息
    #[prost(message, optional, tag = "7")]
    pub senior: ::core::option::Option<member_v2::Senior>,
    /// 契约信息
    #[prost(message, optional, tag = "8")]
    pub contractor: ::core::option::Option<member_v2::Contractor>,
}
/// Nested message and enum types in `MemberV2`.
pub mod member_v2 {
    /// 基本信息
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Basic {
        /// 用户mid
        #[prost(int64, tag = "1")]
        pub mid: i64,
        /// 昵称
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        /// 性别
        #[prost(string, tag = "3")]
        pub sex: ::prost::alloc::string::String,
        /// 头像url
        #[prost(string, tag = "4")]
        pub face: ::prost::alloc::string::String,
        /// 等级
        #[prost(int64, tag = "5")]
        pub level: i64,
    }
    /// 认证
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Official {
        /// 认证类型
        #[prost(int64, tag = "1")]
        pub verify_type: i64,
    }
    /// 大会员
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Vip {
        /// 会员类型
        /// 0:不是大会员 1:月度会员 2:年度大会员
        #[prost(int64, tag = "1")]
        pub r#type: i64,
        /// 会员状态
        #[prost(int64, tag = "2")]
        pub status: i64,
        /// 会员样式
        #[prost(int64, tag = "3")]
        pub theme_type: i64,
        /// 会员铭牌样式url
        #[prost(string, tag = "4")]
        pub label_path: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "5")]
        pub nickname_color: ::prost::alloc::string::String,
        ///
        #[prost(int32, tag = "6")]
        pub avatar_subscript: i32,
        ///
        #[prost(string, tag = "7")]
        pub label_text: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "8")]
        pub vip_label_theme: ::prost::alloc::string::String,
    }
    /// 装扮
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Garb {
        /// 头像框url
        #[prost(string, tag = "1")]
        pub pendant_image: ::prost::alloc::string::String,
        /// 装扮卡url
        #[prost(string, tag = "2")]
        pub card_image: ::prost::alloc::string::String,
        /// 有关注按钮时的装扮卡url
        #[prost(string, tag = "3")]
        pub card_image_with_focus: ::prost::alloc::string::String,
        /// 专属装扮页面url
        #[prost(string, tag = "4")]
        pub card_jump_url: ::prost::alloc::string::String,
        /// 专属装扮id
        #[prost(string, tag = "5")]
        pub card_number: ::prost::alloc::string::String,
        /// 专属装扮id显示颜色
        #[prost(string, tag = "6")]
        pub card_fan_color: ::prost::alloc::string::String,
        /// 是否为专属装扮卡
        #[prost(bool, tag = "7")]
        pub card_is_fan: bool,
    }
    /// 粉丝勋章
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Medal {
        /// 粉丝勋章名
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// 粉丝勋章等级
        #[prost(int64, tag = "2")]
        pub level: i64,
        /// 粉丝勋章显示颜色
        #[prost(int64, tag = "3")]
        pub color_start: i64,
        /// 粉丝勋章底色
        #[prost(int64, tag = "4")]
        pub color_end: i64,
        /// 粉丝勋章边框颜色
        #[prost(int64, tag = "5")]
        pub color_border: i64,
        /// 粉丝勋章名颜色
        #[prost(int64, tag = "6")]
        pub color_name: i64,
        /// 粉丝勋章等级颜色
        #[prost(int64, tag = "7")]
        pub color_level: i64,
        ///
        #[prost(int64, tag = "8")]
        pub guard_level: i64,
        ///
        #[prost(string, tag = "9")]
        pub first_icon: ::prost::alloc::string::String,
        ///
        #[prost(int64, tag = "11")]
        pub level_bg_color: i64,
    }
    /// NFT地区
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Region {
        /// 地区类型
        #[prost(enumeration = "RegionType", tag = "1")]
        pub r#type: i32,
        /// 角标url
        #[prost(string, tag = "2")]
        pub icon: ::prost::alloc::string::String,
        ///
        #[prost(enumeration = "ShowStatus", tag = "3")]
        pub show_status: i32,
    }
    /// NFT信息
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Interaction {
        ///
        #[prost(string, tag = "1")]
        pub itype: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "2")]
        pub metadata_url: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "3")]
        pub nft_id: ::prost::alloc::string::String,
        /// NFT地区
        #[prost(message, optional, tag = "4")]
        pub region: ::core::option::Option<Region>,
    }
    /// NFT
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Nft {
        ///
        #[prost(int32, tag = "1")]
        pub face: i32,
        ///
        #[prost(message, optional, tag = "2")]
        pub interaction: ::core::option::Option<Interaction>,
    }
    /// 硬核会员
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Senior {
        /// 是否为硬核会员
        #[prost(int32, tag = "1")]
        pub is_senior_member: i32,
    }
    /// 契约
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contractor {
        /// 是否和up签订契约
        #[prost(bool, tag = "1")]
        pub is_contractor: bool,
        /// 契约显示文案
        #[prost(string, tag = "2")]
        pub contract_desc: ::prost::alloc::string::String,
    }
    /// 地区类型
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RegionType {
        /// 默认
        Default = 0,
        /// 大陆地区
        Mainland = 1,
        ///
        Gat = 2,
    }
    impl RegionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RegionType::Default => "DEFAULT",
                RegionType::Mainland => "MAINLAND",
                RegionType::Gat => "GAT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "MAINLAND" => Some(Self::Mainland),
                "GAT" => Some(Self::Gat),
                _ => None,
            }
        }
    }
    ///
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ShowStatus {
        ///
        Showdefault = 0,
        ///
        Zoominmainland = 1,
        ///
        Raw = 2,
    }
    impl ShowStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ShowStatus::Showdefault => "SHOWDEFAULT",
                ShowStatus::Zoominmainland => "ZOOMINMAINLAND",
                ShowStatus::Raw => "RAW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SHOWDEFAULT" => Some(Self::Showdefault),
                "ZOOMINMAINLAND" => Some(Self::Zoominmainland),
                "RAW" => Some(Self::Raw),
                _ => None,
            }
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notice {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub id: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub title: ::core::option::Option<OperationTitle>,
    ///
    #[prost(message, optional, tag = "4")]
    pub subtitle: ::core::option::Option<OperationTitle>,
    ///
    #[prost(string, tag = "5")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub report_extra: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationTitle {
    ///
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub is_highlight: bool,
}
/// PGC视频项目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PgcVideoSearchItem {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 类别
    #[prost(string, tag = "2")]
    pub category: ::prost::alloc::string::String,
    /// 封面
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
}
/// 评论区预览-回复
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewListReply {
    /// 页面游标
    #[prost(message, optional, tag = "1")]
    pub cursor: ::core::option::Option<CursorReply>,
    /// 评论列表
    #[prost(message, repeated, tag = "2")]
    pub replies: ::prost::alloc::vec::Vec<ReplyInfo>,
    /// 评论区显示控制字段
    #[prost(message, optional, tag = "3")]
    pub subject_control: ::core::option::Option<SubjectControl>,
    /// UP置顶评论
    #[prost(message, optional, tag = "4")]
    pub up_top: ::core::option::Option<ReplyInfo>,
    /// 管理员置顶评论
    #[prost(message, optional, tag = "5")]
    pub admin_top: ::core::option::Option<ReplyInfo>,
    /// 投票置顶评论
    #[prost(message, optional, tag = "6")]
    pub vote_top: ::core::option::Option<ReplyInfo>,
}
/// 评论区预览-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewListReq {
    /// 目标评论区id
    #[prost(int64, tag = "1")]
    pub oid: i64,
    /// 目标评论区业务type
    #[prost(int64, tag = "2")]
    pub r#type: i64,
    /// 页面游标
    #[prost(message, optional, tag = "3")]
    pub cursor: ::core::option::Option<CursorReq>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QoeInfo {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "3")]
    pub style: i32,
    ///
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub feedback_title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "6")]
    pub score_items: ::prost::alloc::vec::Vec<QoeScoreItem>,
    ///
    #[prost(int64, tag = "7")]
    pub display_rank: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QoeScoreItem {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(float, tag = "3")]
    pub score: f32,
}
/// 评论条目标签信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyCardLabel {
    /// 标签文本
    #[prost(string, tag = "1")]
    pub text_content: ::prost::alloc::string::String,
    /// 文本颜色
    #[prost(string, tag = "2")]
    pub text_color_day: ::prost::alloc::string::String,
    /// 文本颜色夜间
    #[prost(string, tag = "3")]
    pub text_color_night: ::prost::alloc::string::String,
    /// 标签颜色
    #[prost(string, tag = "4")]
    pub label_color_day: ::prost::alloc::string::String,
    /// 标签颜色夜间
    #[prost(string, tag = "5")]
    pub label_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub image: ::prost::alloc::string::String,
    /// 标签类型 0:UP主觉得很赞 1:妙评
    #[prost(int32, tag = "7")]
    pub r#type: i32,
    /// 背景url
    #[prost(string, tag = "8")]
    pub background: ::prost::alloc::string::String,
    /// 背景宽
    #[prost(double, tag = "9")]
    pub background_width: f64,
    /// 背景高
    #[prost(double, tag = "10")]
    pub background_height: f64,
    /// 点击跳转url
    #[prost(string, tag = "11")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "12")]
    pub effect: i64,
    ///
    #[prost(int64, tag = "13")]
    pub effect_start_time: i64,
}
/// 评论条目控制字段
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyControl {
    /// 操作行为标志
    /// 0:无 1:已点赞 2:已点踩
    #[prost(int64, tag = "1")]
    pub action: i64,
    /// 是否UP觉得很赞
    #[prost(bool, tag = "2")]
    pub up_like: bool,
    /// 是否存在UP回复
    #[prost(bool, tag = "3")]
    pub up_reply: bool,
    /// 是否显示关注按钮
    #[prost(bool, tag = "4")]
    pub show_follow_btn: bool,
    /// 是否协管
    #[prost(bool, tag = "5")]
    pub is_assist: bool,
    /// 是否展示标签
    #[prost(string, tag = "6")]
    pub label_text: ::prost::alloc::string::String,
    /// 是否关注
    #[prost(bool, tag = "7")]
    pub following: bool,
    /// 是否粉丝
    #[prost(bool, tag = "8")]
    pub followed: bool,
    /// 是否被自己拉黑
    #[prost(bool, tag = "9")]
    pub blocked: bool,
    /// 是否存在折叠的二级评论
    #[prost(bool, tag = "10")]
    pub has_folded_reply: bool,
    /// 是否折叠
    #[prost(bool, tag = "11")]
    pub is_folded_reply: bool,
    /// 是否UP置顶
    #[prost(bool, tag = "12")]
    pub is_up_top: bool,
    /// 是否管理置顶
    #[prost(bool, tag = "13")]
    pub is_admin_top: bool,
    /// 是否置顶投票评论
    #[prost(bool, tag = "14")]
    pub is_vote_top: bool,
    /// 最大收起显示行数
    #[prost(int64, tag = "15")]
    pub max_line: i64,
    /// 该条评论可不可见
    #[prost(bool, tag = "16")]
    pub invisible: bool,
    /// 是否和up签订契约
    #[prost(bool, tag = "17")]
    pub is_contractor: bool,
    /// 是否是笔记评论
    #[prost(bool, tag = "18")]
    pub is_note: bool,
    /// 评论条目标签列表
    #[prost(message, repeated, tag = "19")]
    pub card_labels: ::prost::alloc::vec::Vec<ReplyCardLabel>,
    /// 子评论数文案 "共x条回复"
    #[prost(string, tag = "20")]
    pub sub_reply_entry_text: ::prost::alloc::string::String,
    /// 子评论数文案 "相关回复共x条"
    #[prost(string, tag = "21")]
    pub sub_reply_title_text: ::prost::alloc::string::String,
    /// 契约显示文案
    #[prost(string, tag = "22")]
    pub contract_desc: ::prost::alloc::string::String,
    /// 发布时间文案 "x天前发布"
    #[prost(string, tag = "23")]
    pub time_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "24")]
    pub biz_scene: ::prost::alloc::string::String,
    /// IP属地信息 "IP属地：xxx"
    #[prost(string, tag = "25")]
    pub location: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyExtra {
    ///
    #[prost(int64, tag = "1")]
    pub season_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub season_type: i64,
    ///
    #[prost(int64, tag = "3")]
    pub ep_id: i64,
    ///
    #[prost(bool, tag = "4")]
    pub is_story: bool,
}
/// 评论条目信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyInfo {
    /// 二级评论列表
    #[prost(message, repeated, tag = "1")]
    pub replies: ::prost::alloc::vec::Vec<ReplyInfo>,
    /// 评论rpid
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// 评论区对象id
    #[prost(int64, tag = "3")]
    pub oid: i64,
    /// 评论区类型
    #[prost(int64, tag = "4")]
    pub r#type: i64,
    /// 发布者UID
    #[prost(int64, tag = "5")]
    pub mid: i64,
    /// 根评论rpid
    #[prost(int64, tag = "6")]
    pub root: i64,
    /// 父评论rpid
    #[prost(int64, tag = "7")]
    pub parent: i64,
    /// 对话评论rpid
    #[prost(int64, tag = "8")]
    pub dialog: i64,
    /// 点赞数
    #[prost(int64, tag = "9")]
    pub like: i64,
    /// 发布时间
    #[prost(int64, tag = "10")]
    pub ctime: i64,
    /// 回复数
    #[prost(int64, tag = "11")]
    pub count: i64,
    /// 评论主体信息
    #[prost(message, optional, tag = "12")]
    pub content: ::core::option::Option<Content>,
    /// 发布者信息
    #[prost(message, optional, tag = "13")]
    pub member: ::core::option::Option<Member>,
    /// 评论控制字段
    #[prost(message, optional, tag = "14")]
    pub reply_control: ::core::option::Option<ReplyControl>,
    /// 发布者信息V2
    #[prost(message, optional, tag = "15")]
    pub member_v2: ::core::option::Option<MemberV2>,
}
/// 查询单条评论-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyInfoReply {
    /// 评论条目信息
    #[prost(message, optional, tag = "1")]
    pub reply: ::core::option::Option<ReplyInfo>,
}
/// 查询单条评论-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyInfoReq {
    /// 评论rpid
    #[prost(int64, tag = "1")]
    pub rpid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub scene: i32,
}
/// 富文本
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RichText {
    /// 富文本类型
    #[prost(oneof = "rich_text::Item", tags = "1")]
    pub item: ::core::option::Option<rich_text::Item>,
}
/// Nested message and enum types in `RichText`.
pub mod rich_text {
    /// 富文本类型
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        /// 笔记
        #[prost(message, tag = "1")]
        Note(super::RichTextNote),
    }
}
/// 笔记
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RichTextNote {
    /// 预览文本
    #[prost(string, tag = "1")]
    pub summary: ::prost::alloc::string::String,
    /// 笔记预览图片url列表
    #[prost(string, repeated, tag = "2")]
    pub images: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 笔记页面url
    #[prost(string, tag = "3")]
    pub click_url: ::prost::alloc::string::String,
    /// 发布日期 YYYY-mm-dd
    #[prost(string, tag = "4")]
    pub last_mtime_text: ::prost::alloc::string::String,
}
/// 评论搜索插入项目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchItem {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// 项目
    #[prost(oneof = "search_item::Item", tags = "2, 3, 4")]
    pub item: ::core::option::Option<search_item::Item>,
}
/// Nested message and enum types in `SearchItem`.
pub mod search_item {
    /// 项目
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        /// 商品
        #[prost(message, tag = "2")]
        Goods(super::GoodsSearchItem),
        /// 视频
        #[prost(message, tag = "3")]
        Video(super::VideoSearchItem),
        /// 专栏
        #[prost(message, tag = "4")]
        Article(super::ArticleSearchItem),
    }
}
/// 评论搜索插入项目响应游标
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchItemCursorReply {
    /// 是否有下一页
    #[prost(bool, tag = "1")]
    pub has_next: bool,
    /// 下页
    #[prost(int64, tag = "2")]
    pub next: i64,
}
/// 评论搜索插入项目请求游标
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchItemCursorReq {
    /// 下一页
    #[prost(int64, tag = "1")]
    pub next: i64,
    /// tab类型
    #[prost(enumeration = "SearchItemType", tag = "2")]
    pub item_type: i32,
}
/// 评论搜索item前置发布-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchItemPreHookReply {
    /// 输入框的文案
    #[prost(string, tag = "1")]
    pub placeholder_text: ::prost::alloc::string::String,
    /// 背景空白的时候的文案
    #[prost(string, tag = "2")]
    pub background_text: ::prost::alloc::string::String,
    /// 有权限的tab栏的顺序
    #[prost(enumeration = "SearchItemType", repeated, tag = "3")]
    pub ordered_type: ::prost::alloc::vec::Vec<i32>,
}
/// 评论搜索item前置发布-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchItemPreHookReq {
    /// 目标评论区id
    #[prost(int64, tag = "1")]
    pub oid: i64,
    /// 目标评论区业务type
    #[prost(int64, tag = "2")]
    pub r#type: i64,
}
/// 评论搜索插入项目-回复
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchItemReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub cursor: ::core::option::Option<SearchItemCursorReply>,
    /// 搜索的结果
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<SearchItem>,
    /// 附加信息
    #[prost(message, optional, tag = "3")]
    pub extra: ::core::option::Option<SearchItemReplyExtraInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchItemReplyExtraInfo {
    ///
    #[prost(string, tag = "1")]
    pub event_id: ::prost::alloc::string::String,
}
/// 评论搜索插入项目-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchItemReq {
    /// 页面游标
    #[prost(message, optional, tag = "1")]
    pub cursor: ::core::option::Option<SearchItemCursorReq>,
    /// 目标评论区id
    #[prost(int64, tag = "2")]
    pub oid: i64,
    /// 目标评论区业务type
    #[prost(int64, tag = "3")]
    pub r#type: i64,
    /// 搜索关键词
    #[prost(string, tag = "4")]
    pub keyword: ::prost::alloc::string::String,
}
/// 评论分享材料-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareRepliesInfoReq {
    /// 评论rpid列表
    #[prost(int64, repeated, tag = "1")]
    pub rpids: ::prost::alloc::vec::Vec<i64>,
    /// 目标评论区id
    #[prost(int64, tag = "2")]
    pub oid: i64,
    /// 目标评论区业务type
    #[prost(int64, tag = "3")]
    pub r#type: i64,
}
/// 评论分享材料-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareRepliesInfoResp {
    /// 评论分享条目列表
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<ShareReplyInfo>,
    /// 源内容标题
    #[prost(string, tag = "2")]
    pub from_title: ::prost::alloc::string::String,
    /// 源内容UP主
    #[prost(string, tag = "3")]
    pub from_up: ::prost::alloc::string::String,
    /// 源内容封面url
    #[prost(string, tag = "4")]
    pub from_pic: ::prost::alloc::string::String,
    /// 源内容页面url
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
    /// logo url
    #[prost(string, tag = "6")]
    pub slogan_pic: ::prost::alloc::string::String,
    /// 标语
    #[prost(string, tag = "7")]
    pub slogan_text: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub topic: ::core::option::Option<ShareReplyTopic>,
    ///
    #[prost(message, optional, tag = "9")]
    pub extra: ::core::option::Option<share_replies_info_resp::ShareExtra>,
}
/// Nested message and enum types in `ShareRepliesInfoResp`.
pub mod share_replies_info_resp {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShareExtra {
        ///
        #[prost(bool, tag = "1")]
        pub is_pgc: bool,
    }
}
/// 评论分享条目信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareReplyInfo {
    /// 用户信息
    #[prost(message, optional, tag = "1")]
    pub member: ::core::option::Option<Member>,
    /// 评论主体信息
    #[prost(message, optional, tag = "2")]
    pub content: ::core::option::Option<Content>,
    /// 分享标题(评论发布者昵称)
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// 分享副标题 "发表了评论"
    #[prost(string, tag = "4")]
    pub sub_title: ::prost::alloc::string::String,
    /// 荣誉信息文案 "获得了up主点赞"
    #[prost(string, tag = "5")]
    pub achievement_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub label_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareReplyTopic {
    ///
    #[prost(message, optional, tag = "1")]
    pub topic: ::core::option::Option<Topic>,
    ///
    #[prost(string, tag = "2")]
    pub origin_text: ::prost::alloc::string::String,
}
/// 评论区控制字段
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectControl {
    /// UP主mid
    #[prost(int64, tag = "1")]
    pub up_mid: i64,
    /// 自己是否为协管
    #[prost(bool, tag = "2")]
    pub is_assist: bool,
    /// 是否只读
    #[prost(bool, tag = "3")]
    pub read_only: bool,
    /// 是否有发起投票权限
    #[prost(bool, tag = "4")]
    pub has_vote_access: bool,
    /// 是否有发起抽奖权限
    #[prost(bool, tag = "5")]
    pub has_lottery_access: bool,
    /// 是否有被折叠评论
    #[prost(bool, tag = "6")]
    pub has_folded_reply: bool,
    /// 空评论区背景文案
    #[prost(string, tag = "7")]
    pub bg_text: ::prost::alloc::string::String,
    /// 是否被UP拉黑
    #[prost(bool, tag = "8")]
    pub up_blocked: bool,
    /// 是否有发起活动权限
    #[prost(bool, tag = "9")]
    pub has_activity_access: bool,
    /// 标题展示控制
    #[prost(bool, tag = "10")]
    pub show_title: bool,
    /// 是否显示UP主操作标志
    #[prost(bool, tag = "11")]
    pub show_up_action: bool,
    /// 是否显示评论区排序切换按钮
    #[prost(int64, tag = "12")]
    pub switcher_type: i64,
    /// 是否禁止输入框
    #[prost(bool, tag = "13")]
    pub input_disable: bool,
    /// 根评论输入框背景文案
    #[prost(string, tag = "14")]
    pub root_text: ::prost::alloc::string::String,
    /// 子评论输入框背景文案
    #[prost(string, tag = "15")]
    pub child_text: ::prost::alloc::string::String,
    /// 评论总数
    #[prost(int64, tag = "16")]
    pub count: i64,
    /// 评论区标题
    #[prost(string, tag = "17")]
    pub title: ::prost::alloc::string::String,
    /// 离开态输入框的文案
    #[prost(string, tag = "18")]
    pub giveup_text: ::prost::alloc::string::String,
    /// 是否允许笔记
    #[prost(bool, tag = "19")]
    pub has_note_access: bool,
    ///
    #[prost(bool, tag = "20")]
    pub disable_jump_emote: bool,
    ///
    #[prost(string, tag = "21")]
    pub empty_background_text_plain: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "22")]
    pub empty_background_text_highlight: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "23")]
    pub empty_background_uri: ::prost::alloc::string::String,
    /// 评论区筛选类型列表
    #[prost(message, repeated, tag = "24")]
    pub support_filter_tags: ::prost::alloc::vec::Vec<subject_control::FilterTag>,
}
/// Nested message and enum types in `SubjectControl`.
pub mod subject_control {
    /// 评论区筛选类型
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FilterTag {
        /// 类型名
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "2")]
        pub event_id: ::prost::alloc::string::String,
    }
}
/// 评论表情推荐列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestEmotesReq {
    /// 目标评论区id
    #[prost(int64, tag = "1")]
    pub oid: i64,
    /// 目标评论区业务type
    #[prost(int64, tag = "2")]
    pub r#type: i64,
}
/// 评论表情推荐列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestEmotesResp {
    /// 表情推荐列表
    #[prost(message, repeated, tag = "1")]
    pub emotes: ::prost::alloc::vec::Vec<Emote>,
}
/// 话题项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    /// 跳转url
    #[prost(string, tag = "1")]
    pub link: ::prost::alloc::string::String,
    /// 话题id
    #[prost(int64, tag = "2")]
    pub id: i64,
}
/// UGC视频项目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UgcVideoSearchItem {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// UP主昵称
    #[prost(string, tag = "2")]
    pub up_nickname: ::prost::alloc::string::String,
    /// 时长(单位为秒)
    #[prost(int64, tag = "3")]
    pub duration: i64,
    /// 封面
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
}
/// 精选评论
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpSelection {
    /// 待审评论数
    #[prost(int64, tag = "1")]
    pub pending_count: i64,
    /// 忽略评论数
    #[prost(int64, tag = "2")]
    pub ignore_count: i64,
}
/// 超链项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Url {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub state: i64,
    /// 图标url
    #[prost(string, tag = "3")]
    pub prefix_icon: ::prost::alloc::string::String,
    /// 客户端内跳转uri
    #[prost(string, tag = "4")]
    pub app_url_schema: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub app_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub app_package_name: ::prost::alloc::string::String,
    /// 点击上报数据
    #[prost(string, tag = "7")]
    pub click_report: ::prost::alloc::string::String,
    /// 是否半屏打开
    #[prost(bool, tag = "8")]
    pub is_half_screen: bool,
    /// 展现上报数据
    #[prost(string, tag = "9")]
    pub exposure_report: ::prost::alloc::string::String,
    /// 扩展字段
    #[prost(message, optional, tag = "10")]
    pub extra: ::core::option::Option<url::Extra>,
    /// 是否下划线
    #[prost(bool, tag = "11")]
    pub underline: bool,
    ///
    #[prost(bool, tag = "12")]
    pub match_once: bool,
    /// 网页url
    #[prost(string, tag = "13")]
    pub pc_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "14")]
    pub icon_position: i32,
}
/// Nested message and enum types in `Url`.
pub mod url {
    /// 扩展字段
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Extra {
        ///
        #[prost(int64, tag = "1")]
        pub goods_item_id: i64,
        ///
        #[prost(string, tag = "2")]
        pub goods_prefetched_cache: ::prost::alloc::string::String,
        ///
        #[prost(int32, tag = "3")]
        pub goods_show_type: i32,
        /// 热词搜索
        #[prost(bool, tag = "4")]
        pub is_word_search: bool,
        ///
        #[prost(int64, tag = "5")]
        pub goods_cm_control: i64,
    }
}
/// 用户回调上报-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCallbackReply {}
/// 用户回调上报-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCallbackReq {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(enumeration = "UserCallbackScene", tag = "2")]
    pub scene: i32,
    ///
    #[prost(enumeration = "UserCallbackAction", tag = "3")]
    pub action: i32,
    /// 目标评论区id
    #[prost(int64, tag = "4")]
    pub oid: i64,
    /// 目标评论区业务type
    #[prost(int64, tag = "5")]
    pub r#type: i64,
}
/// 视频项目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoSearchItem {
    ///
    #[prost(enumeration = "SearchItemVideoSubType", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(oneof = "video_search_item::VideoItem", tags = "2, 3")]
    pub video_item: ::core::option::Option<video_search_item::VideoItem>,
}
/// Nested message and enum types in `VideoSearchItem`.
pub mod video_search_item {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VideoItem {
        /// UGC视频
        #[prost(message, tag = "2")]
        Ugc(super::UgcVideoSearchItem),
        /// PGC视频
        #[prost(message, tag = "3")]
        Pgc(super::PgcVideoSearchItem),
    }
}
/// 投票信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    /// 投票id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 投票标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 参与人数
    #[prost(int64, tag = "3")]
    pub count: i64,
}
/// 来源标识
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DetailListScene {
    /// 评论区展开
    Reply = 0,
    /// 回复消息推送
    MsgFeed = 1,
    ///
    Notify = 2,
}
impl DetailListScene {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DetailListScene::Reply => "REPLY",
            DetailListScene::MsgFeed => "MSG_FEED",
            DetailListScene::Notify => "NOTIFY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REPLY" => Some(Self::Reply),
            "MSG_FEED" => Some(Self::MsgFeed),
            "NOTIFY" => Some(Self::Notify),
            _ => None,
        }
    }
}
/// 排序方式
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Mode {
    ///
    Default = 0,
    /// 默认排序
    Unspecified = 1,
    /// 按时间
    MainListTime = 2,
    /// 按热度
    MainListHot = 3,
}
impl Mode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Mode::Default => "DEFAULT",
            Mode::Unspecified => "UNSPECIFIED",
            Mode::MainListTime => "MAIN_LIST_TIME",
            Mode::MainListHot => "MAIN_LIST_HOT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT" => Some(Self::Default),
            "UNSPECIFIED" => Some(Self::Unspecified),
            "MAIN_LIST_TIME" => Some(Self::MainListTime),
            "MAIN_LIST_HOT" => Some(Self::MainListHot),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchItemType {
    ///
    DefaultItemType = 0,
    ///
    Goods = 1,
    ///
    Video = 2,
    ///
    Article = 3,
}
impl SearchItemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SearchItemType::DefaultItemType => "DEFAULT_ITEM_TYPE",
            SearchItemType::Goods => "GOODS",
            SearchItemType::Video => "VIDEO",
            SearchItemType::Article => "ARTICLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT_ITEM_TYPE" => Some(Self::DefaultItemType),
            "GOODS" => Some(Self::Goods),
            "VIDEO" => Some(Self::Video),
            "ARTICLE" => Some(Self::Article),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchItemVideoSubType {
    ///
    Ugc = 0,
    ///
    Pgc = 1,
}
impl SearchItemVideoSubType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SearchItemVideoSubType::Ugc => "UGC",
            SearchItemVideoSubType::Pgc => "PGC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UGC" => Some(Self::Ugc),
            "PGC" => Some(Self::Pgc),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserCallbackAction {
    ///
    Dismiss = 0,
}
impl UserCallbackAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserCallbackAction::Dismiss => "Dismiss",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Dismiss" => Some(Self::Dismiss),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserCallbackScene {
    ///
    Insert = 0,
}
impl UserCallbackScene {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserCallbackScene::Insert => "Insert",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Insert" => Some(Self::Insert),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod reply_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 评论区
    #[derive(Debug, Clone)]
    pub struct ReplyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReplyClient<tonic::transport::Channel> {
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
    impl<T> ReplyClient<T>
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
        ) -> ReplyClient<InterceptedService<T, F>>
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
            ReplyClient::new(InterceptedService::new(inner, interceptor))
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
        /// 主评论列表接口
        pub async fn main_list(
            &mut self,
            request: impl tonic::IntoRequest<super::MainListReq>,
        ) -> std::result::Result<tonic::Response<super::MainListReply>, tonic::Status> {
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
                "/bilibili.main.community.reply.v1.Reply/MainList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.main.community.reply.v1.Reply", "MainList"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 二级评论明细接口
        pub async fn detail_list(
            &mut self,
            request: impl tonic::IntoRequest<super::DetailListReq>,
        ) -> std::result::Result<
            tonic::Response<super::DetailListReply>,
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
                "/bilibili.main.community.reply.v1.Reply/DetailList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.community.reply.v1.Reply",
                        "DetailList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 对话评论树接口
        pub async fn dialog_list(
            &mut self,
            request: impl tonic::IntoRequest<super::DialogListReq>,
        ) -> std::result::Result<
            tonic::Response<super::DialogListReply>,
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
                "/bilibili.main.community.reply.v1.Reply/DialogList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.community.reply.v1.Reply",
                        "DialogList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 评论预览接口
        pub async fn preview_list(
            &mut self,
            request: impl tonic::IntoRequest<super::PreviewListReq>,
        ) -> std::result::Result<
            tonic::Response<super::PreviewListReply>,
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
                "/bilibili.main.community.reply.v1.Reply/PreviewList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.community.reply.v1.Reply",
                        "PreviewList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 评论搜索item前置发布接口
        pub async fn search_item_pre_hook(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchItemPreHookReq>,
        ) -> std::result::Result<
            tonic::Response<super::SearchItemPreHookReply>,
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
                "/bilibili.main.community.reply.v1.Reply/SearchItemPreHook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.community.reply.v1.Reply",
                        "SearchItemPreHook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 评论搜索插入项目接口
        pub async fn search_item(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchItemReq>,
        ) -> std::result::Result<
            tonic::Response<super::SearchItemReply>,
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
                "/bilibili.main.community.reply.v1.Reply/SearchItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.community.reply.v1.Reply",
                        "SearchItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 评论at用户搜索接口
        pub async fn at_search(
            &mut self,
            request: impl tonic::IntoRequest<super::AtSearchReq>,
        ) -> std::result::Result<tonic::Response<super::AtSearchReply>, tonic::Status> {
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
                "/bilibili.main.community.reply.v1.Reply/AtSearch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.main.community.reply.v1.Reply", "AtSearch"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 查询单条评论接口
        pub async fn reply_info(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplyInfoReq>,
        ) -> std::result::Result<tonic::Response<super::ReplyInfoReply>, tonic::Status> {
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
                "/bilibili.main.community.reply.v1.Reply/ReplyInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.community.reply.v1.Reply",
                        "ReplyInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 用户回调上报接口
        pub async fn user_callback(
            &mut self,
            request: impl tonic::IntoRequest<super::UserCallbackReq>,
        ) -> std::result::Result<
            tonic::Response<super::UserCallbackReply>,
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
                "/bilibili.main.community.reply.v1.Reply/UserCallback",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.community.reply.v1.Reply",
                        "UserCallback",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 评论分享材料接口
        pub async fn share_replies_info(
            &mut self,
            request: impl tonic::IntoRequest<super::ShareRepliesInfoReq>,
        ) -> std::result::Result<
            tonic::Response<super::ShareRepliesInfoResp>,
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
                "/bilibili.main.community.reply.v1.Reply/ShareRepliesInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.community.reply.v1.Reply",
                        "ShareRepliesInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 评论表情推荐列表接口
        pub async fn suggest_emotes(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestEmotesReq>,
        ) -> std::result::Result<
            tonic::Response<super::SuggestEmotesResp>,
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
                "/bilibili.main.community.reply.v1.Reply/SuggestEmotes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.community.reply.v1.Reply",
                        "SuggestEmotes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
