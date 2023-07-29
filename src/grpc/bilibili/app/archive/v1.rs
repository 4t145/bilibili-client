/// 稿件基本信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arc {
    /// 稿件avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 稿件分P数
    #[prost(int64, tag = "2")]
    pub videos: i64,
    /// 分区id
    #[prost(int32, tag = "3")]
    pub type_id: i32,
    /// 二级分区名
    #[prost(string, tag = "4")]
    pub type_name: ::prost::alloc::string::String,
    /// 稿件类型
    /// 1:原创 2:转载
    #[prost(int32, tag = "5")]
    pub copyright: i32,
    /// 稿件封面url
    #[prost(string, tag = "6")]
    pub pic: ::prost::alloc::string::String,
    /// 稿件标题
    #[prost(string, tag = "7")]
    pub title: ::prost::alloc::string::String,
    /// 稿件发布时间
    #[prost(int64, tag = "8")]
    pub pubdate: i64,
    /// 用户投稿时间
    #[prost(int64, tag = "9")]
    pub ctime: i64,
    /// 稿件简介
    #[prost(string, tag = "10")]
    pub desc: ::prost::alloc::string::String,
    /// 稿件状态
    #[prost(int32, tag = "11")]
    pub state: i32,
    /// 访问属性
    /// 0:全部可见 10000:登录可见
    #[prost(int32, tag = "12")]
    pub access: i32,
    /// 属性位配置(现在无了)
    #[prost(int32, tag = "13")]
    pub attribute: i32,
    /// 空
    #[prost(string, tag = "14")]
    pub tag: ::prost::alloc::string::String,
    /// 空
    #[prost(string, repeated, tag = "15")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 稿件总时长(单位为秒)
    #[prost(int64, tag = "16")]
    pub duration: i64,
    /// 参与的活动id
    #[prost(int64, tag = "17")]
    pub mission_id: i64,
    /// 绑定的商单id
    #[prost(int64, tag = "18")]
    pub order_id: i64,
    /// PGC稿件强制重定向url(如番剧、影视)
    #[prost(string, tag = "19")]
    pub redirect_url: ::prost::alloc::string::String,
    /// 空
    #[prost(int64, tag = "20")]
    pub forward: i64,
    /// 控制标志
    #[prost(message, optional, tag = "21")]
    pub rights: ::core::option::Option<Rights>,
    /// UP主信息
    #[prost(message, optional, tag = "22")]
    pub author: ::core::option::Option<Author>,
    /// 状态数
    #[prost(message, optional, tag = "23")]
    pub stat: ::core::option::Option<Stat>,
    /// 空
    #[prost(string, tag = "24")]
    pub report_result: ::prost::alloc::string::String,
    /// 投稿时发送的动态内容
    #[prost(string, tag = "25")]
    pub dynamic: ::prost::alloc::string::String,
    /// 稿件1P cid
    #[prost(int64, tag = "26")]
    pub first_cid: i64,
    /// 稿件1P 分辨率
    #[prost(message, optional, tag = "27")]
    pub dimension: ::core::option::Option<Dimension>,
    /// 合作组成员列表
    #[prost(message, repeated, tag = "28")]
    pub staff_info: ::prost::alloc::vec::Vec<StaffInfo>,
    /// UGC合集id
    #[prost(int64, tag = "29")]
    pub season_id: i64,
    /// 新版属性位配置(也没用)
    #[prost(int64, tag = "30")]
    pub attribute_v2: i64,
    ///
    #[prost(message, optional, tag = "31")]
    pub season_theme: ::core::option::Option<SeasonTheme>,
    ///
    #[prost(string, tag = "40")]
    pub short_link_v2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "41")]
    pub up_from_v2: i32,
    ///
    #[prost(string, tag = "42")]
    pub first_frame: ::prost::alloc::string::String,
}
/// UP主信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Author {
    /// UP主mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// UP主昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// UP主头像url
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
}
/// 分辨率
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dimension {
    /// 宽度
    #[prost(int64, tag = "1")]
    pub width: i64,
    /// 高度
    #[prost(int64, tag = "2")]
    pub height: i64,
    /// 方向
    /// 0:横屏 1:竖屏
    #[prost(int64, tag = "3")]
    pub rotate: i64,
}
/// 分P信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    /// 视频cid
    #[prost(int64, tag = "1")]
    pub cid: i64,
    /// 分P序号
    #[prost(int32, tag = "2")]
    pub page: i32,
    /// 源类型
    /// vupload:B站 qq:腾讯 hunan:芒果
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// 分P标题
    #[prost(string, tag = "4")]
    pub part: ::prost::alloc::string::String,
    /// 分P时长(单位为秒)
    #[prost(int64, tag = "5")]
    pub duration: i64,
    /// 外链vid
    #[prost(string, tag = "6")]
    pub vid: ::prost::alloc::string::String,
    /// 分P简介
    #[prost(string, tag = "7")]
    pub desc: ::prost::alloc::string::String,
    /// 外链url
    #[prost(string, tag = "8")]
    pub web_link: ::prost::alloc::string::String,
    /// 分P分辨率
    #[prost(message, optional, tag = "9")]
    pub dimension: ::core::option::Option<Dimension>,
    ///
    #[prost(string, tag = "10")]
    pub first_frame: ::prost::alloc::string::String,
}
/// 稿件控制标志
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rights {
    /// 老版是否付费
    #[prost(int32, tag = "1")]
    pub bp: i32,
    /// 允许充电
    #[prost(int32, tag = "2")]
    pub elec: i32,
    /// 允许下载
    #[prost(int32, tag = "3")]
    pub download: i32,
    /// 是否电影
    #[prost(int32, tag = "4")]
    pub movie: i32,
    /// PGC稿件需要付费
    #[prost(int32, tag = "5")]
    pub pay: i32,
    /// 是否高码率
    #[prost(int32, tag = "6")]
    pub hd5: i32,
    /// 是否显示“禁止转载”标志
    #[prost(int32, tag = "7")]
    pub no_reprint: i32,
    /// 是否允许自动播放
    #[prost(int32, tag = "8")]
    pub autoplay: i32,
    /// UGC稿件需要付费(旧版)
    #[prost(int32, tag = "9")]
    pub ugc_pay: i32,
    /// 是否联合投稿
    #[prost(int32, tag = "10")]
    pub is_cooperation: i32,
    /// 是否UGC付费预览
    #[prost(int32, tag = "11")]
    pub ugc_pay_preview: i32,
    /// 是否禁止后台播放
    #[prost(int32, tag = "12")]
    pub no_background: i32,
    /// UGC稿件需要付费
    #[prost(int32, tag = "13")]
    pub arc_pay: i32,
    /// 是否已付费可自由观看
    #[prost(int32, tag = "14")]
    pub pay_free_watch: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonTheme {
    ///
    #[prost(string, tag = "1")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub selected_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text_color: ::prost::alloc::string::String,
}
/// 合作成员信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaffInfo {
    /// 成员mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 成员角色
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 属性位
    /// 0:普通 1:赞助商金色标志
    #[prost(int64, tag = "3")]
    pub attribute: i64,
}
/// 状态数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stat {
    /// 稿件avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 播放数(当屏蔽时为-1)
    #[prost(int32, tag = "2")]
    pub view: i32,
    /// 弹幕数
    #[prost(int32, tag = "3")]
    pub danmaku: i32,
    /// 评论数
    #[prost(int32, tag = "4")]
    pub reply: i32,
    /// 收藏数
    #[prost(int32, tag = "5")]
    pub fav: i32,
    /// 投币数
    #[prost(int32, tag = "6")]
    pub coin: i32,
    /// 分享数
    #[prost(int32, tag = "7")]
    pub share: i32,
    /// 当前排名
    #[prost(int32, tag = "8")]
    pub now_rank: i32,
    /// 历史最高排名
    #[prost(int32, tag = "9")]
    pub his_rank: i32,
    /// 点赞数
    #[prost(int32, tag = "10")]
    pub like: i32,
    /// 点踩数(前端不可见故恒为0)
    #[prost(int32, tag = "11")]
    pub dislike: i32,
}
