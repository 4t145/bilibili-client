/// 活动皮肤
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalActSkin {
    /// 动画SVGA资源
    #[prost(string, tag = "1")]
    pub svga: ::prost::alloc::string::String,
    /// 动画SVGA最后一帧图片资源
    #[prost(string, tag = "2")]
    pub last_image: ::prost::alloc::string::String,
    /// 动画播放次数
    #[prost(int64, tag = "3")]
    pub play_times: i64,
}
/// 动态-附加卡-按钮
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalButton {
    /// 按钮类型
    #[prost(enumeration = "AddButtonType", tag = "1")]
    pub r#type: i32,
    /// jump-跳转样式
    #[prost(message, optional, tag = "2")]
    pub jump_style: ::core::option::Option<AdditionalButtonStyle>,
    /// jump-跳转链接
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    /// button-未点样式
    #[prost(message, optional, tag = "4")]
    pub uncheck: ::core::option::Option<AdditionalButtonStyle>,
    /// button-已点样式
    #[prost(message, optional, tag = "5")]
    pub check: ::core::option::Option<AdditionalButtonStyle>,
    /// button-当前状态
    #[prost(enumeration = "AdditionalButtonStatus", tag = "6")]
    pub status: i32,
    /// 按钮点击样式
    #[prost(enumeration = "AdditionalButtonClickType", tag = "7")]
    pub click_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalButtonInteractive {
    /// 是否弹窗
    #[prost(string, tag = "1")]
    pub popups: ::prost::alloc::string::String,
    /// 弹窗确认文案
    #[prost(string, tag = "2")]
    pub confirm: ::prost::alloc::string::String,
    /// 弹窗取消文案
    #[prost(string, tag = "3")]
    pub cancel: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalButtonShare {
    ///
    #[prost(int32, tag = "1")]
    pub show: i32,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
}
/// 动态-附加卡-按钮样式
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalButtonStyle {
    /// icon
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 文案
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    /// 按钮点击交互
    #[prost(message, optional, tag = "3")]
    pub interactive: ::core::option::Option<AdditionalButtonInteractive>,
    /// 当前按钮填充样式
    #[prost(enumeration = "AddButtonBgStyle", tag = "4")]
    pub bg_style: i32,
    /// toast文案, 当disable=1时有效
    #[prost(string, tag = "5")]
    pub toast: ::prost::alloc::string::String,
    /// 当前按钮样式,
    /// 0:高亮 1:置灰(按钮不可点击)
    #[prost(enumeration = "DisableState", tag = "6")]
    pub disable: i32,
    ///
    #[prost(message, optional, tag = "7")]
    pub share: ::core::option::Option<AdditionalButtonShare>,
}
/// 动态-附加卡-番剧卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalPgc {
    /// 头部说明文案
    #[prost(string, tag = "1")]
    pub head_text: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 展示图
    #[prost(string, tag = "3")]
    pub image_url: ::prost::alloc::string::String,
    /// 描述文字1
    #[prost(string, tag = "4")]
    pub desc_text_1: ::prost::alloc::string::String,
    /// 描述文字2
    #[prost(string, tag = "5")]
    pub desc_text_2: ::prost::alloc::string::String,
    /// 点击跳转链接
    #[prost(string, tag = "6")]
    pub url: ::prost::alloc::string::String,
    /// 按钮
    #[prost(message, optional, tag = "7")]
    pub button: ::core::option::Option<AdditionalButton>,
    /// 头部icon
    #[prost(string, tag = "8")]
    pub head_icon: ::prost::alloc::string::String,
    /// style
    #[prost(enumeration = "ImageStyle", tag = "9")]
    pub style: i32,
    /// 动态本身的类型 type
    #[prost(string, tag = "10")]
    pub r#type: ::prost::alloc::string::String,
}
/// 动态-附加卡-专栏
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionArticle {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub cover: ::core::option::Option<MdlDynDrawItem>,
    ///
    #[prost(string, tag = "3")]
    pub desc_text_left: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc_text_right: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub card_type: ::prost::alloc::string::String,
}
/// 动态-附加卡-通用卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionCommon {
    /// 头部说明文案
    #[prost(string, tag = "1")]
    pub head_text: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 展示图
    #[prost(string, tag = "3")]
    pub image_url: ::prost::alloc::string::String,
    /// 描述文字1
    #[prost(string, tag = "4")]
    pub desc_text_1: ::prost::alloc::string::String,
    /// 描述文字2
    #[prost(string, tag = "5")]
    pub desc_text_2: ::prost::alloc::string::String,
    /// 点击跳转链接
    #[prost(string, tag = "6")]
    pub url: ::prost::alloc::string::String,
    /// 按钮
    #[prost(message, optional, tag = "7")]
    pub button: ::core::option::Option<AdditionalButton>,
    /// 头部icon
    #[prost(string, tag = "8")]
    pub head_icon: ::prost::alloc::string::String,
    /// style
    #[prost(enumeration = "ImageStyle", tag = "9")]
    pub style: i32,
    /// 动态本身的类型 type
    #[prost(string, tag = "10")]
    pub r#type: ::prost::alloc::string::String,
    /// 附加卡类型
    ///
    /// ogv manga
    #[prost(string, tag = "11")]
    pub card_type: ::prost::alloc::string::String,
}
/// 动态-附加卡-电竞卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionEsport {
    /// 电竞类型
    #[prost(enumeration = "EspaceStyle", tag = "1")]
    pub style: i32,
    /// 动态本身的类型 type
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// 附加卡类型
    ///
    /// ogv manga
    #[prost(string, tag = "4")]
    pub card_type: ::prost::alloc::string::String,
    #[prost(oneof = "addition_esport::Item", tags = "2")]
    pub item: ::core::option::Option<addition_esport::Item>,
}
/// Nested message and enum types in `AdditionEsport`.
pub mod addition_esport {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        /// moba类
        #[prost(message, tag = "2")]
        AdditionEsportMoba(super::AdditionEsportMoba),
    }
}
/// 动态-附加卡-电竞卡-moba类
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionEsportMoba {
    /// 头部说明文案
    #[prost(string, tag = "1")]
    pub head_text: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 战队列表
    #[prost(message, repeated, tag = "3")]
    pub match_team: ::prost::alloc::vec::Vec<MatchTeam>,
    /// 比赛信息
    #[prost(message, optional, tag = "4")]
    pub addition_esport_moba_status: ::core::option::Option<AdditionEsportMobaStatus>,
    /// 卡片跳转
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    /// 按钮
    #[prost(message, optional, tag = "6")]
    pub button: ::core::option::Option<AdditionalButton>,
    /// 副标题
    #[prost(string, tag = "7")]
    pub sub_title: ::prost::alloc::string::String,
    /// 动态本身的类型 type
    #[prost(string, tag = "10")]
    pub r#type: ::prost::alloc::string::String,
    /// 附加卡类型
    #[prost(string, tag = "11")]
    pub card_type: ::prost::alloc::string::String,
    /// 附加卡图标
    #[prost(string, tag = "12")]
    pub head_icon: ::prost::alloc::string::String,
}
/// 动态-附加卡-电竞卡-moba类-比赛信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionEsportMobaStatus {
    /// 文案类
    #[prost(message, repeated, tag = "1")]
    pub addition_esport_moba_status_desc: ::prost::alloc::vec::Vec<
        AdditionEsportMobaStatusDesc,
    >,
    /// 比赛状态文案
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 比赛状态状态
    #[prost(int32, tag = "3")]
    pub status: i32,
    /// 日间色值
    #[prost(string, tag = "4")]
    pub color: ::prost::alloc::string::String,
    /// 夜间色值
    #[prost(string, tag = "5")]
    pub night_color: ::prost::alloc::string::String,
}
/// 动态-附加卡-电竞卡-moba类-比赛信息-文案类
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionEsportMobaStatusDesc {
    /// 文案
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 日间色值
    #[prost(string, tag = "2")]
    pub color: ::prost::alloc::string::String,
    /// 夜间色值
    #[prost(string, tag = "3")]
    pub night_color: ::prost::alloc::string::String,
}
/// 动态-附加卡-商品卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionGoods {
    /// 推荐文案
    #[prost(string, tag = "1")]
    pub rcmd_desc: ::prost::alloc::string::String,
    /// 商品信息
    #[prost(message, repeated, tag = "2")]
    pub goods_items: ::prost::alloc::vec::Vec<GoodsItem>,
    /// 附加卡类型
    #[prost(string, tag = "3")]
    pub card_type: ::prost::alloc::string::String,
    /// 头部icon
    #[prost(string, tag = "4")]
    pub icon: ::prost::alloc::string::String,
    /// 商品附加卡整卡跳转
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    /// 商品类型
    /// 1:淘宝 2:会员购，注：实际是获取的goods_items里面的第一个source_type
    #[prost(int32, tag = "6")]
    pub source_type: i32,
    ///
    #[prost(int32, tag = "7")]
    pub jump_type: i32,
    ///
    #[prost(string, tag = "8")]
    pub app_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub ad_mark_icon: ::prost::alloc::string::String,
}
/// 动态-附加卡-直播附加卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionLiveRoom {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub badge: ::core::option::Option<VideoBadge>,
    ///
    #[prost(message, optional, tag = "4")]
    pub desc_text_upper: ::core::option::Option<CoverIconWithText>,
    ///
    #[prost(string, tag = "5")]
    pub desc_text_lower: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub card_type: ::prost::alloc::string::String,
}
/// 动态-附加卡-UGC视频附加卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionUgc {
    /// 说明文案
    #[prost(string, tag = "1")]
    pub head_text: ::prost::alloc::string::String,
    /// 稿件标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 封面
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    /// 描述文字1
    #[prost(string, tag = "4")]
    pub desc_text_1: ::prost::alloc::string::String,
    /// 描述文字2
    #[prost(string, tag = "5")]
    pub desc_text_2: ::prost::alloc::string::String,
    /// 接秒开
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    /// 时长
    #[prost(string, tag = "7")]
    pub duration: ::prost::alloc::string::String,
    /// 标题支持换行-标题支持单行和双行，本期不支持填充up昵称，支持双行展示，字段默认为true
    #[prost(bool, tag = "8")]
    pub line_feed: bool,
    /// 附加卡类型
    #[prost(string, tag = "9")]
    pub card_type: ::prost::alloc::string::String,
}
/// up主预约发布卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionUp {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 高亮文本，描述文字1
    #[prost(message, optional, tag = "2")]
    pub desc_text_1: ::core::option::Option<HighlightText>,
    /// 描述文字2
    #[prost(string, tag = "3")]
    pub desc_text_2: ::prost::alloc::string::String,
    /// 点击跳转链接
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    /// 按钮
    #[prost(message, optional, tag = "5")]
    pub button: ::core::option::Option<AdditionalButton>,
    /// 附加卡类型
    #[prost(string, tag = "6")]
    pub card_type: ::prost::alloc::string::String,
    /// 预约人数(用于预约人数变化)
    #[prost(int64, tag = "7")]
    pub reserve_total: i64,
    /// 活动皮肤
    #[prost(message, optional, tag = "8")]
    pub act_skin: ::core::option::Option<AdditionalActSkin>,
    /// 预约id
    #[prost(int64, tag = "9")]
    pub rid: i64,
    ///
    #[prost(int32, tag = "10")]
    pub lottery_type: i32,
    ///
    #[prost(message, optional, tag = "11")]
    pub desc_text3: ::core::option::Option<HighlightText>,
    ///
    #[prost(int64, tag = "12")]
    pub up_mid: i64,
    ///
    #[prost(message, optional, tag = "13")]
    pub user_info: ::core::option::Option<AdditionUserInfo>,
    ///
    #[prost(string, tag = "14")]
    pub dynamic_id: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "15")]
    pub show_text2: bool,
    ///
    #[prost(int64, tag = "16")]
    pub dyn_type: i64,
    ///
    #[prost(string, tag = "17")]
    pub business_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "18")]
    pub badge_text: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "19")]
    pub is_premiere: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionUserInfo {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub face: ::prost::alloc::string::String,
}
/// 动态-附加卡-投票
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionVote {
    /// 封面图
    #[prost(string, tag = "1")]
    pub image_url: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 展示项1
    #[prost(string, tag = "3")]
    pub text_1: ::prost::alloc::string::String,
    /// button文案
    #[prost(string, tag = "4")]
    pub button_text: ::prost::alloc::string::String,
    /// 点击跳转链接
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
}
/// 动态模块-投票
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionVote2 {
    /// 投票类型
    #[prost(enumeration = "AdditionVoteType", tag = "1")]
    pub addition_vote_type: i32,
    /// 投票ID
    #[prost(int64, tag = "2")]
    pub vote_id: i64,
    /// 标题
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// 已过期： xxx人参与· 投票已过期。button 展示去查看
    /// 未过期： xxx人参与· 剩xx天xx时xx分。button展示去投票
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// 剩余时间
    #[prost(int64, tag = "5")]
    pub deadline: i64,
    /// 生效文案
    #[prost(string, tag = "6")]
    pub open_text: ::prost::alloc::string::String,
    /// 过期文案
    #[prost(string, tag = "7")]
    pub close_text: ::prost::alloc::string::String,
    /// 已投票
    #[prost(string, tag = "8")]
    pub voted_text: ::prost::alloc::string::String,
    /// 投票状态
    #[prost(enumeration = "AdditionVoteState", tag = "9")]
    pub state: i32,
    /// 业务类型
    /// 0:动态投票 1:话题h5组件
    #[prost(int32, tag = "13")]
    pub biz_type: i32,
    /// 投票总人数
    #[prost(int64, tag = "14")]
    pub total: i64,
    /// 附加卡类型
    #[prost(string, tag = "15")]
    pub card_type: ::prost::alloc::string::String,
    /// 异常提示
    #[prost(string, tag = "16")]
    pub tips: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "17")]
    pub uri: ::prost::alloc::string::String,
    /// 是否投票
    #[prost(bool, tag = "18")]
    pub is_voted: bool,
    /// 投票最多多选个数，单选为1
    #[prost(int32, tag = "19")]
    pub choice_cnt: i32,
    /// 是否默认选中分享到动态
    #[prost(bool, tag = "20")]
    pub defaule_select_share: bool,
    /// 投票信息
    #[prost(oneof = "addition_vote2::Item", tags = "10, 11, 12")]
    pub item: ::core::option::Option<addition_vote2::Item>,
}
/// Nested message and enum types in `AdditionVote2`.
pub mod addition_vote2 {
    /// 投票信息
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        ///
        #[prost(message, tag = "10")]
        AdditionVoteWord(super::AdditionVoteWord),
        ///
        #[prost(message, tag = "11")]
        AdditionVotePic(super::AdditionVotePic),
        ///
        #[prost(message, tag = "12")]
        AdditionVoteDefaule(super::AdditionVoteDefaule),
    }
}
/// 外露投票
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionVoteDefaule {
    /// 图片 多张
    #[prost(string, repeated, tag = "1")]
    pub cover: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 外露图片类型
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionVotePic {
    /// 图片投票详情
    #[prost(message, repeated, tag = "1")]
    pub item: ::prost::alloc::vec::Vec<AdditionVotePicItem>,
}
/// 图片投票详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionVotePicItem {
    /// 选项索引，从1开始
    #[prost(int32, tag = "1")]
    pub opt_idx: i32,
    /// 图片
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    /// 选中状态
    #[prost(bool, tag = "3")]
    pub is_vote: bool,
    /// 人数
    #[prost(int32, tag = "4")]
    pub total: i32,
    /// 占比
    #[prost(double, tag = "5")]
    pub persent: f64,
    /// 标题文案
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    /// 是否投票人数最多的选项
    #[prost(bool, tag = "7")]
    pub is_max_option: bool,
}
/// 外露文字类型
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionVoteWord {
    /// 外露文字投票详情
    #[prost(message, repeated, tag = "1")]
    pub item: ::prost::alloc::vec::Vec<AdditionVoteWordItem>,
}
/// 外露文字投票详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionVoteWordItem {
    /// 选项索引，从1开始
    #[prost(int32, tag = "1")]
    pub opt_idx: i32,
    /// 文案
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 选中状态
    #[prost(bool, tag = "3")]
    pub is_vote: bool,
    /// 人数
    #[prost(int32, tag = "4")]
    pub total: i32,
    /// 占比
    #[prost(double, tag = "5")]
    pub persent: f64,
    /// 是否投票人数最多的选项
    #[prost(bool, tag = "6")]
    pub is_max_option: bool,
}
/// 综合页请求广告所需字段，由客户端-网关透传
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdParam {
    /// 综合页请求广告所需字段，由客户端-网关透传
    #[prost(string, tag = "1")]
    pub ad_extra: ::prost::alloc::string::String,
    /// request_id
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlumniDynamicsReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    ///
    #[prost(string, tag = "2")]
    pub toast: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlumniDynamicsReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub first_time: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int32, tag = "4")]
    pub local_time: i32,
    ///
    #[prost(int32, tag = "5")]
    pub page: i32,
    ///
    #[prost(int32, tag = "6")]
    pub from_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusBannerInfo {
    ///
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub jump_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusBillboardInternalReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "3")]
    pub version_code: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusBillBoardReply {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub help_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub build_time: i64,
    ///
    #[prost(string, tag = "5")]
    pub version_code: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "6")]
    pub list: ::prost::alloc::vec::Vec<OfficialItem>,
    ///
    #[prost(string, tag = "7")]
    pub share_uri: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub bind_notice: i32,
    ///
    #[prost(string, tag = "9")]
    pub update_toast: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub campus_id: i64,
    ///
    #[prost(message, optional, tag = "11")]
    pub open_progress: ::core::option::Option<CampusFeatureProgress>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusBillBoardReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub version_code: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "4")]
    pub from_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusEntryTabReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusEntryTabResp {
    ///
    #[prost(enumeration = "CampusEntryType", tag = "1")]
    pub entry_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusFeatureProgress {
    ///
    #[prost(int64, tag = "1")]
    pub progress_full: i64,
    ///
    #[prost(int64, tag = "2")]
    pub progress_achieved: i64,
    ///
    #[prost(string, tag = "3")]
    pub desc_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc_1: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub btn: ::core::option::Option<CampusLabel>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusFeedbackInfo {
    ///
    #[prost(int32, tag = "1")]
    pub biz_type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub biz_id: i64,
    ///
    #[prost(int64, tag = "3")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "4")]
    pub reason: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusFeedbackReply {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusFeedbackReq {
    ///
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<CampusFeedbackInfo>,
    ///
    #[prost(int32, tag = "2")]
    pub from: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusHomePagesReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub top: ::core::option::Option<CampusRcmdTop>,
    ///
    #[prost(message, optional, tag = "2")]
    pub campus_top: ::core::option::Option<CampusTop>,
    ///
    #[prost(int32, tag = "3")]
    pub page_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusHomePagesReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "3")]
    pub lat: f64,
    ///
    #[prost(double, tag = "4")]
    pub lng: f64,
    ///
    #[prost(message, optional, tag = "5")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int32, tag = "6")]
    pub page_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusHomeRcmdTopic {
    ///
    #[prost(message, optional, tag = "1")]
    pub title: ::core::option::Option<ModuleTitle>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub topic: ::prost::alloc::vec::Vec<TopicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusInfo {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub online: i64,
    ///
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusLabel {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMateLikeListReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<ModuleAuthor>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMateLikeListReq {
    ///
    #[prost(int64, tag = "1")]
    pub dynamic_id: i64,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "2")]
    pub from_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngBadge {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub badge_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub upload_hint_msg: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngBasicInfo {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub hint_msg: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngDetailReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<CampusMngItem>,
    ///
    #[prost(string, tag = "2")]
    pub top_hint_bar_msg: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bottom_submit_hint_msg: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "5")]
    pub campus_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngDetailReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngItem {
    ///
    #[prost(int32, tag = "1")]
    pub audit_status: i32,
    ///
    #[prost(string, tag = "2")]
    pub audit_message: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub item_type: i32,
    ///
    #[prost(string, tag = "4")]
    pub mng_item_id: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "5")]
    pub is_del: bool,
    /// Oneof field:
    #[prost(oneof = "campus_mng_item::Item", tags = "6, 7, 8, 9")]
    pub item: ::core::option::Option<campus_mng_item::Item>,
}
/// Nested message and enum types in `CampusMngItem`.
pub mod campus_mng_item {
    /// Oneof field:
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        ///
        #[prost(message, tag = "6")]
        BasicInfo(super::CampusMngBasicInfo),
        ///
        #[prost(message, tag = "7")]
        Badge(super::CampusMngBadge),
        ///
        #[prost(string, tag = "8")]
        Slogan(::prost::alloc::string::String),
        ///
        #[prost(message, tag = "9")]
        Quiz(super::CampusMngQuiz),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngQuiz {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub more_label: ::core::option::Option<CampusLabel>,
    ///
    #[prost(string, tag = "3")]
    pub add_label: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub submit_label: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub quiz_count: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngQuizDetail {
    ///
    #[prost(int64, tag = "1")]
    pub quiz_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub question: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub correct_answer: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "4")]
    pub wrong_answer_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "5")]
    pub audit_status: i32,
    ///
    #[prost(string, tag = "6")]
    pub audit_message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngQuizOperateReply {
    ///
    #[prost(string, tag = "1")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub quiz: ::prost::alloc::vec::Vec<CampusMngQuizDetail>,
    ///
    #[prost(int64, tag = "3")]
    pub quiz_total: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngQuizOperateReq {
    ///
    #[prost(int32, tag = "1")]
    pub action: i32,
    ///
    #[prost(int64, tag = "2")]
    pub campus_id: i64,
    ///
    #[prost(message, repeated, tag = "3")]
    pub quiz: ::prost::alloc::vec::Vec<CampusMngQuizDetail>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngSlogan {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub slogan: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub input_hint_msg: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngSubmitReply {
    ///
    #[prost(string, tag = "1")]
    pub toast: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusMngSubmitReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub modified_items: ::prost::alloc::vec::Vec<CampusMngItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusNoticeInfo {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub button: ::core::option::Option<CampusLabel>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRcmdFeedReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    ///
    #[prost(string, tag = "2")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub guide_bar: ::core::option::Option<GuideBarInfo>,
    ///
    #[prost(bool, tag = "4")]
    pub has_more: bool,
    ///
    #[prost(bool, tag = "5")]
    pub update: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRcmdFeedReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub first_time: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int32, tag = "4")]
    pub local_time: i32,
    ///
    #[prost(int32, tag = "5")]
    pub page: i32,
    ///
    #[prost(int32, tag = "6")]
    pub scroll: i32,
    ///
    #[prost(string, tag = "7")]
    pub view_dyn_id: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "8")]
    pub from_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRcmdInfo {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<CampusRcmdItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRcmdItem {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<RcmdItem>,
    ///
    #[prost(int64, tag = "3")]
    pub campus_id: i64,
    ///
    #[prost(message, optional, tag = "4")]
    pub entry_label: ::core::option::Option<CampusLabel>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRcmdReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub top: ::core::option::Option<CampusRcmdTop>,
    ///
    #[prost(message, optional, tag = "2")]
    pub rcmd: ::core::option::Option<CampusRcmdInfo>,
    ///
    #[prost(message, optional, tag = "3")]
    pub campus_top: ::core::option::Option<CampusTop>,
    ///
    #[prost(int32, tag = "4")]
    pub page_type: i32,
    ///
    #[prost(int32, tag = "5")]
    pub jump_home_pop: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRcmdReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "3")]
    pub lat: f64,
    ///
    #[prost(double, tag = "4")]
    pub lng: f64,
    ///
    #[prost(message, optional, tag = "5")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "6")]
    pub from_type: i32,
    ///
    #[prost(enumeration = "CampusHomePageType", tag = "7")]
    pub page_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRcmdTop {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub r#type: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub button: ::core::option::Option<RcmdTopButton>,
    ///
    #[prost(message, optional, tag = "7")]
    pub switch_label: ::core::option::Option<CampusLabel>,
    ///
    #[prost(message, optional, tag = "8")]
    pub notice_label: ::core::option::Option<CampusLabel>,
    ///
    #[prost(string, tag = "9")]
    pub desc2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub desc3: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "11")]
    pub invite_label: ::core::option::Option<CampusLabel>,
    ///
    #[prost(message, optional, tag = "12")]
    pub reserve_label: ::core::option::Option<CampusLabel>,
    ///
    #[prost(int64, tag = "13")]
    pub reserve_number: i64,
    ///
    #[prost(int64, tag = "14")]
    pub max_reserve: i64,
    ///
    #[prost(message, optional, tag = "15")]
    pub school_label: ::core::option::Option<CampusLabel>,
    ///
    #[prost(message, optional, tag = "16")]
    pub mng_label: ::core::option::Option<CampusLabel>,
    ///
    #[prost(message, optional, tag = "17")]
    pub rcmd_topic: ::core::option::Option<CampusHomeRcmdTopic>,
    ///
    #[prost(bool, tag = "18")]
    pub audit_before_open: bool,
    ///
    #[prost(string, tag = "19")]
    pub audit_message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRecommendReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<RcmdItem>,
    ///
    #[prost(bool, tag = "2")]
    pub has_more: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRecommendReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub page_no: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(enumeration = "CampusRcmdReqFrom", tag = "4")]
    pub from: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRedDotReply {
    ///
    #[prost(int32, tag = "1")]
    pub red_dot: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusRedDotReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "2")]
    pub from_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusShowTabInfo {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "4")]
    pub red_dot: i32,
    ///
    #[prost(string, tag = "5")]
    pub icon_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusSquareReply {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub list: ::prost::alloc::vec::Vec<RcmdCampusBrief>,
    ///
    #[prost(message, optional, tag = "3")]
    pub button: ::core::option::Option<CampusLabel>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusSquareReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(double, tag = "2")]
    pub lat: f64,
    ///
    #[prost(double, tag = "3")]
    pub lng: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusTop {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub tabs: ::prost::alloc::vec::Vec<CampusShowTabInfo>,
    ///
    #[prost(message, optional, tag = "4")]
    pub switch_label: ::core::option::Option<CampusLabel>,
    ///
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "6")]
    pub banner: ::prost::alloc::vec::Vec<CampusBannerInfo>,
    ///
    #[prost(message, optional, tag = "7")]
    pub invite_label: ::core::option::Option<CampusLabel>,
    ///
    #[prost(message, optional, tag = "8")]
    pub notice: ::core::option::Option<CampusNoticeInfo>,
    ///
    #[prost(message, optional, tag = "9")]
    pub topic_square: ::core::option::Option<TopicSquareInfo>,
    ///
    #[prost(string, tag = "10")]
    pub campus_badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub campus_background: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub campus_motto: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "13")]
    pub mng_entry: ::core::option::Option<CampusLabel>,
    ///
    #[prost(string, tag = "14")]
    pub campus_intro: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "15")]
    pub campus_name_link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "16")]
    pub bottom_left_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusTopicRcmdFeedReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    ///
    #[prost(string, tag = "2")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "3")]
    pub has_more: bool,
    ///
    #[prost(string, tag = "4")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub join_discuss: ::core::option::Option<IconButton>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusTopicRcmdFeedReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int32, tag = "4")]
    pub local_time: i32,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "5")]
    pub from_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardParagraph {
    ///
    #[prost(message, optional, tag = "1")]
    pub additional_card: ::core::option::Option<ModuleAdditional>,
    ///
    #[prost(string, tag = "3")]
    pub biz_id: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "LinkNodeType", tag = "2")]
    pub biz_type: i32,
}
/// 动态卡片列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardVideoDynList {
    /// 动态列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    /// 更新的动态数
    #[prost(int64, tag = "2")]
    pub update_num: i64,
    /// 历史偏移
    #[prost(string, tag = "3")]
    pub history_offset: ::prost::alloc::string::String,
    /// 更新基础信息
    #[prost(string, tag = "4")]
    pub update_baseline: ::prost::alloc::string::String,
    /// 是否还有更多数据
    #[prost(bool, tag = "5")]
    pub has_more: bool,
}
/// 视频页-我的追番
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardVideoFollowList {
    /// 查看全部(跳转链接)
    #[prost(string, tag = "1")]
    pub view_all_link: ::prost::alloc::string::String,
    /// 追番列表
    #[prost(message, repeated, tag = "2")]
    pub list: ::prost::alloc::vec::Vec<FollowListItem>,
}
/// 视频页-最近访问
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardVideoUpList {
    /// 标题展示文案
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// up主列表
    #[prost(message, repeated, tag = "2")]
    pub list: ::prost::alloc::vec::Vec<UpListItem>,
    /// 服务端生成的透传上报字段
    #[prost(string, tag = "3")]
    pub footprint: ::prost::alloc::string::String,
    /// 直播数
    #[prost(int32, tag = "4")]
    pub show_live_num: i32,
    /// 跳转label
    #[prost(message, optional, tag = "5")]
    pub more_label: ::core::option::Option<UpListMoreLabel>,
    /// 标题开关(综合页)
    #[prost(int32, tag = "6")]
    pub title_switch: i32,
    /// 是否展示右上角查看更多label
    #[prost(bool, tag = "7")]
    pub show_more_label: bool,
    /// 是否在快速消费页查看更多按钮
    #[prost(bool, tag = "8")]
    pub show_in_personal: bool,
    /// 是否展示右侧查看更多按钮
    #[prost(bool, tag = "9")]
    pub show_more_button: bool,
    ///
    #[prost(message, repeated, tag = "10")]
    pub list_second: ::prost::alloc::vec::Vec<UpListItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelInfo {
    ///
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub channel_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "4")]
    pub is_atten: bool,
    ///
    #[prost(string, tag = "5")]
    pub type_icon: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "6")]
    pub items: ::prost::alloc::vec::Vec<RcmdItem>,
    ///
    #[prost(string, tag = "7")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub jump_uri: ::prost::alloc::string::String,
}
/// 评论外露展示项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmtShowItem {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub uid: i64,
    /// 用户昵称
    #[prost(string, tag = "2")]
    pub uname: ::prost::alloc::string::String,
    /// 点击跳转链接
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 评论内容
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Colors {
    ///
    #[prost(string, tag = "1")]
    pub color_day: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub color_night: ::prost::alloc::string::String,
}
/// 精选评论区
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentDetail {
    /// 该功能能不能用
    #[prost(bool, tag = "1")]
    pub can_modify: bool,
    /// up关闭评论区功能 1允许关闭 0允许开放
    /// 精选评论区功能 1允许停止评论精选 0允许评论精选
    #[prost(int64, tag = "2")]
    pub status: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    ///
    #[prost(bool, tag = "1")]
    pub story_vertical_exp: bool,
    ///
    #[prost(int64, tag = "2")]
    pub detail_view_bits: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoverIconWithText {
    ///
    #[prost(int32, tag = "1")]
    pub icon: i32,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// 装扮卡片-粉丝勋章信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecoCardFan {
    /// 是否是粉丝
    #[prost(int32, tag = "1")]
    pub is_fan: i32,
    /// 数量
    #[prost(int32, tag = "2")]
    pub number: i32,
    /// 数量 str
    #[prost(string, tag = "3")]
    pub number_str: ::prost::alloc::string::String,
    /// 颜色
    #[prost(string, tag = "4")]
    pub color: ::prost::alloc::string::String,
}
/// 装扮卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecorateCard {
    /// 装扮卡片id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 装扮卡片链接
    #[prost(string, tag = "2")]
    pub card_url: ::prost::alloc::string::String,
    /// 装扮卡片点击跳转链接
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    /// 粉丝样式
    #[prost(message, optional, tag = "4")]
    pub fan: ::core::option::Option<DecoCardFan>,
}
/// 文本描述
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Description {
    /// 文本内容
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 文本类型
    #[prost(enumeration = "DescType", tag = "2")]
    pub r#type: i32,
    /// 点击跳转链接
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// emoji类型
    #[prost(enumeration = "EmojiType", tag = "4")]
    pub emoji_type: i32,
    /// 商品类型
    #[prost(string, tag = "5")]
    pub goods_type: ::prost::alloc::string::String,
    /// 前置Icon
    #[prost(string, tag = "6")]
    pub icon_url: ::prost::alloc::string::String,
    /// icon_name
    #[prost(string, tag = "7")]
    pub icon_name: ::prost::alloc::string::String,
    /// 资源ID
    #[prost(string, tag = "8")]
    pub rid: ::prost::alloc::string::String,
    /// 商品卡特殊字段
    #[prost(message, optional, tag = "9")]
    pub goods: ::core::option::Option<ModuleDescGoods>,
    /// 文本原始文案
    #[prost(string, tag = "10")]
    pub orig_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "11")]
    pub emoji_size: i32,
    ///
    #[prost(message, optional, tag = "12")]
    pub emoji_size_spec: ::core::option::Option<EmojiSizeSpec>,
}
/// 尺寸信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dimension {
    ///
    #[prost(int64, tag = "1")]
    pub height: i64,
    ///
    #[prost(int64, tag = "2")]
    pub width: i64,
    ///
    #[prost(int64, tag = "3")]
    pub rotate: i64,
}
/// 动态通用附加卡-follow/取消follow-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynAdditionCommonFollowReply {
    ///
    #[prost(enumeration = "AdditionalButtonStatus", tag = "1")]
    pub status: i32,
}
/// 动态通用附加卡-follow/取消follow-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynAdditionCommonFollowReq {
    ///
    #[prost(enumeration = "AdditionalButtonStatus", tag = "1")]
    pub status: i32,
    ///
    #[prost(string, tag = "2")]
    pub dyn_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub card_type: ::prost::alloc::string::String,
}
/// 最近访问-个人feed流列表-返回
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynAllPersonalReply {
    /// 动态列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    /// 偏移量
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    /// 是否还有更多数据
    #[prost(bool, tag = "3")]
    pub has_more: bool,
    /// 已读进度
    #[prost(string, tag = "4")]
    pub read_offset: ::prost::alloc::string::String,
    /// 关注状态
    #[prost(message, optional, tag = "5")]
    pub relation: ::core::option::Option<Relation>,
    /// 顶部预约卡
    #[prost(message, optional, tag = "6")]
    pub addition_up: ::core::option::Option<TopAdditionUp>,
    ///
    #[prost(string, tag = "7")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub title_sub: ::prost::alloc::string::String,
}
/// 最近访问-个人feed流列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynAllPersonalReq {
    /// 被访问者的 UID
    #[prost(int64, tag = "1")]
    pub host_uid: i64,
    /// 偏移量 第一页可传空
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    /// 标明下拉几次
    #[prost(int32, tag = "3")]
    pub page: i32,
    /// 是否是预加载 默认是1；客户端预加载。1：是预加载，不更新已读进度，不会影响小红点；0：非预加载，更新已读进度
    #[prost(int32, tag = "4")]
    pub is_preload: i32,
    /// 秒开参数 新版本废弃，统一使用player_args
    #[prost(message, optional, tag = "5")]
    pub playurl_param: ::core::option::Option<PlayurlParam>,
    /// 客户端时区 兼容UTC-14和Etc/GMT+12,时区区间\[-12,14\] 东八区为8
    #[prost(int32, tag = "6")]
    pub local_time: i32,
    /// 服务端生成的透传上报字段
    #[prost(string, tag = "7")]
    pub footprint: ::prost::alloc::string::String,
    /// 来源
    #[prost(string, tag = "8")]
    pub from: ::prost::alloc::string::String,
    /// 秒开用
    #[prost(message, optional, tag = "9")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(string, tag = "10")]
    pub personal_extra: ::prost::alloc::string::String,
}
/// 动态综合页-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynAllReply {
    /// 卡片列表
    #[prost(message, optional, tag = "1")]
    pub dynamic_list: ::core::option::Option<DynamicList>,
    /// 顶部up list
    #[prost(message, optional, tag = "2")]
    pub up_list: ::core::option::Option<CardVideoUpList>,
    /// 话题广场
    #[prost(message, optional, tag = "3")]
    pub topic_list: ::core::option::Option<TopicList>,
    /// 无关注推荐
    #[prost(message, optional, tag = "4")]
    pub unfollow: ::core::option::Option<Unfollow>,
    /// 分区UP推荐
    #[prost(message, optional, tag = "5")]
    pub region_rcmd: ::core::option::Option<DynRegionRcmd>,
    ///
    #[prost(message, optional, tag = "6")]
    pub config: ::core::option::Option<Config>,
}
/// 动态综合页-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynAllReq {
    /// 透传 update_baseline
    #[prost(string, tag = "1")]
    pub update_baseline: ::prost::alloc::string::String,
    /// 透传 history_offset
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    /// 向下翻页数
    #[prost(int32, tag = "3")]
    pub page: i32,
    /// 刷新方式 1向上刷新 2向下翻页
    #[prost(enumeration = "Refresh", tag = "4")]
    pub refresh_type: i32,
    /// 秒开参数 新版本废弃，统一使用player_args
    #[prost(message, optional, tag = "5")]
    pub playurl_param: ::core::option::Option<PlayurlParam>,
    /// 综合页当前更新的最大值
    #[prost(string, tag = "6")]
    pub assist_baseline: ::prost::alloc::string::String,
    /// 客户端时区 兼容UTC-14和Etc/GMT+12,时区区间\[-12,14\] 东八区为8
    #[prost(int32, tag = "7")]
    pub local_time: i32,
    /// 推荐up主入参(new的时候传)
    #[prost(message, optional, tag = "8")]
    pub rcmd_ups_param: ::core::option::Option<RcmdUPsParam>,
    /// 广告参数
    #[prost(message, optional, tag = "9")]
    pub ad_param: ::core::option::Option<AdParam>,
    /// 是否冷启
    #[prost(int32, tag = "10")]
    pub cold_start: i32,
    /// 来源
    #[prost(string, tag = "11")]
    pub from: ::prost::alloc::string::String,
    /// 秒开参数
    #[prost(message, optional, tag = "12")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int64, tag = "13")]
    pub tab_recall_uid: i64,
    ///
    #[prost(int32, tag = "14")]
    pub tab_recall_type: i32,
}
/// 最近访问-标记已读-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynAllUpdOffsetReq {
    /// 被访问者的UID
    #[prost(int64, tag = "1")]
    pub host_uid: i64,
    /// 用户已读进度
    #[prost(string, tag = "2")]
    pub read_offset: ::prost::alloc::string::String,
    /// 服务端生成的透传上报字段
    #[prost(string, tag = "3")]
    pub footprint: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub personal_extra: ::prost::alloc::string::String,
}
/// 动态卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicItem {
    /// 动态卡片类型
    #[prost(enumeration = "DynamicType", tag = "1")]
    pub card_type: i32,
    /// 转发类型下，源卡片类型
    #[prost(enumeration = "DynamicType", tag = "2")]
    pub item_type: i32,
    /// 模块内容
    #[prost(message, repeated, tag = "3")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
    /// 操作相关字段
    #[prost(message, optional, tag = "4")]
    pub extend: ::core::option::Option<Extend>,
    /// 该卡片下面是否含有折叠卡
    #[prost(int32, tag = "5")]
    pub has_fold: i32,
    /// 透传到客户端的埋点字段。
    #[prost(string, tag = "6")]
    pub server_info: ::prost::alloc::string::String,
}
/// 动态卡片列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicList {
    /// 动态列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    /// 更新的动态数
    #[prost(int64, tag = "2")]
    pub update_num: i64,
    /// 历史偏移
    #[prost(string, tag = "3")]
    pub history_offset: ::prost::alloc::string::String,
    /// 更新基础信息
    #[prost(string, tag = "4")]
    pub update_baseline: ::prost::alloc::string::String,
    /// 是否还有更多数据
    #[prost(bool, tag = "5")]
    pub has_more: bool,
}
/// 动态详情页-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynDetailReply {
    /// 动态详情
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<DynamicItem>,
}
/// 动态详情页-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynDetailReq {
    /// up主uid
    #[prost(int64, tag = "1")]
    pub uid: i64,
    /// 动态ID
    #[prost(string, tag = "2")]
    pub dynamic_id: ::prost::alloc::string::String,
    /// 动态类型
    #[prost(int64, tag = "3")]
    pub dyn_type: i64,
    /// 业务方资源id
    #[prost(int64, tag = "4")]
    pub rid: i64,
    /// 广告参数
    #[prost(message, optional, tag = "5")]
    pub ad_param: ::core::option::Option<AdParam>,
    /// From来源
    #[prost(string, tag = "6")]
    pub from: ::prost::alloc::string::String,
    /// 秒开参数
    #[prost(message, optional, tag = "7")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    /// 分享id
    #[prost(string, tag = "8")]
    pub share_id: ::prost::alloc::string::String,
    /// 分享类型
    /// 1:文字 2:图片 3:链接 4:视频 5:音频
    #[prost(int32, tag = "9")]
    pub share_mode: i32,
    /// 客户端时区 兼容UTC-14和Etc/GMT+12,时区区间\[-12,14\] 东八区为8
    #[prost(int32, tag = "10")]
    pub local_time: i32,
    /// pattern
    #[prost(string, tag = "11")]
    pub pattern: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "12")]
    pub config: ::core::option::Option<Config>,
}
/// 批量动态id获取动态详情-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynDetailsReply {
    /// 动态列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
}
/// 批量动态id获取动态详情-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynDetailsReq {
    /// 动态id
    #[prost(string, tag = "1")]
    pub dynamic_ids: ::prost::alloc::string::String,
    /// 秒开参数 新版本废弃，统一使用player_args
    #[prost(message, optional, tag = "2")]
    pub playurl_param: ::core::option::Option<PlayurlParam>,
    /// 客户端时区 兼容UTC-14和Etc/GMT+12,时区区间\[-12,14\] 东八区为8
    #[prost(int32, tag = "3")]
    pub local_time: i32,
    /// 秒开参数
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(message, optional, tag = "5")]
    pub config: ::core::option::Option<Config>,
}
/// 动态发布生成临时卡-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynFakeCardReply {
    /// 动态卡片
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<DynamicItem>,
}
/// 动态发布生成临时卡-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynFakeCardReq {
    /// 卡片内容json string
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynFeatureGate {
    ///
    #[prost(bool, tag = "1")]
    pub enhanced_interaction: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynFriendReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub dyn_list: ::prost::alloc::vec::Vec<DynamicItem>,
    ///
    #[prost(bool, tag = "2")]
    pub has_more: bool,
    ///
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynFriendReq {
    ///
    #[prost(string, tag = "1")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub local_time: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
/// 轻浏览-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynLightReply {
    /// 卡片列表
    #[prost(message, optional, tag = "1")]
    pub dynamic_list: ::core::option::Option<DynamicList>,
}
/// 轻浏览-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynLightReq {
    /// 透传 history_offset
    #[prost(string, tag = "1")]
    pub history_offset: ::prost::alloc::string::String,
    /// 向下翻页数
    #[prost(int32, tag = "2")]
    pub page: i32,
    /// 来源
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// 秒开参数
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    /// 客户端时区 兼容UTC-14和Etc/GMT+12,时区区间\[-12,14\] 东八区为8
    #[prost(int32, tag = "5")]
    pub local_time: i32,
    ///
    #[prost(int32, tag = "6")]
    pub from_type: i32,
    ///
    #[prost(int64, tag = "7")]
    pub fake_uid: i64,
}
/// 查看更多-列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynMixUpListViewMoreReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<MixUpListItem>,
    ///
    #[prost(string, tag = "2")]
    pub search_default_text: ::prost::alloc::string::String,
    /// 排序类型列表
    #[prost(message, repeated, tag = "3")]
    pub sort_types: ::prost::alloc::vec::Vec<SortType>,
    /// 是否展示更多的排序策略
    #[prost(bool, tag = "4")]
    pub show_more_sort_types: bool,
    /// 默认排序策略
    #[prost(int32, tag = "5")]
    pub default_sort_type: i32,
}
/// 查看更多-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynMixUpListViewMoreReq {
    /// 排序策略
    /// 1:推荐排序 2:最常访问 3:最近关注，其他值为默认排序
    #[prost(int32, tag = "1")]
    pub sort_type: i32,
}
/// 推荐页-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRcmdReply {
    /// 推荐页返回参数
    #[prost(message, optional, tag = "1")]
    pub region_rcmd: ::core::option::Option<DynRegionRcmd>,
    ///
    #[prost(message, optional, tag = "2")]
    pub dynamic_list: ::core::option::Option<DynamicList>,
}
/// 推荐页-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRcmdReq {
    /// 秒开参数
    #[prost(message, optional, tag = "1")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    /// 客户端时区 兼容UTC-14和Etc/GMT+12,时区区间\[-12,14\] 东八区为8
    #[prost(int32, tag = "2")]
    pub local_time: i32,
    ///
    #[prost(int64, tag = "3")]
    pub fake_uid: i64,
    ///
    #[prost(bool, tag = "4")]
    pub is_refresh: bool,
}
/// 关注推荐up主换一换-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRcmdUpExchangeReply {
    /// 无关注推荐
    #[prost(message, optional, tag = "1")]
    pub unfollow: ::core::option::Option<Unfollow>,
}
/// 关注推荐up主换一换-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRcmdUpExchangeReq {
    /// 登录用户id
    #[prost(int64, tag = "1")]
    pub uid: i64,
    /// 上一次不感兴趣的ts，单位：秒；该字段透传给搜索
    #[prost(int64, tag = "2")]
    pub dislike_ts: i64,
    /// 需要与服务端确认或参照客户端现有参数
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
}
/// 推荐页返回参数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRegionRcmd {
    /// 分区推荐项目列表
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<DynRegionRcmdItem>,
    /// 分区聚类推荐选项
    #[prost(message, optional, tag = "2")]
    pub opts: ::core::option::Option<RcmdOption>,
}
/// 分区推荐项目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRegionRcmdItem {
    /// 分区id
    #[prost(int64, tag = "1")]
    pub rid: i64,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 推荐模块
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<ModuleRcmd>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynScreenTab {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "3")]
    pub default_tab: bool,
    ///
    #[prost(bool, tag = "4")]
    pub strategy_show_on_entrance: bool,
    ///
    #[prost(bool, tag = "5")]
    pub strategy_show_on_refresh: bool,
    ///
    #[prost(bool, tag = "6")]
    pub strategy_show_on_pull_up: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynSearchReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub channel_info: ::core::option::Option<SearchChannel>,
    ///
    #[prost(message, optional, tag = "2")]
    pub search_topic: ::core::option::Option<SearchTopic>,
    ///
    #[prost(message, optional, tag = "3")]
    pub search_info: ::core::option::Option<SearchInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynSearchReq {
    ///
    #[prost(string, tag = "1")]
    pub keyword: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub page: i32,
    ///
    #[prost(int32, tag = "3")]
    pub local_time: i32,
    ///
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynServerDetailsReply {
    ///
    #[prost(map = "int64, message", tag = "1")]
    pub items: ::std::collections::HashMap<i64, DynamicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynServerDetailsReq {
    ///
    #[prost(int32, tag = "2")]
    pub local_time: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(string, tag = "4")]
    pub mobi_app: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub device: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub buvid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub build: i64,
    ///
    #[prost(int64, tag = "8")]
    pub mid: i64,
    ///
    #[prost(string, tag = "9")]
    pub platform: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "10")]
    pub is_master: bool,
    ///
    #[prost(int64, repeated, tag = "11")]
    pub top_dynamic_ids: ::prost::alloc::vec::Vec<i64>,
}
/// 空间页动态-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynSpaceReq {
    /// 被访问者，也就是空间主人的uid
    #[prost(int64, tag = "1")]
    pub host_uid: i64,
    /// 动态偏移history_offset
    #[prost(string, tag = "2")]
    pub history_offset: ::prost::alloc::string::String,
    /// 秒开参数
    #[prost(message, optional, tag = "3")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    /// 客户端时区 兼容UTC-14和Etc/GMT+12,时区区间\[-12,14\] 东八区为8
    #[prost(int32, tag = "4")]
    pub local_time: i32,
    /// 向下翻页数，默认从1开始
    #[prost(int64, tag = "5")]
    pub page: i64,
    /// 来源，空间页：space，直播tab：live
    #[prost(string, tag = "6")]
    pub from: ::prost::alloc::string::String,
}
/// 空间页动态-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynSpaceRsp {
    /// 卡片列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    /// 历史偏移
    #[prost(string, tag = "2")]
    pub history_offset: ::prost::alloc::string::String,
    /// 是否还有更多数据
    #[prost(bool, tag = "3")]
    pub has_more: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynSpaceSearchDetailsReply {
    ///
    #[prost(map = "int64, message", tag = "1")]
    pub items: ::std::collections::HashMap<i64, DynamicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynSpaceSearchDetailsReq {
    ///
    #[prost(string, repeated, tag = "2")]
    pub search_words: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "3")]
    pub local_time: i32,
    ///
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(string, tag = "5")]
    pub mobi_app: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub device: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub buvid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub build: i64,
    ///
    #[prost(int64, tag = "9")]
    pub mid: i64,
    ///
    #[prost(string, tag = "10")]
    pub platform: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub ip: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "12")]
    pub net_type: i32,
    ///
    #[prost(int32, tag = "13")]
    pub tf_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynTab {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bubble: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub red_point: i32,
    ///
    #[prost(int64, tag = "5")]
    pub city_id: i64,
    ///
    #[prost(int32, tag = "6")]
    pub is_popup: i32,
    ///
    #[prost(message, optional, tag = "7")]
    pub popup: ::core::option::Option<Popup>,
    ///
    #[prost(bool, tag = "8")]
    pub default_tab: bool,
    ///
    #[prost(string, tag = "9")]
    pub sub_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub anchor: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub internal_test: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "12")]
    pub r#type: i32,
    ///
    #[prost(message, optional, boxed, tag = "13")]
    pub back_up: ::core::option::Option<::prost::alloc::boxed::Box<DynTab>>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynTabReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub dyn_tab: ::prost::alloc::vec::Vec<DynTab>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub screen_tab: ::prost::alloc::vec::Vec<DynScreenTab>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynTabReq {
    ///
    #[prost(int32, tag = "1")]
    pub teenagers_mode: i32,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "2")]
    pub from_type: i32,
}
/// 动态点赞-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynThumbReq {
    /// 用户uid
    #[prost(int64, tag = "1")]
    pub uid: i64,
    /// 动态id
    #[prost(string, tag = "2")]
    pub dyn_id: ::prost::alloc::string::String,
    /// 动态类型(透传extend中的dyn_type)
    #[prost(int64, tag = "3")]
    pub dyn_type: i64,
    /// 业务方资源id
    #[prost(string, tag = "4")]
    pub rid: ::prost::alloc::string::String,
    /// 点赞类型
    #[prost(enumeration = "ThumbType", tag = "5")]
    pub r#type: i32,
}
/// 最近访问-个人feed流列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoPersonalReply {
    /// 动态列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    /// 偏移量
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    /// 是否还有更多数据
    #[prost(bool, tag = "3")]
    pub has_more: bool,
    /// 已读进度
    #[prost(string, tag = "4")]
    pub read_offset: ::prost::alloc::string::String,
    /// 关注状态
    #[prost(message, optional, tag = "5")]
    pub relation: ::core::option::Option<Relation>,
    /// 顶部预约卡
    #[prost(message, optional, tag = "6")]
    pub addition_up: ::core::option::Option<TopAdditionUp>,
    ///
    #[prost(string, tag = "7")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub title_sub: ::prost::alloc::string::String,
}
/// 最近访问-个人feed流列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoPersonalReq {
    /// 被访问者的 UID
    #[prost(int64, tag = "1")]
    pub host_uid: i64,
    /// 偏移量 第一页可传空
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    /// 标明下拉几次
    #[prost(int32, tag = "3")]
    pub page: i32,
    /// 是否是预加载
    #[prost(int32, tag = "4")]
    pub is_preload: i32,
    /// 秒开参数 新版本废弃，统一使用player_args
    #[prost(message, optional, tag = "5")]
    pub playurl_param: ::core::option::Option<PlayurlParam>,
    /// 客户端时区 兼容UTC-14和Etc/GMT+12,时区区间\[-12,14\] 东八区为8
    #[prost(int32, tag = "6")]
    pub local_time: i32,
    /// 服务端生成的透传上报字段
    #[prost(string, tag = "7")]
    pub footprint: ::prost::alloc::string::String,
    /// 来源
    #[prost(string, tag = "8")]
    pub from: ::prost::alloc::string::String,
    /// 秒开参数
    #[prost(message, optional, tag = "9")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int64, tag = "10")]
    pub pegasus_avid: i64,
    ///
    #[prost(string, tag = "11")]
    pub personal_extra: ::prost::alloc::string::String,
}
/// 动态视频页-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoReply {
    /// 卡片列表
    #[prost(message, optional, tag = "1")]
    pub dynamic_list: ::core::option::Option<CardVideoDynList>,
    /// 动态卡片
    #[prost(message, optional, tag = "2")]
    pub video_up_list: ::core::option::Option<CardVideoUpList>,
    /// 视频页-我的追番
    #[prost(message, optional, tag = "3")]
    pub video_follow_list: ::core::option::Option<CardVideoFollowList>,
}
/// 动态视频页-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoReq {
    /// 透传 update_baseline
    #[prost(string, tag = "1")]
    pub update_baseline: ::prost::alloc::string::String,
    /// 透传 history_offset
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    /// 向下翻页数
    #[prost(int32, tag = "3")]
    pub page: i32,
    /// 刷新方式
    /// 1:向上刷新 2:向下翻页
    #[prost(enumeration = "Refresh", tag = "4")]
    pub refresh_type: i32,
    /// 秒开参数 新版本废弃，统一使用player_args
    #[prost(message, optional, tag = "5")]
    pub playurl_param: ::core::option::Option<PlayurlParam>,
    /// 综合页当前更新的最大值
    #[prost(string, tag = "6")]
    pub assist_baseline: ::prost::alloc::string::String,
    /// 客户端时区 兼容UTC-14和Etc/GMT+12,时区区间\[-12,14\] 东八区为8
    #[prost(int32, tag = "7")]
    pub local_time: i32,
    /// 来源
    #[prost(string, tag = "8")]
    pub from: ::prost::alloc::string::String,
    /// 秒开参数
    #[prost(message, optional, tag = "9")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
/// 最近访问-标记已读-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoUpdOffsetReq {
    /// 被访问者的UID
    #[prost(int64, tag = "1")]
    pub host_uid: i64,
    /// 用户已读进度
    #[prost(string, tag = "2")]
    pub read_offset: ::prost::alloc::string::String,
    /// 服务端生成的透传上报字段
    #[prost(string, tag = "3")]
    pub footprint: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub personal_extra: ::prost::alloc::string::String,
}
/// 投票操作-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVoteReply {
    /// 投票详情
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<AdditionVote2>,
    /// 投票操作返回状态
    #[prost(string, tag = "2")]
    pub toast: ::prost::alloc::string::String,
}
/// 投票操作-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVoteReq {
    /// 投票ID
    #[prost(int64, tag = "1")]
    pub vote_id: i64,
    /// 选项索引数组
    #[prost(int64, repeated, tag = "2")]
    pub votes: ::prost::alloc::vec::Vec<i64>,
    /// 状态
    #[prost(enumeration = "VoteStatus", tag = "3")]
    pub status: i32,
    /// 动态ID
    #[prost(string, tag = "4")]
    pub dynamic_id: ::prost::alloc::string::String,
    /// 是否分享
    #[prost(bool, tag = "5")]
    pub share: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmojiSizeSpec {
    ///
    #[prost(int64, tag = "1")]
    pub width: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmoteNode {
    ///
    #[prost(string, tag = "2")]
    pub emote_url: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub emote_width: ::core::option::Option<EmoteSize>,
    ///
    #[prost(message, optional, tag = "5")]
    pub inline_img_cfg: ::core::option::Option<ImgInlineCfg>,
    ///
    #[prost(bool, tag = "4")]
    pub is_inline_img: bool,
    ///
    #[prost(message, optional, tag = "1")]
    pub raw_text: ::core::option::Option<WordNode>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmoteSize {
    ///
    #[prost(double, tag = "1")]
    pub width: f64,
    ///
    #[prost(int32, tag = "2")]
    pub emoji_size: i32,
}
/// 扩展字段，用于动态部分操作使用
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extend {
    /// 动态id
    #[prost(string, tag = "1")]
    pub dyn_id_str: ::prost::alloc::string::String,
    /// 业务方id
    #[prost(string, tag = "2")]
    pub business_id: ::prost::alloc::string::String,
    /// 源动态id
    #[prost(string, tag = "3")]
    pub orig_dyn_id_str: ::prost::alloc::string::String,
    /// 转发卡：用户名
    #[prost(string, tag = "4")]
    pub orig_name: ::prost::alloc::string::String,
    /// 转发卡：图片url
    #[prost(string, tag = "5")]
    pub orig_img_url: ::prost::alloc::string::String,
    /// 转发卡：文字内容
    #[prost(message, repeated, tag = "6")]
    pub orig_desc: ::prost::alloc::vec::Vec<Description>,
    /// 填充文字内容
    #[prost(message, repeated, tag = "7")]
    pub desc: ::prost::alloc::vec::Vec<Description>,
    /// 被转发的源动态类型
    #[prost(enumeration = "DynamicType", tag = "8")]
    pub orig_dyn_type: i32,
    /// 分享到站外展示类型
    #[prost(string, tag = "9")]
    pub share_type: ::prost::alloc::string::String,
    /// 分享的场景
    #[prost(string, tag = "10")]
    pub share_scene: ::prost::alloc::string::String,
    /// 是否快速转发
    #[prost(bool, tag = "11")]
    pub is_fast_share: bool,
    /// r_type 分享和转发
    #[prost(int32, tag = "12")]
    pub r_type: i32,
    /// 数据源的动态类型
    #[prost(int64, tag = "13")]
    pub dyn_type: i64,
    /// 用户id
    #[prost(int64, tag = "14")]
    pub uid: i64,
    /// 卡片跳转
    #[prost(string, tag = "15")]
    pub card_url: ::prost::alloc::string::String,
    /// 透传字段
    #[prost(message, optional, tag = "16")]
    pub source_content: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// 转发卡：用户头像
    #[prost(string, tag = "17")]
    pub orig_face: ::prost::alloc::string::String,
    /// 评论跳转
    #[prost(message, optional, tag = "18")]
    pub reply: ::core::option::Option<ExtendReply>,
    ///
    #[prost(string, tag = "19")]
    pub track_id: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "20")]
    pub opus_summary: ::core::option::Option<ModuleOpusSummary>,
    ///
    #[prost(message, optional, tag = "21")]
    pub only_fans_property: ::core::option::Option<OnlyFansProperty>,
    ///
    #[prost(message, optional, tag = "22")]
    pub feature_gate: ::core::option::Option<DynFeatureGate>,
    ///
    #[prost(bool, tag = "23")]
    pub is_in_audit: bool,
    ///
    #[prost(map = "string, string", tag = "24")]
    pub history_report: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// 评论扩展
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendReply {
    /// 基础跳转地址
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// 参数部分
    #[prost(message, repeated, tag = "2")]
    pub params: ::prost::alloc::vec::Vec<ExtendReplyParam>,
}
/// 评论扩展参数部分
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendReplyParam {
    /// 参数名
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// 参数值
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// 动态-拓展小卡模块-通用小卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtInfoCommon {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 小图标
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    /// poiType
    #[prost(int32, tag = "4")]
    pub poi_type: i32,
    /// 类型
    #[prost(enumeration = "DynExtendType", tag = "5")]
    pub r#type: i32,
    /// 客户端埋点用
    #[prost(string, tag = "6")]
    pub sub_module: ::prost::alloc::string::String,
    /// 行动点文案
    #[prost(string, tag = "7")]
    pub action_text: ::prost::alloc::string::String,
    /// 行动点链接
    #[prost(string, tag = "8")]
    pub action_url: ::prost::alloc::string::String,
    /// 资源rid
    #[prost(int64, tag = "9")]
    pub rid: i64,
    /// 轻浏览是否展示
    #[prost(bool, tag = "10")]
    pub is_show_light: bool,
}
/// 动态-拓展小卡模块-游戏小卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtInfoGame {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 小图标
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
}
/// 动态-拓展小卡模块-热门小卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtInfoHot {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 小图标
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
}
/// 动态-拓展小卡模块-lbs小卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtInfoLbs {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 小图标
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    /// poiType
    #[prost(int32, tag = "4")]
    pub poi_type: i32,
}
/// 动态-拓展小卡模块-ogv小卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtInfoOgv {
    /// ogv小卡
    #[prost(message, repeated, tag = "1")]
    pub info_ogv: ::prost::alloc::vec::Vec<InfoOgv>,
}
/// 动态-拓展小卡模块-话题小卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtInfoTopic {
    /// 标题-话题名
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 小图标
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedFilterReply {
    ///
    #[prost(string, tag = "1")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub has_more: bool,
    ///
    #[prost(message, repeated, tag = "3")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedFilterReq {
    ///
    #[prost(string, tag = "1")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub tab: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub local_time: i32,
    ///
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(message, optional, tag = "5")]
    pub ad_param: ::core::option::Option<AdParam>,
    ///
    #[prost(int32, tag = "6")]
    pub cold_start: i32,
    ///
    #[prost(int64, tag = "7")]
    pub page: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchTabSettingReply {
    ///
    #[prost(int32, tag = "1")]
    pub status: i32,
}
/// 视频页-我的追番-番剧信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowListItem {
    /// season_id
    #[prost(int64, tag = "1")]
    pub season_id: i64,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    /// new_ep
    #[prost(message, optional, tag = "5")]
    pub new_ep: ::core::option::Option<NewEp>,
    /// 子标题
    #[prost(string, tag = "6")]
    pub sub_title: ::prost::alloc::string::String,
    /// 卡片位次
    #[prost(int64, tag = "7")]
    pub pos: i64,
}
/// 动态-附加卡-商品卡-商品
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoodsItem {
    /// 图片
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    /// schemaPackageName(Android用)
    #[prost(string, tag = "2")]
    pub schema_package_name: ::prost::alloc::string::String,
    /// 商品类型
    /// 1:淘宝 2:会员购
    #[prost(int32, tag = "3")]
    pub source_type: i32,
    /// 跳转链接
    #[prost(string, tag = "4")]
    pub jump_url: ::prost::alloc::string::String,
    /// 跳转文案
    #[prost(string, tag = "5")]
    pub jump_desc: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    /// 摘要
    #[prost(string, tag = "7")]
    pub brief: ::prost::alloc::string::String,
    /// 价格
    #[prost(string, tag = "8")]
    pub price: ::prost::alloc::string::String,
    /// item_id
    #[prost(int64, tag = "9")]
    pub item_id: i64,
    /// schema_url
    #[prost(string, tag = "10")]
    pub schema_url: ::prost::alloc::string::String,
    /// open_white_list
    #[prost(string, repeated, tag = "11")]
    pub open_white_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// use_web_v2
    #[prost(bool, tag = "12")]
    pub user_web_v2: bool,
    /// ad mark
    #[prost(string, tag = "13")]
    pub ad_mark: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub app_name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "GoodsJumpType", tag = "15")]
    pub jump_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuideBarInfo {
    ///
    #[prost(int32, tag = "1")]
    pub show: i32,
    ///
    #[prost(int32, tag = "2")]
    pub page: i32,
    ///
    #[prost(int32, tag = "3")]
    pub position: i32,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub jump_page: i32,
    ///
    #[prost(int32, tag = "6")]
    pub jump_position: i32,
}
/// 高亮文本
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HighlightText {
    /// 展示文本
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 高亮类型
    #[prost(enumeration = "HighlightTextStyle", tag = "2")]
    pub text_style: i32,
    ///
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeSubscribeReply {
    ///
    #[prost(int32, tag = "1")]
    pub online: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeSubscribeReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IconBadge {
    ///
    #[prost(string, tag = "1")]
    pub icon_bg_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IconButton {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon_head: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon_tail: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub jump_uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSet {
    ///
    #[prost(string, tag = "1")]
    pub img_day: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub img_dark: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImgInlineCfg {
    ///
    #[prost(double, tag = "1")]
    pub width: f64,
    ///
    #[prost(double, tag = "2")]
    pub height: f64,
    ///
    #[prost(message, optional, tag = "3")]
    pub color: ::core::option::Option<Colors>,
}
/// 动态-拓展小卡模块-ogv小卡-(one of 片单、榜单、分区)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoOgv {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 小图标
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    /// 客户端埋点用
    #[prost(string, tag = "4")]
    pub sub_module: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionFace {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub face: ::prost::alloc::string::String,
}
/// 外露交互模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionItem {
    /// 外露模块类型
    #[prost(enumeration = "LocalIconType", tag = "1")]
    pub icon_type: i32,
    /// 外露模块文案
    #[prost(message, repeated, tag = "2")]
    pub desc: ::prost::alloc::vec::Vec<Description>,
    /// 外露模块uri相关 根据type不同用法不同
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 动态id
    #[prost(string, tag = "4")]
    pub dynamic_id: ::prost::alloc::string::String,
    /// 评论mid
    #[prost(int64, tag = "6")]
    pub comment_mid: i64,
    ///
    #[prost(message, repeated, tag = "7")]
    pub faces: ::prost::alloc::vec::Vec<InteractionFace>,
    ///
    #[prost(message, optional, tag = "8")]
    pub stat: ::core::option::Option<InteractionStat>,
    ///
    #[prost(string, tag = "9")]
    pub icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionStat {
    ///
    #[prost(int64, tag = "1")]
    pub like: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbsPoiDetail {
    ///
    #[prost(string, tag = "1")]
    pub poi: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub r#type: i64,
    ///
    #[prost(string, repeated, tag = "3")]
    pub base_pic: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, repeated, tag = "4")]
    pub cover: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "5")]
    pub address: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbsPoiReply {
    ///
    #[prost(bool, tag = "1")]
    pub has_more: bool,
    ///
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub detail: ::core::option::Option<LbsPoiDetail>,
    ///
    #[prost(message, repeated, tag = "4")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbsPoiReq {
    ///
    #[prost(string, tag = "1")]
    pub poi: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub r#type: i64,
    ///
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub refresh_type: i32,
    ///
    #[prost(int32, tag = "5")]
    pub local_time: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyTopicFeedReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    ///
    #[prost(bool, tag = "2")]
    pub has_more: bool,
    ///
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub supported_sort_types: ::prost::alloc::vec::Vec<SortType>,
    ///
    #[prost(message, repeated, tag = "5")]
    pub feed_card_filters: ::prost::alloc::vec::Vec<SortType>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyTopicFeedReq {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub topic_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub sort_type: ::core::option::Option<SortType>,
    ///
    #[prost(message, optional, tag = "5")]
    pub card_filter: ::core::option::Option<SortType>,
}
/// 点赞动画
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeAnimation {
    /// 开始动画
    #[prost(string, tag = "1")]
    pub begin: ::prost::alloc::string::String,
    /// 过程动画
    #[prost(string, tag = "2")]
    pub proc: ::prost::alloc::string::String,
    /// 结束动画
    #[prost(string, tag = "3")]
    pub end: ::prost::alloc::string::String,
    /// id
    #[prost(int64, tag = "4")]
    pub like_icon_id: i64,
}
/// 点赞拓展信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeInfo {
    /// 点赞动画
    #[prost(message, optional, tag = "1")]
    pub animation: ::core::option::Option<LikeAnimation>,
    /// 是否点赞
    #[prost(bool, tag = "2")]
    pub is_like: bool,
}
/// 点赞列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeListReply {
    /// 用户模块列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<ModuleAuthor>,
    /// 是否还有更多数据
    #[prost(bool, tag = "2")]
    pub has_more: bool,
    /// 点赞总数
    #[prost(int64, tag = "3")]
    pub total_count: i64,
}
/// 点赞列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeListReq {
    /// 动态ID
    #[prost(string, tag = "1")]
    pub dynamic_id: ::prost::alloc::string::String,
    /// 动态类型
    #[prost(int64, tag = "2")]
    pub dyn_type: i64,
    /// 业务方资源id
    #[prost(int64, tag = "3")]
    pub rid: i64,
    /// 上一页最后一个uid
    #[prost(int64, tag = "4")]
    pub uid_offset: i64,
    /// 下拉页数
    #[prost(int32, tag = "5")]
    pub page: i32,
}
/// 点赞用户
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeUser {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub uid: i64,
    /// 用户昵称
    #[prost(string, tag = "2")]
    pub uname: ::prost::alloc::string::String,
    /// 点击跳转链接
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineParagraph {
    ///
    #[prost(message, optional, tag = "1")]
    pub pic: ::core::option::Option<MdlDynDrawItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkNode {
    ///
    #[prost(string, tag = "1")]
    pub show_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub icon_suffix: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub link_type: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "LinkNodeType", tag = "6")]
    pub link_type_enum: i32,
    ///
    #[prost(string, tag = "7")]
    pub biz_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub timestamp: i64,
    ///
    #[prost(message, optional, tag = "9")]
    pub goods_item: ::core::option::Option<GoodsItem>,
    ///
    #[prost(message, optional, tag = "10")]
    pub note_video_ts: ::core::option::Option<NoteVideoTs>,
}
/// 直播信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveInfo {
    /// 是否在直播
    /// 0:未直播 1:正在直播 (废弃)
    #[prost(int32, tag = "1")]
    pub is_living: i32,
    /// 跳转链接
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 直播状态
    #[prost(enumeration = "LiveState", tag = "3")]
    pub live_state: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LivePendant {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub pendant_id: i64,
}
/// 动态-附加卡-电竞卡-战队
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchTeam {
    /// 战队ID
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 战队名
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 战队图标
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    /// 日间色值
    #[prost(string, tag = "4")]
    pub color: ::prost::alloc::string::String,
    /// 夜间色值
    #[prost(string, tag = "5")]
    pub night_color: ::prost::alloc::string::String,
}
/// 动态列表渲染部分-详情模块-小程序/小游戏
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynApplet {
    /// 小程序id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 主标题
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    /// 副标题
    #[prost(string, tag = "5")]
    pub sub_title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "6")]
    pub cover: ::prost::alloc::string::String,
    /// 小程序icon
    #[prost(string, tag = "7")]
    pub icon: ::prost::alloc::string::String,
    /// 小程序标题
    #[prost(string, tag = "8")]
    pub label: ::prost::alloc::string::String,
    /// 按钮文案
    #[prost(string, tag = "9")]
    pub button_title: ::prost::alloc::string::String,
}
/// 动态-详情模块-稿件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynArchive {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    /// 秒开地址
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 视频封面展示项 1
    #[prost(string, tag = "4")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    /// 视频封面展示项 2
    #[prost(string, tag = "5")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    /// 封面视频展示项 3
    #[prost(string, tag = "6")]
    pub cover_left_text_3: ::prost::alloc::string::String,
    /// avid
    #[prost(int64, tag = "7")]
    pub avid: i64,
    /// cid
    #[prost(int64, tag = "8")]
    pub cid: i64,
    /// 视频源类型
    #[prost(enumeration = "MediaType", tag = "9")]
    pub media_type: i32,
    /// 尺寸信息
    #[prost(message, optional, tag = "10")]
    pub dimension: ::core::option::Option<Dimension>,
    /// 角标，多个角标之前有间距
    #[prost(message, repeated, tag = "11")]
    pub badge: ::prost::alloc::vec::Vec<VideoBadge>,
    /// 是否能够自动播放
    #[prost(bool, tag = "12")]
    pub can_play: bool,
    /// stype
    #[prost(enumeration = "VideoType", tag = "13")]
    pub stype: i32,
    /// 是否PGC
    #[prost(bool, tag = "14")]
    pub is_pgc: bool,
    /// inline播放地址
    #[prost(string, tag = "15")]
    pub inline_url: ::prost::alloc::string::String,
    /// PGC的epid
    #[prost(int64, tag = "16")]
    pub episode_id: i64,
    /// 子类型
    #[prost(int32, tag = "17")]
    pub sub_type: i32,
    /// PGC的ssid
    #[prost(int64, tag = "18")]
    pub pgc_season_id: i64,
    /// 播放按钮
    #[prost(string, tag = "19")]
    pub play_icon: ::prost::alloc::string::String,
    /// 时长
    #[prost(int64, tag = "20")]
    pub duration: i64,
    /// 跳转地址
    #[prost(string, tag = "21")]
    pub jump_url: ::prost::alloc::string::String,
    /// 番剧是否为预览视频
    #[prost(bool, tag = "22")]
    pub is_preview: bool,
    /// 新角标，多个角标之前没有间距
    #[prost(message, repeated, tag = "23")]
    pub badge_category: ::prost::alloc::vec::Vec<VideoBadge>,
    /// 当前是否是pgc正片
    #[prost(bool, tag = "24")]
    pub is_feature: bool,
    /// 是否是预约召回
    #[prost(enumeration = "ReserveType", tag = "25")]
    pub reserve_type: i32,
    /// bvid
    #[prost(string, tag = "26")]
    pub bvid: ::prost::alloc::string::String,
    /// 播放数
    #[prost(int32, tag = "27")]
    pub view: i32,
    ///
    #[prost(bool, tag = "28")]
    pub show_premiere_badge: bool,
    ///
    #[prost(bool, tag = "29")]
    pub premiere_card: bool,
    ///
    #[prost(bool, tag = "30")]
    pub show_progress: bool,
    ///
    #[prost(int64, tag = "31")]
    pub part_duration: i64,
    ///
    #[prost(int64, tag = "32")]
    pub part_progress: i64,
}
/// 动态列表渲染部分-详情模块-专栏模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynArticle {
    /// 专栏id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// 文案部分
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    /// 配图
    #[prost(string, repeated, tag = "5")]
    pub covers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 阅读量标签
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
    /// 模板类型
    #[prost(int32, tag = "7")]
    pub template_id: i32,
}
/// 动态列表渲染部分-详情模块-通用
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynCommon {
    /// 物料id
    #[prost(int64, tag = "1")]
    pub oid: i64,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// 描述 漫画卡标题下第一行
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    /// 封面
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    /// 标签1 漫画卡标题下第二行
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
    /// 所属业务类型
    #[prost(int32, tag = "7")]
    pub biz_type: i32,
    /// 镜像数据ID
    #[prost(int64, tag = "8")]
    pub sketch_id: i64,
    /// 卡片样式
    #[prost(enumeration = "MdlDynCommonType", tag = "9")]
    pub style: i32,
    /// 角标
    #[prost(message, repeated, tag = "10")]
    pub badge: ::prost::alloc::vec::Vec<VideoBadge>,
    ///
    #[prost(message, optional, tag = "11")]
    pub button: ::core::option::Option<AdditionalButton>,
}
/// 动态-详情模块-付费课程批次
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynCourBatch {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 展示项 1(本集标题)
    #[prost(string, tag = "4")]
    pub text_1: ::prost::alloc::string::String,
    /// 展示项 2(更新了多少个视频)
    #[prost(string, tag = "5")]
    pub text_2: ::prost::alloc::string::String,
    /// 角标
    #[prost(message, optional, tag = "6")]
    pub badge: ::core::option::Option<VideoBadge>,
    /// 播放按钮
    #[prost(string, tag = "7")]
    pub play_icon: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "8")]
    pub can_play: bool,
    ///
    #[prost(bool, tag = "9")]
    pub is_preview: bool,
    ///
    #[prost(string, tag = "10")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub cover_left_text_3: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "13")]
    pub avid: i64,
    ///
    #[prost(int64, tag = "14")]
    pub cid: i64,
    ///
    #[prost(int64, tag = "15")]
    pub epid: i64,
    ///
    #[prost(int64, tag = "16")]
    pub duration: i64,
    ///
    #[prost(int64, tag = "17")]
    pub season_id: i64,
}
/// 动态-详情模块-付费课程系列
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynCourSeason {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 展示项 1(更新信息)
    #[prost(string, tag = "4")]
    pub text_1: ::prost::alloc::string::String,
    /// 描述信息
    #[prost(string, tag = "5")]
    pub desc: ::prost::alloc::string::String,
    /// 角标
    #[prost(message, optional, tag = "6")]
    pub badge: ::core::option::Option<VideoBadge>,
    /// 播放按钮
    #[prost(string, tag = "7")]
    pub play_icon: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "8")]
    pub can_play: bool,
    ///
    #[prost(bool, tag = "9")]
    pub is_preview: bool,
    ///
    #[prost(int64, tag = "10")]
    pub avid: i64,
    ///
    #[prost(int64, tag = "11")]
    pub cid: i64,
    ///
    #[prost(int64, tag = "12")]
    pub epid: i64,
    ///
    #[prost(int64, tag = "13")]
    pub duration: i64,
    ///
    #[prost(int64, tag = "14")]
    pub season_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynCourUp {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub text_1: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub badge: ::core::option::Option<VideoBadge>,
    ///
    #[prost(string, tag = "7")]
    pub play_icon: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "8")]
    pub can_play: bool,
    ///
    #[prost(bool, tag = "9")]
    pub is_preview: bool,
    ///
    #[prost(int64, tag = "10")]
    pub avid: i64,
    ///
    #[prost(int64, tag = "11")]
    pub cid: i64,
    ///
    #[prost(int64, tag = "12")]
    pub epid: i64,
    ///
    #[prost(int64, tag = "13")]
    pub duration: i64,
    ///
    #[prost(int64, tag = "14")]
    pub season_id: i64,
}
/// 动态列表渲染部分-详情模块-图文模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynDraw {
    /// 图片
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<MdlDynDrawItem>,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 图文ID
    #[prost(int64, tag = "3")]
    pub id: i64,
    ///
    #[prost(bool, tag = "4")]
    pub is_draw_first: bool,
    ///
    #[prost(bool, tag = "5")]
    pub is_big_cover: bool,
    ///
    #[prost(bool, tag = "6")]
    pub is_article_cover: bool,
}
/// 动态列表渲染部分-详情模块-图文
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynDrawItem {
    /// 图片链接
    #[prost(string, tag = "1")]
    pub src: ::prost::alloc::string::String,
    /// 图片宽度
    #[prost(int64, tag = "2")]
    pub width: i64,
    /// 图片高度
    #[prost(int64, tag = "3")]
    pub height: i64,
    /// 图片大小
    #[prost(float, tag = "4")]
    pub size: f32,
    /// 图片标签
    #[prost(message, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<MdlDynDrawTag>,
}
/// 动态列表渲染部分-详情模块-图文-标签
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynDrawTag {
    /// 标签类型
    #[prost(enumeration = "MdlDynDrawTagType", tag = "1")]
    pub r#type: i32,
    /// 标签详情
    #[prost(message, optional, tag = "2")]
    pub item: ::core::option::Option<MdlDynDrawTagItem>,
}
/// 动态列表部分-详情模块-图文-标签详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynDrawTagItem {
    /// 跳转链接
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// 标签内容
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    /// 坐标-x
    #[prost(int64, tag = "3")]
    pub x: i64,
    /// 坐标-y
    #[prost(int64, tag = "4")]
    pub y: i64,
    /// 方向
    #[prost(int32, tag = "5")]
    pub orientation: i32,
    /// 来源
    /// 0:未知 1:淘宝 2:自营
    #[prost(int32, tag = "6")]
    pub source: i32,
    /// 商品id
    #[prost(int64, tag = "7")]
    pub item_id: i64,
    /// 用户mid
    #[prost(int64, tag = "8")]
    pub mid: i64,
    /// 话题id
    #[prost(int64, tag = "9")]
    pub tid: i64,
    /// lbs信息
    #[prost(string, tag = "10")]
    pub poi: ::prost::alloc::string::String,
    /// 商品标签链接
    #[prost(string, tag = "11")]
    pub schema_url: ::prost::alloc::string::String,
}
/// 动态列表渲染部分-详情模块-转发模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynForward {
    /// 动态转发核心模块 套娃
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<DynamicItem>,
    /// 透传类型
    /// 0:分享 1:转发
    #[prost(int32, tag = "2")]
    pub rtype: i32,
}
/// 动态列表渲染部分-详情模块-直播
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynLive {
    /// 房间号
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 直播间标题
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// 直播间封面
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
    /// 标题1 例: 陪伴学习
    #[prost(string, tag = "5")]
    pub cover_label: ::prost::alloc::string::String,
    /// 标题2 例: 54.6万人气
    #[prost(string, tag = "6")]
    pub cover_label2: ::prost::alloc::string::String,
    /// 直播状态
    #[prost(enumeration = "LiveState", tag = "7")]
    pub live_state: i32,
    /// 直播角标
    #[prost(message, optional, tag = "8")]
    pub badge: ::core::option::Option<VideoBadge>,
    /// 是否是预约召回
    #[prost(enumeration = "ReserveType", tag = "9")]
    pub reserve_type: i32,
}
/// 动态列表渲染部分-详情模块-直播推荐
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynLiveRcmd {
    /// 直播数据
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// 是否是预约召回
    #[prost(enumeration = "ReserveType", tag = "2")]
    pub reserve_type: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub pendant: ::core::option::Option<LivePendant>,
}
/// 动态列表渲染部分-详情模块-播单
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynMedialist {
    /// 播单id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 主标题
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// 副标题
    #[prost(string, tag = "4")]
    pub sub_title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    /// 封面类型
    #[prost(int32, tag = "6")]
    pub cover_type: i32,
    /// 角标
    #[prost(message, optional, tag = "7")]
    pub badge: ::core::option::Option<VideoBadge>,
}
/// 动态列表渲染部分-详情模块-音频模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynMusic {
    /// 音频id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// upId
    #[prost(int64, tag = "3")]
    pub up_id: i64,
    /// 歌名
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    /// 专辑封面
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    /// 展示项1
    #[prost(string, tag = "6")]
    pub label1: ::prost::alloc::string::String,
    /// upper
    #[prost(string, tag = "7")]
    pub upper: ::prost::alloc::string::String,
}
/// 动态-详情模块-pgc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynPgc {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    /// 秒开地址
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 视频封面展示项 1
    #[prost(string, tag = "4")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    /// 视频封面展示项 2
    #[prost(string, tag = "5")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    /// 封面视频展示项 3
    #[prost(string, tag = "6")]
    pub cover_left_text_3: ::prost::alloc::string::String,
    /// cid
    #[prost(int64, tag = "7")]
    pub cid: i64,
    /// season_id
    #[prost(int64, tag = "8")]
    pub season_id: i64,
    /// epid
    #[prost(int64, tag = "9")]
    pub epid: i64,
    /// aid
    #[prost(int64, tag = "10")]
    pub aid: i64,
    /// 视频源类型
    #[prost(enumeration = "MediaType", tag = "11")]
    pub media_type: i32,
    /// 番剧类型
    #[prost(enumeration = "VideoSubType", tag = "12")]
    pub sub_type: i32,
    /// 番剧是否为预览视频
    #[prost(bool, tag = "13")]
    pub is_preview: bool,
    /// 尺寸信息
    #[prost(message, optional, tag = "14")]
    pub dimension: ::core::option::Option<Dimension>,
    /// 角标，多个角标之前有间距
    #[prost(message, repeated, tag = "15")]
    pub badge: ::prost::alloc::vec::Vec<VideoBadge>,
    /// 是否能够自动播放
    #[prost(bool, tag = "16")]
    pub can_play: bool,
    /// season
    #[prost(message, optional, tag = "17")]
    pub season: ::core::option::Option<PgcSeason>,
    /// 播放按钮
    #[prost(string, tag = "18")]
    pub play_icon: ::prost::alloc::string::String,
    /// 时长
    #[prost(int64, tag = "19")]
    pub duration: i64,
    /// 跳转地址
    #[prost(string, tag = "20")]
    pub jump_url: ::prost::alloc::string::String,
    /// 新角标，多个角标之前没有间距
    #[prost(message, repeated, tag = "21")]
    pub badge_category: ::prost::alloc::vec::Vec<VideoBadge>,
    /// 当前是否是pgc正片
    #[prost(bool, tag = "22")]
    pub is_feature: bool,
}
/// 动态列表渲染部分-详情模块-订阅卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynSubscription {
    /// 卡片物料id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 广告创意id
    #[prost(int64, tag = "2")]
    pub ad_id: i64,
    /// 跳转地址
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    /// 广告标题
    #[prost(string, tag = "6")]
    pub ad_title: ::prost::alloc::string::String,
    /// 角标
    #[prost(message, optional, tag = "7")]
    pub badge: ::core::option::Option<VideoBadge>,
    /// 小提示
    #[prost(string, tag = "8")]
    pub tips: ::prost::alloc::string::String,
}
/// 动态新附加卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynSubscriptionNew {
    /// 样式类型
    #[prost(enumeration = "MdlDynSubscriptionNewStyle", tag = "1")]
    pub style: i32,
    /// 新订阅卡数据
    #[prost(oneof = "mdl_dyn_subscription_new::Item", tags = "2, 3")]
    pub item: ::core::option::Option<mdl_dyn_subscription_new::Item>,
}
/// Nested message and enum types in `MdlDynSubscriptionNew`.
pub mod mdl_dyn_subscription_new {
    /// 新订阅卡数据
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        ///
        #[prost(message, tag = "2")]
        DynSubscription(super::MdlDynSubscription),
        /// 直播推荐
        #[prost(message, tag = "3")]
        DynLiveRcmd(super::MdlDynLiveRcmd),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynTopicSet {
    ///
    #[prost(message, repeated, tag = "1")]
    pub topics: ::prost::alloc::vec::Vec<TopicItem>,
    ///
    #[prost(message, optional, tag = "2")]
    pub more_btn: ::core::option::Option<IconButton>,
    ///
    #[prost(int64, tag = "3")]
    pub topic_set_id: i64,
    ///
    #[prost(int64, tag = "4")]
    pub push_id: i64,
}
/// 动态列表渲染部分-UGC合集
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdlDynUgcSeason {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    /// 秒开地址
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 视频封面展示项 1
    #[prost(string, tag = "4")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    /// 视频封面展示项 2
    #[prost(string, tag = "5")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    /// 封面视频展示项 3
    #[prost(string, tag = "6")]
    pub cover_left_text_3: ::prost::alloc::string::String,
    /// 卡片物料id
    #[prost(int64, tag = "7")]
    pub id: i64,
    /// inline播放地址
    #[prost(string, tag = "8")]
    pub inline_url: ::prost::alloc::string::String,
    /// 是否能够自动播放
    #[prost(bool, tag = "9")]
    pub can_play: bool,
    /// 播放按钮
    #[prost(string, tag = "10")]
    pub play_icon: ::prost::alloc::string::String,
    /// avid
    #[prost(int64, tag = "11")]
    pub avid: i64,
    /// cid
    #[prost(int64, tag = "12")]
    pub cid: i64,
    /// 尺寸信息
    #[prost(message, optional, tag = "13")]
    pub dimension: ::core::option::Option<Dimension>,
    /// 时长
    #[prost(int64, tag = "14")]
    pub duration: i64,
    /// 跳转地址
    #[prost(string, tag = "15")]
    pub jump_url: ::prost::alloc::string::String,
}
/// 查看更多-列表单条数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MixUpListItem {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub uid: i64,
    /// 特别关注
    /// 0:否 1:是
    #[prost(int32, tag = "2")]
    pub special_attention: i32,
    /// 小红点状态
    /// 0:没有 1:有
    #[prost(int32, tag = "3")]
    pub reddot_state: i32,
    /// 直播信息
    #[prost(message, optional, tag = "4")]
    pub live_info: ::core::option::Option<MixUpListLiveItem>,
    /// 昵称
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// 头像
    #[prost(string, tag = "6")]
    pub face: ::prost::alloc::string::String,
    /// 认证信息
    #[prost(message, optional, tag = "7")]
    pub official: ::core::option::Option<OfficialVerify>,
    /// 大会员信息
    #[prost(message, optional, tag = "8")]
    pub vip: ::core::option::Option<VipInfo>,
    /// 关注状态
    #[prost(message, optional, tag = "9")]
    pub relation: ::core::option::Option<Relation>,
    ///
    #[prost(int32, tag = "10")]
    pub permire_state: i32,
    ///
    #[prost(string, tag = "11")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MixUpListLiveItem {
    /// 直播状态
    /// 0:未直播 1:直播中
    #[prost(bool, tag = "1")]
    pub status: bool,
    /// 房间号
    #[prost(int64, tag = "2")]
    pub room_id: i64,
    /// 跳转地址
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}
/// 动态模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// 类型
    #[prost(enumeration = "DynModuleType", tag = "1")]
    pub module_type: i32,
    #[prost(
        oneof = "module::ModuleItem",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35"
    )]
    pub module_item: ::core::option::Option<module::ModuleItem>,
}
/// Nested message and enum types in `Module`.
pub mod module {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModuleItem {
        /// 用户模块 1
        #[prost(message, tag = "2")]
        ModuleAuthor(super::ModuleAuthor),
        /// 争议黄条模块 2
        #[prost(message, tag = "3")]
        ModuleDispute(super::ModuleDispute),
        /// 动态正文模块 3
        #[prost(message, tag = "4")]
        ModuleDesc(super::ModuleDesc),
        /// 动态卡模块 4
        #[prost(message, tag = "5")]
        ModuleDynamic(super::ModuleDynamic),
        /// 点赞外露(废弃)
        #[prost(message, tag = "6")]
        ModuleLikeUser(super::ModuleLikeUser),
        /// 小卡模块 6
        #[prost(message, tag = "7")]
        ModuleExtend(super::ModuleExtend),
        /// 大卡模块 5
        #[prost(message, tag = "8")]
        ModuleAdditional(super::ModuleAdditional),
        /// 计数模块 8
        #[prost(message, tag = "9")]
        ModuleStat(super::ModuleStat),
        /// 折叠模块 9
        #[prost(message, tag = "10")]
        ModuleFold(super::ModuleFold),
        /// 评论外露(废弃)
        #[prost(message, tag = "11")]
        ModuleComment(super::ModuleComment),
        /// 外露交互模块(点赞、评论) 7
        #[prost(message, tag = "12")]
        ModuleInteraction(super::ModuleInteraction),
        /// 转发卡-原卡用户模块
        #[prost(message, tag = "13")]
        ModuleAuthorForward(super::ModuleAuthorForward),
        /// 广告卡
        #[prost(message, tag = "14")]
        ModuleAd(super::ModuleAd),
        /// 通栏
        #[prost(message, tag = "15")]
        ModuleBanner(super::ModuleBanner),
        /// 获取物料失败
        #[prost(message, tag = "16")]
        ModuleItemNull(super::ModuleItemNull),
        /// 分享组件
        #[prost(message, tag = "17")]
        ModuleShareInfo(super::ModuleShareInfo),
        /// 相关推荐模块
        #[prost(message, tag = "18")]
        ModuleRecommend(super::ModuleRecommend),
        /// 顶部模块
        #[prost(message, tag = "19")]
        ModuleTop(super::ModuleTop),
        /// 底部模块
        #[prost(message, tag = "20")]
        ModuleButtom(super::ModuleButtom),
        /// 转发卡计数模块
        #[prost(message, tag = "21")]
        ModuleStatForward(super::ModuleStat),
        ///
        #[prost(message, tag = "22")]
        ModuleStory(super::ModuleStory),
        ///
        #[prost(message, tag = "23")]
        ModuleTopic(super::ModuleTopic),
        ///
        #[prost(message, tag = "24")]
        ModuleTopicDetailsExt(super::ModuleTopicDetailsExt),
        ///
        #[prost(message, tag = "25")]
        ModuleTopTag(super::ModuleTopTag),
        ///
        #[prost(message, tag = "26")]
        ModuleTopicBrief(super::ModuleTopicBrief),
        ///
        #[prost(message, tag = "27")]
        ModuleTitle(super::ModuleTitle),
        ///
        #[prost(message, tag = "28")]
        ModuleButton(super::ModuleButton),
        ///
        #[prost(message, tag = "29")]
        ModuleNotice(super::ModuleNotice),
        ///
        #[prost(message, tag = "30")]
        ModuleOpusSummary(super::ModuleOpusSummary),
        ///
        #[prost(message, tag = "31")]
        ModuleCopyright(super::ModuleCopyright),
        ///
        #[prost(message, tag = "32")]
        ModuleParagraph(super::ModuleParagraph),
        ///
        #[prost(message, tag = "33")]
        ModuleBlocked(super::ModuleBlocked),
        ///
        #[prost(message, tag = "34")]
        ModuleTextNotice(super::ModuleTextNotice),
        ///
        #[prost(message, tag = "35")]
        ModuleOpusCollection(super::ModuleOpusCollection),
    }
}
/// 动态列表-用户模块-广告卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAd {
    /// 广告透传信息
    #[prost(message, optional, tag = "1")]
    pub source_content: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// 用户模块
    #[prost(message, optional, tag = "2")]
    pub module_author: ::core::option::Option<ModuleAuthor>,
    ///
    #[prost(int32, tag = "3")]
    pub ad_content_type: i32,
    ///
    #[prost(string, tag = "4")]
    pub cover_left_text1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_left_text2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub cover_left_text3: ::prost::alloc::string::String,
}
/// 动态-附加卡模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAdditional {
    /// 类型
    #[prost(enumeration = "AdditionalType", tag = "1")]
    pub r#type: i32,
    /// 附加卡物料ID
    #[prost(int64, tag = "7")]
    pub rid: i64,
    ///
    #[prost(bool, tag = "11")]
    pub need_write_calender: bool,
    #[prost(oneof = "module_additional::Item", tags = "2, 3, 4, 5, 6, 8, 9, 10, 12, 13")]
    pub item: ::core::option::Option<module_additional::Item>,
}
/// Nested message and enum types in `ModuleAdditional`.
pub mod module_additional {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        /// 废弃
        #[prost(message, tag = "2")]
        Pgc(super::AdditionalPgc),
        ///
        #[prost(message, tag = "3")]
        Goods(super::AdditionGoods),
        /// 废弃
        #[prost(message, tag = "4")]
        Vote(super::AdditionVote),
        ///
        #[prost(message, tag = "5")]
        Common(super::AdditionCommon),
        ///
        #[prost(message, tag = "6")]
        Esport(super::AdditionEsport),
        /// 投票
        #[prost(message, tag = "8")]
        Vote2(super::AdditionVote2),
        ///
        #[prost(message, tag = "9")]
        Ugc(super::AdditionUgc),
        /// up主预约发布卡
        #[prost(message, tag = "10")]
        Up(super::AdditionUp),
        ///
        #[prost(message, tag = "12")]
        Article(super::AdditionArticle),
        ///
        #[prost(message, tag = "13")]
        Live(super::AdditionLiveRoom),
    }
}
/// 动态-发布人模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAuthor {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 时间标签
    #[prost(string, tag = "2")]
    pub ptime_label_text: ::prost::alloc::string::String,
    /// 用户详情
    #[prost(message, optional, tag = "3")]
    pub author: ::core::option::Option<UserInfo>,
    /// 装扮卡片
    #[prost(message, optional, tag = "4")]
    pub decorate_card: ::core::option::Option<DecorateCard>,
    /// 点击跳转链接
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    /// 右侧操作区域 - 三点样式
    #[prost(message, repeated, tag = "6")]
    pub tp_list: ::prost::alloc::vec::Vec<ThreePointItem>,
    /// 右侧操作区域样式枚举
    #[prost(enumeration = "ModuleAuthorBadgeType", tag = "7")]
    pub badge_type: i32,
    /// 右侧操作区域 - 按钮样式
    #[prost(message, optional, tag = "8")]
    pub badge_button: ::core::option::Option<ModuleAuthorBadgeButton>,
    /// 是否关注
    /// 1:关注 0:不关注 默认0，注：点赞列表使用，其他场景不使用该字段
    #[prost(int32, tag = "9")]
    pub attend: i32,
    /// 关注状态
    #[prost(message, optional, tag = "10")]
    pub relation: ::core::option::Option<Relation>,
    /// 右侧操作区域 - 提权样式
    #[prost(message, optional, tag = "11")]
    pub weight: ::core::option::Option<Weight>,
    /// 是否展示关注
    #[prost(bool, tag = "12")]
    pub show_follow: bool,
    /// 是否置顶
    #[prost(bool, tag = "13")]
    pub is_top: bool,
    /// ip属地
    #[prost(string, tag = "14")]
    pub ptime_location_text: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "15")]
    pub show_level: bool,
    ///
    #[prost(message, optional, tag = "16")]
    pub only_fans: ::core::option::Option<OnlyFans>,
}
/// 动态列表渲染部分-用户模块-按钮
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAuthorBadgeButton {
    /// 图标
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 文案
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 状态
    #[prost(int32, tag = "3")]
    pub state: i32,
    /// 物料ID
    #[prost(int64, tag = "4")]
    pub id: i64,
}
/// 动态列表-用户模块-转发模板
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAuthorForward {
    /// 展示标题
    #[prost(message, repeated, tag = "1")]
    pub title: ::prost::alloc::vec::Vec<ModuleAuthorForwardTitle>,
    /// 源卡片跳转链接
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    /// 用户uid
    #[prost(int64, tag = "3")]
    pub uid: i64,
    /// 时间标签
    #[prost(string, tag = "4")]
    pub ptime_label_text: ::prost::alloc::string::String,
    /// 是否展示关注
    #[prost(bool, tag = "5")]
    pub show_follow: bool,
    /// 源up主头像
    #[prost(string, tag = "6")]
    pub face_url: ::prost::alloc::string::String,
    /// 双向关系
    #[prost(message, optional, tag = "7")]
    pub relation: ::core::option::Option<Relation>,
    /// 右侧操作区域 - 三点样式
    #[prost(message, repeated, tag = "8")]
    pub tp_list: ::prost::alloc::vec::Vec<ThreePointItem>,
}
/// 动态列表-用户模块-转发模板-title部分
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAuthorForwardTitle {
    /// 文案
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}
/// 动态列表-通栏
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBanner {
    /// 模块标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 卡片类型
    #[prost(enumeration = "ModuleBannerType", tag = "2")]
    pub r#type: i32,
    /// 不感兴趣文案
    #[prost(string, tag = "4")]
    pub dislike_text: ::prost::alloc::string::String,
    /// 不感兴趣图标
    #[prost(string, tag = "5")]
    pub dislike_icon: ::prost::alloc::string::String,
    /// 卡片
    #[prost(oneof = "module_banner::Item", tags = "3")]
    pub item: ::core::option::Option<module_banner::Item>,
}
/// Nested message and enum types in `ModuleBanner`.
pub mod module_banner {
    /// 卡片
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        #[prost(message, tag = "3")]
        User(super::ModuleBannerUser),
    }
}
/// 动态通栏-用户
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBannerUser {
    /// 卡片列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<ModuleBannerUserItem>,
}
/// 动态通栏-推荐用户卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBannerUserItem {
    /// up主头像
    #[prost(string, tag = "1")]
    pub face: ::prost::alloc::string::String,
    /// up主昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// up主uid
    #[prost(int64, tag = "3")]
    pub uid: i64,
    /// 直播状态
    #[prost(enumeration = "LiveState", tag = "4")]
    pub live_state: i32,
    /// 认证信息
    #[prost(message, optional, tag = "5")]
    pub official: ::core::option::Option<OfficialVerify>,
    /// 大会员信息
    #[prost(message, optional, tag = "6")]
    pub vip: ::core::option::Option<VipInfo>,
    /// 标签信息
    #[prost(string, tag = "7")]
    pub label: ::prost::alloc::string::String,
    /// 按钮
    #[prost(message, optional, tag = "8")]
    pub button: ::core::option::Option<AdditionalButton>,
    /// 跳转地址
    #[prost(string, tag = "9")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "10")]
    pub relation: ::core::option::Option<Relation>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBlocked {
    ///
    #[prost(message, optional, tag = "1")]
    pub icon: ::core::option::Option<ImageSet>,
    ///
    #[prost(message, optional, tag = "2")]
    pub bg_img: ::core::option::Option<ImageSet>,
    ///
    #[prost(string, tag = "3")]
    pub hint_message: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub act_btn: ::core::option::Option<IconButton>,
    ///
    #[prost(enumeration = "MdlBlockedStyle", tag = "5")]
    pub block_style: i32,
    ///
    #[prost(string, tag = "6")]
    pub sub_hint_message: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub video_bottom_text_upper: ::core::option::Option<OneLineText>,
    ///
    #[prost(message, optional, tag = "8")]
    pub video_bottom_text_lower: ::core::option::Option<OneLineText>,
    ///
    #[prost(string, tag = "9")]
    pub archive_title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "10")]
    pub hint_message_one_line: ::core::option::Option<OneLineText>,
}
/// 底部模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleButtom {
    /// 计数模块
    #[prost(message, optional, tag = "1")]
    pub module_stat: ::core::option::Option<ModuleStat>,
    ///
    #[prost(bool, tag = "2")]
    pub comment_box: bool,
    ///
    #[prost(string, tag = "3")]
    pub comment_box_msg: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "module_buttom::InteractionIcon", repeated, tag = "4")]
    pub interaction_icons: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(message, repeated, tag = "5")]
    pub faces: ::prost::alloc::vec::Vec<InteractionFace>,
}
/// Nested message and enum types in `ModuleButtom`.
pub mod module_buttom {
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
    pub enum InteractionIcon {
        IconInvalid = 0,
        IconForward = 1,
        IconComment = 2,
        IconFavorite = 3,
        IconLike = 4,
    }
    impl InteractionIcon {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InteractionIcon::IconInvalid => "ICON_INVALID",
                InteractionIcon::IconForward => "ICON_FORWARD",
                InteractionIcon::IconComment => "ICON_COMMENT",
                InteractionIcon::IconFavorite => "ICON_FAVORITE",
                InteractionIcon::IconLike => "ICON_LIKE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ICON_INVALID" => Some(Self::IconInvalid),
                "ICON_FORWARD" => Some(Self::IconForward),
                "ICON_COMMENT" => Some(Self::IconComment),
                "ICON_FAVORITE" => Some(Self::IconFavorite),
                "ICON_LIKE" => Some(Self::IconLike),
                _ => None,
            }
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleButton {
    ///
    #[prost(message, optional, tag = "1")]
    pub btn: ::core::option::Option<IconButton>,
}
/// 评论外露模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleComment {
    /// 评论外露展示项
    #[prost(message, repeated, tag = "1")]
    pub cmt_show_item: ::prost::alloc::vec::Vec<CmtShowItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleCopyright {
    ///
    #[prost(string, tag = "1")]
    pub left_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub right_text: ::prost::alloc::string::String,
}
/// 动态-描述文字模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDesc {
    /// 描述信息(已按高亮拆分)
    #[prost(message, repeated, tag = "1")]
    pub desc: ::prost::alloc::vec::Vec<Description>,
    /// 点击跳转链接
    #[prost(string, tag = "2")]
    pub jump_uri: ::prost::alloc::string::String,
    /// 文本原本
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
}
/// 正文商品卡参数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDescGoods {
    /// 商品类型
    /// 1:淘宝 2:会员购
    #[prost(int32, tag = "1")]
    pub source_type: i32,
    /// 跳转链接
    #[prost(string, tag = "2")]
    pub jump_url: ::prost::alloc::string::String,
    /// schema_url
    #[prost(string, tag = "3")]
    pub schema_url: ::prost::alloc::string::String,
    /// item_id
    #[prost(int64, tag = "4")]
    pub item_id: i64,
    /// open_white_list
    #[prost(string, repeated, tag = "5")]
    pub open_white_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// use_web_v2
    #[prost(bool, tag = "6")]
    pub user_web_v2: bool,
    /// ad mark
    #[prost(string, tag = "7")]
    pub ad_mark: ::prost::alloc::string::String,
    /// schemaPackageName(Android用)
    #[prost(string, tag = "8")]
    pub schema_package_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub jump_type: i32,
    ///
    #[prost(string, tag = "10")]
    pub app_name: ::prost::alloc::string::String,
}
/// 动态-争议小黄条模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDispute {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 描述内容
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}
/// 动态-详情模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDynamic {
    /// 类型
    #[prost(enumeration = "ModuleDynamicType", tag = "1")]
    pub r#type: i32,
    #[prost(
        oneof = "module_dynamic::ModuleItem",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19"
    )]
    pub module_item: ::core::option::Option<module_dynamic::ModuleItem>,
}
/// Nested message and enum types in `ModuleDynamic`.
pub mod module_dynamic {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModuleItem {
        /// 稿件
        #[prost(message, tag = "2")]
        DynArchive(super::MdlDynArchive),
        /// pgc
        #[prost(message, tag = "3")]
        DynPgc(super::MdlDynPgc),
        /// 付费课程-系列
        #[prost(message, tag = "4")]
        DynCourSeason(super::MdlDynCourSeason),
        /// 付费课程-批次
        #[prost(message, tag = "5")]
        DynCourBatch(super::MdlDynCourBatch),
        /// 转发卡
        #[prost(message, tag = "6")]
        DynForward(super::MdlDynForward),
        /// 图文
        #[prost(message, tag = "7")]
        DynDraw(super::MdlDynDraw),
        /// 专栏
        #[prost(message, tag = "8")]
        DynArticle(super::MdlDynArticle),
        /// 音频
        #[prost(message, tag = "9")]
        DynMusic(super::MdlDynMusic),
        /// 通用卡方
        #[prost(message, tag = "10")]
        DynCommon(super::MdlDynCommon),
        /// 直播卡
        #[prost(message, tag = "11")]
        DynCommonLive(super::MdlDynLive),
        /// 播单
        #[prost(message, tag = "12")]
        DynMedialist(super::MdlDynMedialist),
        /// 小程序卡
        #[prost(message, tag = "13")]
        DynApplet(super::MdlDynApplet),
        /// 订阅卡
        #[prost(message, tag = "14")]
        DynSubscription(super::MdlDynSubscription),
        /// 直播推荐卡
        #[prost(message, tag = "15")]
        DynLiveRcmd(super::MdlDynLiveRcmd),
        /// UGC合集
        #[prost(message, tag = "16")]
        DynUgcSeason(super::MdlDynUgcSeason),
        /// 订阅卡
        #[prost(message, tag = "17")]
        DynSubscriptionNew(super::MdlDynSubscriptionNew),
        /// 课程
        #[prost(message, tag = "18")]
        DynCourBatchUp(super::MdlDynCourUp),
        /// 话题集合
        #[prost(message, tag = "19")]
        DynTopicSet(super::MdlDynTopicSet),
    }
}
/// 动态-小卡模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleExtend {
    /// 详情
    #[prost(message, repeated, tag = "1")]
    pub extend: ::prost::alloc::vec::Vec<ModuleExtendItem>,
    /// 模块整体跳转uri
    ///
    /// 废弃
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
/// 动态-拓展小卡模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleExtendItem {
    /// 类型
    #[prost(enumeration = "DynExtendType", tag = "1")]
    pub r#type: i32,
    /// 卡片详情
    #[prost(oneof = "module_extend_item::Extend", tags = "2, 3, 4, 5, 6, 7")]
    pub extend: ::core::option::Option<module_extend_item::Extend>,
}
/// Nested message and enum types in `ModuleExtendItem`.
pub mod module_extend_item {
    /// 卡片详情
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Extend {
        /// 废弃
        #[prost(message, tag = "2")]
        ExtInfoTopic(super::ExtInfoTopic),
        /// 废弃
        #[prost(message, tag = "3")]
        ExtInfoLbs(super::ExtInfoLbs),
        /// 废弃
        #[prost(message, tag = "4")]
        ExtInfoHot(super::ExtInfoHot),
        /// 废弃
        #[prost(message, tag = "5")]
        ExtInfoGame(super::ExtInfoGame),
        ///
        #[prost(message, tag = "6")]
        ExtInfoCommon(super::ExtInfoCommon),
        ///
        #[prost(message, tag = "7")]
        ExtInfoOgv(super::ExtInfoOgv),
    }
}
/// 动态-折叠模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleFold {
    /// 折叠分类
    #[prost(enumeration = "FoldType", tag = "1")]
    pub fold_type: i32,
    /// 折叠文案
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    /// 被折叠的动态
    #[prost(string, tag = "3")]
    pub fold_ids: ::prost::alloc::string::String,
    /// 被折叠的用户信息
    #[prost(message, repeated, tag = "4")]
    pub fold_users: ::prost::alloc::vec::Vec<UserInfo>,
    ///
    #[prost(message, optional, tag = "5")]
    pub topic_merged_resource: ::core::option::Option<TopicMergedResource>,
}
/// 外露交互模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleInteraction {
    /// 外露交互模块
    #[prost(message, repeated, tag = "1")]
    pub interaction_item: ::prost::alloc::vec::Vec<InteractionItem>,
}
/// 获取物料失败模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleItemNull {
    /// 图标
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 文案
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// 动态-点赞用户模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleLikeUser {
    /// 点赞用户
    #[prost(message, repeated, tag = "1")]
    pub like_users: ::prost::alloc::vec::Vec<LikeUser>,
    /// 文案
    #[prost(string, tag = "2")]
    pub display_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleNotice {
    ///
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub notice_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleOpusCollection {
    ///
    #[prost(message, optional, tag = "1")]
    pub collection_info: ::core::option::Option<OpusCollection>,
    ///
    #[prost(string, tag = "2")]
    pub title_upper: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub title_prefix_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub total_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleOpusSummary {
    ///
    #[prost(message, optional, tag = "1")]
    pub title: ::core::option::Option<Paragraph>,
    ///
    #[prost(message, optional, tag = "2")]
    pub summary: ::core::option::Option<Paragraph>,
    ///
    #[prost(string, tag = "3")]
    pub summary_jump_btn_text: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub covers: ::prost::alloc::vec::Vec<MdlDynDrawItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleParagraph {
    ///
    #[prost(message, optional, tag = "1")]
    pub paragraph: ::core::option::Option<Paragraph>,
    ///
    #[prost(bool, tag = "2")]
    pub is_article_title: bool,
    ///
    #[prost(message, optional, tag = "3")]
    pub para_spacing: ::core::option::Option<ParaSpacing>,
}
/// 推荐模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleRcmd {
    /// 用户头像
    #[prost(message, optional, tag = "1")]
    pub author: ::core::option::Option<RcmdAuthor>,
    /// 推荐卡片列表
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<RcmdItem>,
    /// 透传到客户端的埋点字段
    #[prost(string, tag = "3")]
    pub server_info: ::prost::alloc::string::String,
}
/// 相关推荐模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleRecommend {
    /// 模块标题
    #[prost(string, tag = "1")]
    pub module_title: ::prost::alloc::string::String,
    /// 图片
    #[prost(string, tag = "2")]
    pub image: ::prost::alloc::string::String,
    /// 标签
    #[prost(string, tag = "3")]
    pub tag: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "5")]
    pub jump_url: ::prost::alloc::string::String,
    /// 序列化的广告信息
    #[prost(message, repeated, tag = "6")]
    pub ad: ::prost::alloc::vec::Vec<super::super::super::super::google::protobuf::Any>,
}
/// 分享模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleShareInfo {
    /// 展示标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 分享组件列表
    #[prost(message, repeated, tag = "2")]
    pub share_channels: ::prost::alloc::vec::Vec<ShareChannel>,
    /// share_origin
    #[prost(string, tag = "3")]
    pub share_origin: ::prost::alloc::string::String,
    /// 业务id
    #[prost(string, tag = "4")]
    pub oid: ::prost::alloc::string::String,
    /// sid
    #[prost(string, tag = "5")]
    pub sid: ::prost::alloc::string::String,
}
/// 动态-计数模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleStat {
    /// 转发数
    #[prost(int64, tag = "1")]
    pub repost: i64,
    /// 点赞数
    #[prost(int64, tag = "2")]
    pub like: i64,
    /// 评论数
    #[prost(int64, tag = "3")]
    pub reply: i64,
    /// 点赞拓展信息
    #[prost(message, optional, tag = "4")]
    pub like_info: ::core::option::Option<LikeInfo>,
    /// 禁评
    #[prost(bool, tag = "5")]
    pub no_comment: bool,
    /// 禁转
    #[prost(bool, tag = "6")]
    pub no_forward: bool,
    /// 点击评论跳转链接
    #[prost(string, tag = "7")]
    pub reply_url: ::prost::alloc::string::String,
    /// 禁评文案
    #[prost(string, tag = "8")]
    pub no_comment_text: ::prost::alloc::string::String,
    /// 禁转文案
    #[prost(string, tag = "9")]
    pub no_forward_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleStory {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<StoryItem>,
    ///
    #[prost(bool, tag = "3")]
    pub show_publish_entrance: bool,
    ///
    #[prost(int64, tag = "4")]
    pub fold_state: i64,
    ///
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub publish_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleTextNotice {
    ///
    #[prost(message, optional, tag = "1")]
    pub notice: ::core::option::Option<OneLineText>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleTitle {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub right_btn: ::core::option::Option<IconButton>,
    ///
    #[prost(int32, tag = "3")]
    pub title_style: i32,
}
/// 顶部模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleTop {
    /// 三点模块
    #[prost(message, repeated, tag = "1")]
    pub tp_list: ::prost::alloc::vec::Vec<ThreePointItem>,
    ///
    #[prost(message, optional, tag = "2")]
    pub archive: ::core::option::Option<MdlDynArchive>,
    ///
    #[prost(message, optional, tag = "3")]
    pub author: ::core::option::Option<ModuleAuthor>,
    ///
    #[prost(bool, tag = "4")]
    pub hidden_nav_bar: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleTopic {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleTopicBrief {
    ///
    #[prost(message, optional, tag = "1")]
    pub topic: ::core::option::Option<TopicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleTopicDetailsExt {
    ///
    #[prost(string, tag = "1")]
    pub comment_guide: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleTopTag {
    ///
    #[prost(string, tag = "1")]
    pub tag_name: ::prost::alloc::string::String,
}
/// 认证名牌
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nameplate {
    /// nid
    #[prost(int64, tag = "1")]
    pub nid: i64,
    /// 名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 图片地址
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    /// 小图地址
    #[prost(string, tag = "4")]
    pub image_small: ::prost::alloc::string::String,
    /// 等级
    #[prost(string, tag = "5")]
    pub level: ::prost::alloc::string::String,
    /// 获取条件
    #[prost(string, tag = "6")]
    pub condition: ::prost::alloc::string::String,
}
/// 最新ep
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEp {
    /// 最新话epid
    #[prost(int32, tag = "1")]
    pub id: i32,
    /// 更新至XX话
    #[prost(string, tag = "2")]
    pub index_show: ::prost::alloc::string::String,
    /// 更新剧集的封面
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftInfo {
    ///
    #[prost(enumeration = "NftRegionType", tag = "1")]
    pub region_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub region_icon: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "NftShowStatus", tag = "3")]
    pub region_show_status: i32,
}
/// 空响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoReply {}
/// 空请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoReq {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoteVideoTs {
    ///
    #[prost(int64, tag = "1")]
    pub cid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub oid_type: i64,
    ///
    #[prost(int64, tag = "3")]
    pub status: i64,
    ///
    #[prost(int64, tag = "4")]
    pub index: i64,
    ///
    #[prost(int64, tag = "5")]
    pub seconds: i64,
    ///
    #[prost(int64, tag = "6")]
    pub cid_count: i64,
    ///
    #[prost(string, tag = "7")]
    pub key: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "9")]
    pub epid: i64,
    ///
    #[prost(string, tag = "8")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialAccountInfo {
    ///
    #[prost(message, optional, tag = "1")]
    pub author: ::core::option::Option<UserInfo>,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub relation: ::core::option::Option<Relation>,
    ///
    #[prost(string, tag = "5")]
    pub desc_text1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub desc_text2: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialAccountsReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<OfficialAccountInfo>,
    ///
    #[prost(bool, tag = "2")]
    pub has_more: bool,
    ///
    #[prost(int64, tag = "3")]
    pub offset: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialAccountsReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub offset: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialDynamicsReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<OfficialItem>,
    ///
    #[prost(int64, tag = "2")]
    pub offset: i64,
    ///
    #[prost(bool, tag = "3")]
    pub has_more: bool,
    ///
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialDynamicsReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub offset: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialItem {
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    #[prost(oneof = "official_item::RcmdItem", tags = "2, 3")]
    pub rcmd_item: ::core::option::Option<official_item::RcmdItem>,
}
/// Nested message and enum types in `OfficialItem`.
pub mod official_item {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RcmdItem {
        #[prost(message, tag = "2")]
        RcmdArchive(super::OfficialRcmdArchive),
        #[prost(message, tag = "3")]
        RcmdDynamic(super::OfficialRcmdDynamic),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialRcmdArchive {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub desc_icon1: i32,
    ///
    #[prost(string, tag = "5")]
    pub desc_text1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub desc_icon2: i32,
    ///
    #[prost(string, tag = "7")]
    pub desc_text2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub reason: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "9")]
    pub show_three_point: bool,
    ///
    #[prost(string, tag = "10")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "12")]
    pub mid: i64,
    ///
    #[prost(string, tag = "13")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "14")]
    pub dynamic_id: i64,
    ///
    #[prost(int64, tag = "15")]
    pub cid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialRcmdDynamic {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover_right_top_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub desc_icon1: i32,
    ///
    #[prost(string, tag = "5")]
    pub desc_text1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub desc_icon2: i32,
    ///
    #[prost(string, tag = "7")]
    pub desc_text2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub reason: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub dynamic_id: i64,
    ///
    #[prost(int64, tag = "11")]
    pub mid: i64,
    ///
    #[prost(string, tag = "12")]
    pub user_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "13")]
    pub rid: i64,
}
/// 认证信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialVerify {
    /// 127:未认证 0:个人 1:机构
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 认证描述
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    /// 是否关注
    #[prost(int32, tag = "3")]
    pub is_atten: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneLineText {
    ///
    #[prost(message, repeated, tag = "1")]
    pub texts: ::prost::alloc::vec::Vec<TextWithPriority>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnlyFans {
    ///
    #[prost(bool, tag = "1")]
    pub is_only_fans: bool,
    ///
    #[prost(message, optional, tag = "2")]
    pub badge: ::core::option::Option<IconBadge>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnlyFansProperty {
    ///
    #[prost(bool, tag = "1")]
    pub has_privilege: bool,
    ///
    #[prost(bool, tag = "2")]
    pub is_only_fans: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpusCollection {
    ///
    #[prost(int64, tag = "1")]
    pub collection_id: i64,
    ///
    #[prost(message, optional, tag = "2")]
    pub title: ::core::option::Option<OneLineText>,
    ///
    #[prost(string, tag = "3")]
    pub detail_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub intro: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "5")]
    pub all_items: ::prost::alloc::vec::Vec<OpusCollectionItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpusCollectionItem {
    ///
    #[prost(int64, tag = "1")]
    pub opus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub pub_time: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "5")]
    pub is_selected_highlight: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paragraph {
    ///
    #[prost(enumeration = "paragraph::ParagraphType", tag = "1")]
    pub para_type: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub para_format: ::core::option::Option<paragraph::ParagraphFormat>,
    ///
    #[prost(oneof = "paragraph::Content", tags = "3, 4, 5, 6")]
    pub content: ::core::option::Option<paragraph::Content>,
}
/// Nested message and enum types in `Paragraph`.
pub mod paragraph {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListFormat {
        ///
        #[prost(int32, tag = "1")]
        pub level: i32,
        ///
        #[prost(int32, tag = "2")]
        pub order: i32,
        ///
        #[prost(string, tag = "3")]
        pub theme: ::prost::alloc::string::String,
    }
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParagraphFormat {
        ///
        #[prost(enumeration = "ParagraphAlign", tag = "1")]
        pub align: i32,
        ///
        #[prost(message, optional, tag = "2")]
        pub list_format: ::core::option::Option<ListFormat>,
    }
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
    pub enum ParagraphAlign {
        Left = 0,
        Middle = 1,
        Right = 2,
    }
    impl ParagraphAlign {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ParagraphAlign::Left => "LEFT",
                ParagraphAlign::Middle => "MIDDLE",
                ParagraphAlign::Right => "RIGHT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LEFT" => Some(Self::Left),
                "MIDDLE" => Some(Self::Middle),
                "RIGHT" => Some(Self::Right),
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
    pub enum ParagraphType {
        Invalid = 0,
        Text = 1,
        Pictures = 2,
        Line = 3,
        Reference = 4,
        SortedList = 5,
        UnsortedList = 6,
        LinkCard = 7,
    }
    impl ParagraphType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ParagraphType::Invalid => "INVALID",
                ParagraphType::Text => "TEXT",
                ParagraphType::Pictures => "PICTURES",
                ParagraphType::Line => "LINE",
                ParagraphType::Reference => "REFERENCE",
                ParagraphType::SortedList => "SORTED_LIST",
                ParagraphType::UnsortedList => "UNSORTED_LIST",
                ParagraphType::LinkCard => "LINK_CARD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVALID" => Some(Self::Invalid),
                "TEXT" => Some(Self::Text),
                "PICTURES" => Some(Self::Pictures),
                "LINE" => Some(Self::Line),
                "REFERENCE" => Some(Self::Reference),
                "SORTED_LIST" => Some(Self::SortedList),
                "UNSORTED_LIST" => Some(Self::UnsortedList),
                "LINK_CARD" => Some(Self::LinkCard),
                _ => None,
            }
        }
    }
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "3")]
        Text(super::TextParagraph),
        #[prost(message, tag = "4")]
        Pic(super::PicParagraph),
        #[prost(message, tag = "5")]
        Line(super::LineParagraph),
        #[prost(message, tag = "6")]
        LinkCard(super::CardParagraph),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParaSpacing {
    ///
    #[prost(double, tag = "1")]
    pub spacing_before_para: f64,
    ///
    #[prost(double, tag = "2")]
    pub spacing_after_para: f64,
    ///
    #[prost(double, tag = "3")]
    pub line_spacing: f64,
}
/// PGC单季信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PgcSeason {
    /// 是否完结
    #[prost(int32, tag = "1")]
    pub is_finish: i32,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 类型
    #[prost(int32, tag = "3")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PicParagraph {
    ///
    #[prost(message, optional, tag = "1")]
    pub pics: ::core::option::Option<MdlDynDraw>,
    ///
    #[prost(enumeration = "pic_paragraph::PicParagraphStyle", tag = "2")]
    pub style: i32,
}
/// Nested message and enum types in `PicParagraph`.
pub mod pic_paragraph {
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
    pub enum PicParagraphStyle {
        Invalid = 0,
        NineCell = 1,
        BigScroll = 2,
    }
    impl PicParagraphStyle {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PicParagraphStyle::Invalid => "INVALID",
                PicParagraphStyle::NineCell => "NINE_CELL",
                PicParagraphStyle::BigScroll => "BIG_SCROLL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVALID" => Some(Self::Invalid),
                "NINE_CELL" => Some(Self::NineCell),
                "BIG_SCROLL" => Some(Self::BigScroll),
                _ => None,
            }
        }
    }
}
/// 秒开通用参数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayurlParam {
    /// 清晰度
    #[prost(int32, tag = "1")]
    pub qn: i32,
    /// 流版本
    #[prost(int32, tag = "2")]
    pub fnver: i32,
    /// 流类型
    #[prost(int32, tag = "3")]
    pub fnval: i32,
    /// 是否强制使用域名
    #[prost(int32, tag = "4")]
    pub force_host: i32,
    /// 是否4k
    #[prost(int32, tag = "5")]
    pub fourk: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Popup {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdArchive {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    /// 视频封面展示项 1
    #[prost(enumeration = "CoverIcon", tag = "3")]
    pub cover_left_icon_1: i32,
    /// 视频封面展示项 1
    #[prost(string, tag = "4")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    /// 秒开地址
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    /// 是否PGC
    #[prost(bool, tag = "6")]
    pub is_pgc: bool,
    /// aid
    #[prost(int64, tag = "7")]
    pub aid: i64,
    ///
    #[prost(message, optional, tag = "8")]
    pub badge: ::core::option::Option<IconBadge>,
    ///
    #[prost(int32, tag = "9")]
    pub cover_left_icon2: i32,
    ///
    #[prost(string, tag = "10")]
    pub cover_left_text2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "11")]
    pub cover_left_icon3: i32,
    ///
    #[prost(string, tag = "12")]
    pub cover_left_text3: ::prost::alloc::string::String,
}
/// 推荐UP主用户模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdAuthor {
    /// 用户详情
    #[prost(message, optional, tag = "1")]
    pub author: ::core::option::Option<UserInfo>,
    /// 描述：粉丝数、推荐理由
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    /// 关注状态
    #[prost(message, optional, tag = "3")]
    pub relation: ::core::option::Option<Relation>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdCampusBrief {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub campus_badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
}
/// 推荐卡片列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdItem {
    /// 卡片类型
    #[prost(enumeration = "RcmdType", tag = "1")]
    pub r#type: i32,
    /// 卡片列表
    #[prost(oneof = "rcmd_item::RcmdItem", tags = "2")]
    pub rcmd_item: ::core::option::Option<rcmd_item::RcmdItem>,
}
/// Nested message and enum types in `RcmdItem`.
pub mod rcmd_item {
    /// 卡片列表
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RcmdItem {
        ///
        #[prost(message, tag = "2")]
        RcmdArchive(super::RcmdArchive),
    }
}
/// 分区聚类推荐选项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdOption {
    /// 视频是否展示标题
    #[prost(bool, tag = "1")]
    pub show_title: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdReason {
    ///
    #[prost(string, tag = "1")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "RcmdReasonStyle", tag = "2")]
    pub style: i32,
    ///
    #[prost(string, tag = "3")]
    pub rcmd_reason: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub up_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdTopButton {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}
/// 推荐up主入参
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdUPsParam {
    #[prost(int64, tag = "1")]
    pub dislike_ts: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactionListItem {
    /// 用户信息
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<UserInfo>,
    /// 关注关系
    #[prost(message, optional, tag = "2")]
    pub relation: ::core::option::Option<Relation>,
    /// 显示文字
    #[prost(string, tag = "3")]
    pub act_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub rcmd_reason: ::prost::alloc::string::String,
}
/// 新版动态转发点赞列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactionListReply {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 列表
    #[prost(message, repeated, tag = "2")]
    pub list: ::prost::alloc::vec::Vec<ReactionListItem>,
    /// 偏移
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    /// 是否还有更多
    #[prost(bool, tag = "4")]
    pub has_more: bool,
}
/// 新版动态转发点赞列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactionListReq {
    /// 动态ID
    #[prost(int64, tag = "1")]
    pub dynamic_id: i64,
    /// 动态类型
    #[prost(int64, tag = "2")]
    pub dyn_type: i64,
    /// 业务方资源id
    #[prost(int64, tag = "3")]
    pub rid: i64,
    /// 偏移,使用上一页回包中的offset字段；第一页不传。
    #[prost(string, tag = "4")]
    pub offset: ::prost::alloc::string::String,
}
/// 关注关系
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    /// 关注状态
    #[prost(enumeration = "RelationStatus", tag = "1")]
    pub status: i32,
    /// 关注
    #[prost(int32, tag = "2")]
    pub is_follow: i32,
    /// 被关注
    #[prost(int32, tag = "3")]
    pub is_followed: i32,
    /// 文案
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
}
/// 转发列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepostListReq {
    /// 动态ID
    #[prost(string, tag = "1")]
    pub dynamic_id: ::prost::alloc::string::String,
    /// 动态类型
    #[prost(int64, tag = "2")]
    pub dyn_type: i64,
    /// 业务方资源id
    #[prost(int64, tag = "3")]
    pub rid: i64,
    /// 偏移,使用上一页回包中的offset字段；第一页不传。
    #[prost(string, tag = "4")]
    pub offset: ::prost::alloc::string::String,
    /// 来源
    #[prost(string, tag = "5")]
    pub from: ::prost::alloc::string::String,
    /// 评论类型
    #[prost(enumeration = "RepostType", tag = "6")]
    pub repost_type: i32,
}
/// 转发列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepostListRsp {
    /// 列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    /// 偏移
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    /// 是否还有更多
    #[prost(bool, tag = "3")]
    pub has_more: bool,
    /// 转发总数
    #[prost(int64, tag = "4")]
    pub total_count: i64,
    /// 评论类型
    #[prost(enumeration = "RepostType", tag = "5")]
    pub repost_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchoolRecommendReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<CampusInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchoolRecommendReq {
    ///
    #[prost(float, tag = "1")]
    pub lat: f32,
    ///
    #[prost(float, tag = "2")]
    pub lng: f32,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "3")]
    pub from_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchoolSearchReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<CampusInfo>,
    ///
    #[prost(message, optional, tag = "2")]
    pub toast: ::core::option::Option<SearchToast>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchoolSearchReq {
    ///
    #[prost(string, tag = "1")]
    pub keyword: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "2")]
    pub from_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchChannel {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub more_button: ::core::option::Option<SearchTopicButton>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub channels: ::prost::alloc::vec::Vec<ChannelInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchInfo {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    ///
    #[prost(string, tag = "3")]
    pub track_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub total: i64,
    ///
    #[prost(bool, tag = "5")]
    pub has_more: bool,
    ///
    #[prost(string, tag = "6")]
    pub version: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchToast {
    ///
    #[prost(string, tag = "1")]
    pub desc_text1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub desc_text2: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTopic {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub more_button: ::core::option::Option<SearchTopicButton>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<SearchTopicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTopicButton {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub jump_uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTopicItem {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub topic_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "5")]
    pub is_activity: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDecisionReq {
    ///
    #[prost(int32, tag = "1")]
    pub result: i32,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "2")]
    pub from_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRecentCampusReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "3")]
    pub from_type: i32,
}
/// 分享渠道组件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareChannel {
    /// 分享名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 分享按钮图片
    #[prost(string, tag = "2")]
    pub image: ::prost::alloc::string::String,
    /// 分享渠道
    #[prost(string, tag = "3")]
    pub channel: ::prost::alloc::string::String,
    /// 预约卡分享图信息，仅分享有预约信息的动态时存在
    #[prost(message, optional, tag = "4")]
    pub reserve: ::core::option::Option<ShareReserve>,
}
/// 预约卡分享图信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareReserve {
    /// 展示标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 描述(时间+类型)
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    /// 二维码附带icon
    #[prost(string, tag = "3")]
    pub qr_code_icon: ::prost::alloc::string::String,
    /// 二维码附带文本
    #[prost(string, tag = "4")]
    pub qr_code_text: ::prost::alloc::string::String,
    /// 二维码url
    #[prost(string, tag = "5")]
    pub qr_code_url: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub user_info: ::core::option::Option<AdditionUserInfo>,
}
/// 排序类型
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortType {
    /// 排序策略
    /// 1:推荐排序 2:最常访问 3:最近关注
    #[prost(int32, tag = "1")]
    pub sort_type: i32,
    /// 排序策略名称
    #[prost(string, tag = "2")]
    pub sort_type_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoryArchive {
    ///
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub aid: i64,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub dimension: ::core::option::Option<Dimension>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoryItem {
    ///
    #[prost(message, optional, tag = "1")]
    pub author: ::core::option::Option<UserInfo>,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub status: i64,
    ///
    #[prost(int32, tag = "4")]
    pub r#type: i32,
    #[prost(oneof = "story_item::RcmdItem", tags = "5")]
    pub rcmd_item: ::core::option::Option<story_item::RcmdItem>,
}
/// Nested message and enum types in `StoryItem`.
pub mod story_item {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RcmdItem {
        ///
        #[prost(message, tag = "5")]
        StoryArchive(super::StoryArchive),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCampusReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub campus_name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "CampusReqFromType", tag = "3")]
    pub from_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextNode {
    ///
    #[prost(enumeration = "text_node::TextNodeType", tag = "1")]
    pub node_type: i32,
    #[prost(string, tag = "2")]
    pub raw_text: ::prost::alloc::string::String,
    ///
    #[prost(oneof = "text_node::Text", tags = "3, 4, 5")]
    pub text: ::core::option::Option<text_node::Text>,
}
/// Nested message and enum types in `TextNode`.
pub mod text_node {
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
    pub enum TextNodeType {
        Invalid = 0,
        Words = 1,
        Emote = 2,
        At = 3,
        BizLink = 4,
    }
    impl TextNodeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TextNodeType::Invalid => "INVALID",
                TextNodeType::Words => "WORDS",
                TextNodeType::Emote => "EMOTE",
                TextNodeType::At => "AT",
                TextNodeType::BizLink => "BIZ_LINK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVALID" => Some(Self::Invalid),
                "WORDS" => Some(Self::Words),
                "EMOTE" => Some(Self::Emote),
                "AT" => Some(Self::At),
                "BIZ_LINK" => Some(Self::BizLink),
                _ => None,
            }
        }
    }
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Text {
        #[prost(message, tag = "3")]
        Word(super::WordNode),
        #[prost(message, tag = "4")]
        Emote(super::EmoteNode),
        #[prost(message, tag = "5")]
        Link(super::LinkNode),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextParagraph {
    ///
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<TextNode>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextWithPriority {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub priority: i64,
}
/// 三点-关注
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointAttention {
    /// attention icon
    #[prost(string, tag = "1")]
    pub attention_icon: ::prost::alloc::string::String,
    /// 关注时显示的文案
    #[prost(string, tag = "2")]
    pub attention_text: ::prost::alloc::string::String,
    /// not attention icon
    #[prost(string, tag = "3")]
    pub not_attention_icon: ::prost::alloc::string::String,
    /// 未关注时显示的文案
    #[prost(string, tag = "4")]
    pub not_attention_text: ::prost::alloc::string::String,
    /// 当前关注状态
    #[prost(enumeration = "ThreePointAttentionStatus", tag = "5")]
    pub status: i32,
}
/// 三点-自动播放 旧版不维护
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointAutoPlay {
    /// open icon
    #[prost(string, tag = "1")]
    pub open_icon: ::prost::alloc::string::String,
    /// 开启时显示文案
    #[prost(string, tag = "2")]
    pub open_text: ::prost::alloc::string::String,
    /// close icon
    #[prost(string, tag = "3")]
    pub close_icon: ::prost::alloc::string::String,
    /// 关闭时显示文案
    #[prost(string, tag = "4")]
    pub close_text: ::prost::alloc::string::String,
    /// 开启时显示文案v2
    #[prost(string, tag = "5")]
    pub open_text_v2: ::prost::alloc::string::String,
    /// 关闭时显示文案v2
    #[prost(string, tag = "6")]
    pub close_text_v2: ::prost::alloc::string::String,
    /// 仅wifi/免流 icon
    #[prost(string, tag = "7")]
    pub only_icon: ::prost::alloc::string::String,
    /// 仅wifi/免流 文案
    #[prost(string, tag = "8")]
    pub only_text: ::prost::alloc::string::String,
    /// open icon v2
    #[prost(string, tag = "9")]
    pub open_icon_v2: ::prost::alloc::string::String,
    /// close icon v2
    #[prost(string, tag = "10")]
    pub close_icon_v2: ::prost::alloc::string::String,
}
/// 三点-评论
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointComment {
    /// 精选评论区功能
    #[prost(message, optional, tag = "1")]
    pub up_selection: ::core::option::Option<CommentDetail>,
    /// up关闭评论区功能
    #[prost(message, optional, tag = "2")]
    pub up_close: ::core::option::Option<CommentDetail>,
    /// icon
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
}
/// 三点-默认结构(使用此背景、举报、删除)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointDefault {
    /// icon
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// id
    #[prost(string, tag = "4")]
    pub id: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub toast: ::core::option::Option<ThreePointDefaultToast>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointDefaultToast {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
/// 三点-不感兴趣
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointDislike {
    /// icon
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
/// 三点-收藏
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointFavorite {
    /// icon
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 物料ID
    #[prost(int64, tag = "3")]
    pub id: i64,
    /// 是否订阅
    #[prost(bool, tag = "4")]
    pub is_favourite: bool,
    /// 取消收藏图标
    #[prost(string, tag = "5")]
    pub cancel_icon: ::prost::alloc::string::String,
    /// 取消收藏文案
    #[prost(string, tag = "6")]
    pub cancel_title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointHide {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub interactive: ::core::option::Option<ThreePointHideInteractive>,
    ///
    #[prost(int64, tag = "4")]
    pub blook_fid: i64,
    ///
    #[prost(string, tag = "5")]
    pub blook_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointHideInteractive {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub confirm: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cancel: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub toast: ::prost::alloc::string::String,
}
/// 三点Item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointItem {
    /// 类型
    #[prost(enumeration = "ThreePointType", tag = "1")]
    pub r#type: i32,
    #[prost(
        oneof = "three_point_item::Item",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub item: ::core::option::Option<three_point_item::Item>,
}
/// Nested message and enum types in `ThreePointItem`.
pub mod three_point_item {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        /// 默认结构
        #[prost(message, tag = "2")]
        Default(super::ThreePointDefault),
        /// 自动播放
        #[prost(message, tag = "3")]
        AutoPlayer(super::ThreePointAutoPlay),
        /// 分享
        #[prost(message, tag = "4")]
        Share(super::ThreePointShare),
        /// 关注
        #[prost(message, tag = "5")]
        Attention(super::ThreePointAttention),
        /// 稍后在看
        #[prost(message, tag = "6")]
        Wait(super::ThreePointWait),
        /// 不感兴趣
        #[prost(message, tag = "7")]
        Dislike(super::ThreePointDislike),
        /// 收藏
        #[prost(message, tag = "8")]
        Favorite(super::ThreePointFavorite),
        /// 置顶
        #[prost(message, tag = "9")]
        Top(super::ThreePointTop),
        /// 评论
        #[prost(message, tag = "10")]
        Comment(super::ThreePointComment),
        ///
        #[prost(message, tag = "11")]
        Hide(super::ThreePointHide),
        ///
        #[prost(message, tag = "12")]
        TopicIrrelevant(super::ThreePointTopicIrrelevant),
    }
}
/// 三点-分享
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointShare {
    /// icon
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 分享渠道
    #[prost(message, repeated, tag = "3")]
    pub channel: ::prost::alloc::vec::Vec<ThreePointShareChannel>,
    /// 分享渠道名
    #[prost(string, tag = "4")]
    pub channel_name: ::prost::alloc::string::String,
    /// 预约卡分享图信息，仅分享有预约信息的动态时存在
    #[prost(message, optional, tag = "5")]
    pub reserve: ::core::option::Option<ShareReserve>,
}
/// 三点-分享渠道
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointShareChannel {
    /// icon
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
/// 三点-置顶
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointTop {
    /// icon
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 状态
    #[prost(enumeration = "TopType", tag = "3")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointTopicIrrelevant {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub topic_id: i64,
    ///
    #[prost(int64, tag = "5")]
    pub res_id: i64,
    ///
    #[prost(int64, tag = "6")]
    pub res_type: i64,
    ///
    #[prost(string, tag = "7")]
    pub reason: ::prost::alloc::string::String,
}
/// 三点-稍后在看
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointWait {
    /// addition icon
    #[prost(string, tag = "1")]
    pub addition_icon: ::prost::alloc::string::String,
    /// 已添加时的文案
    #[prost(string, tag = "2")]
    pub addition_text: ::prost::alloc::string::String,
    /// no addition icon
    #[prost(string, tag = "3")]
    pub no_addition_icon: ::prost::alloc::string::String,
    /// 未添加时的文案
    #[prost(string, tag = "4")]
    pub no_addition_text: ::prost::alloc::string::String,
    /// avid
    #[prost(int64, tag = "5")]
    pub id: i64,
}
/// 顶部预约卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopAdditionUp {
    /// 预约卡
    #[prost(message, repeated, tag = "1")]
    pub up: ::prost::alloc::vec::Vec<AdditionUp>,
    /// 折叠数量，大于多少个进行折叠
    #[prost(int32, tag = "2")]
    pub has_fold: i32,
}
/// 话题广场操作按钮
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicButton {
    /// 按钮图标
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 按钮文案
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 跳转
    #[prost(string, tag = "3")]
    pub jump_uri: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "4")]
    pub red_dot: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicItem {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub topic_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub desc2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub rcmd_desc: ::prost::alloc::string::String,
}
/// 综合页-话题广场
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicList {
    /// 模块标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 话题列表
    #[prost(message, repeated, tag = "2")]
    pub topic_list_item: ::prost::alloc::vec::Vec<TopicListItem>,
    /// 发起活动
    #[prost(message, optional, tag = "3")]
    pub act_button: ::core::option::Option<TopicButton>,
    /// 查看更多
    #[prost(message, optional, tag = "4")]
    pub more_button: ::core::option::Option<TopicButton>,
    /// 透传服务端上报
    #[prost(string, tag = "5")]
    pub server_info: ::prost::alloc::string::String,
}
/// 综合页-话题广场-话题
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicListItem {
    /// 前置图标
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 前置图标文案
    #[prost(string, tag = "2")]
    pub icon_title: ::prost::alloc::string::String,
    /// 话题id
    #[prost(int64, tag = "3")]
    pub topic_id: i64,
    /// 话题名
    #[prost(string, tag = "4")]
    pub topic_name: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
    /// 卡片位次
    #[prost(int64, tag = "6")]
    pub pos: i64,
    /// 透传服务端上报
    #[prost(string, tag = "7")]
    pub server_info: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub head_icon_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "9")]
    pub up_mid: i64,
    ///
    #[prost(string, tag = "10")]
    pub tail_icon_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub extension: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicListReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<TopicItem>,
    ///
    #[prost(bool, tag = "2")]
    pub has_more: bool,
    ///
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicListReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicMergedResource {
    ///
    #[prost(int32, tag = "1")]
    pub merge_type: i32,
    ///
    #[prost(int32, tag = "2")]
    pub merged_res_cnt: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicRcmdCard {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub topic_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub button: ::core::option::Option<CampusLabel>,
    ///
    #[prost(string, tag = "5")]
    pub desc1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub desc2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub update_desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicSquareInfo {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub button: ::core::option::Option<CampusLabel>,
    ///
    #[prost(message, optional, tag = "3")]
    pub rcmd: ::core::option::Option<TopicRcmdCard>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicSquareReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<TopicSquareInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicSquareReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
}
/// 综合页-无关注列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unfollow {
    /// 标题展示文案
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 无关注列表
    #[prost(message, repeated, tag = "2")]
    pub list: ::prost::alloc::vec::Vec<UnfollowUserItem>,
    /// trackID
    #[prost(string, tag = "3")]
    pub track_id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnfollowMatchReq {
    ///
    #[prost(int64, tag = "1")]
    pub cid: i64,
}
/// 综合页-无关注列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnfollowUserItem {
    /// 是否有更新
    #[prost(bool, tag = "1")]
    pub has_update: bool,
    /// up主头像
    #[prost(string, tag = "2")]
    pub face: ::prost::alloc::string::String,
    /// up主昵称
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// up主uid
    #[prost(int64, tag = "4")]
    pub uid: i64,
    /// 排序字段 从1开始
    #[prost(int32, tag = "5")]
    pub pos: i32,
    /// 直播状态
    #[prost(enumeration = "LiveState", tag = "6")]
    pub live_state: i32,
    /// 认证信息
    #[prost(message, optional, tag = "7")]
    pub official: ::core::option::Option<OfficialVerify>,
    /// 大会员信息
    #[prost(message, optional, tag = "8")]
    pub vip: ::core::option::Option<VipInfo>,
    /// up介绍
    #[prost(string, tag = "9")]
    pub sign: ::prost::alloc::string::String,
    /// 标签信息
    #[prost(string, tag = "10")]
    pub label: ::prost::alloc::string::String,
    /// 按钮
    #[prost(message, optional, tag = "11")]
    pub button: ::core::option::Option<AdditionalButton>,
    /// 跳转地址
    #[prost(string, tag = "12")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTabSettingReq {
    ///
    #[prost(enumeration = "HomePageTabSttingStatus", tag = "1")]
    pub status: i32,
}
/// 动态顶部up列表-up主信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpListItem {
    /// 是否有更新
    #[prost(bool, tag = "1")]
    pub has_update: bool,
    /// up主头像
    #[prost(string, tag = "2")]
    pub face: ::prost::alloc::string::String,
    /// up主昵称
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// up主uid
    #[prost(int64, tag = "4")]
    pub uid: i64,
    /// 排序字段 从1开始
    #[prost(int64, tag = "5")]
    pub pos: i64,
    /// 用户类型
    #[prost(enumeration = "UserItemType", tag = "6")]
    pub user_item_type: i32,
    /// 直播头像样式-日
    #[prost(message, optional, tag = "7")]
    pub display_style_day: ::core::option::Option<UserItemStyle>,
    /// 直播头像样式-夜
    #[prost(message, optional, tag = "8")]
    pub display_style_night: ::core::option::Option<UserItemStyle>,
    /// 直播埋点
    #[prost(int64, tag = "9")]
    pub style_id: i64,
    /// 直播状态
    #[prost(enumeration = "LiveState", tag = "10")]
    pub live_state: i32,
    /// 分割线
    #[prost(bool, tag = "11")]
    pub separator: bool,
    /// 跳转
    #[prost(string, tag = "12")]
    pub uri: ::prost::alloc::string::String,
    /// UP主预约上报使用
    #[prost(bool, tag = "13")]
    pub is_recall: bool,
    ///
    #[prost(message, optional, tag = "14")]
    pub update_icon: ::core::option::Option<IconBadge>,
    ///
    #[prost(string, tag = "15")]
    pub live_rcmd_reason: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "16")]
    pub live_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "17")]
    pub personal_extra: ::prost::alloc::string::String,
}
/// 最常访问-查看更多
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpListMoreLabel {
    /// 文案
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
/// 用户信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 用户昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 用户头像
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    /// 认证信息
    #[prost(message, optional, tag = "4")]
    pub official: ::core::option::Option<OfficialVerify>,
    /// 大会员信息
    #[prost(message, optional, tag = "5")]
    pub vip: ::core::option::Option<VipInfo>,
    /// 直播信息
    #[prost(message, optional, tag = "6")]
    pub live: ::core::option::Option<LiveInfo>,
    /// 空间页跳转链接
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    /// 挂件信息
    #[prost(message, optional, tag = "8")]
    pub pendant: ::core::option::Option<UserPendant>,
    /// 认证名牌
    #[prost(message, optional, tag = "9")]
    pub nameplate: ::core::option::Option<Nameplate>,
    /// 用户等级
    #[prost(int32, tag = "10")]
    pub level: i32,
    /// 用户简介
    #[prost(string, tag = "11")]
    pub sign: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "12")]
    pub face_nft: i32,
    ///
    #[prost(int32, tag = "13")]
    pub face_nft_new: i32,
    ///
    #[prost(message, optional, tag = "14")]
    pub nft_info: ::core::option::Option<NftInfo>,
    ///
    #[prost(int32, tag = "15")]
    pub is_senior_member: i32,
    ///
    #[prost(message, optional, tag = "16")]
    pub avatar: ::core::option::Option<
        super::super::super::dagw::component::avatar::v1::AvatarItem,
    >,
}
/// 直播头像样式
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserItemStyle {
    ///
    #[prost(string, tag = "1")]
    pub rect_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub rect_text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub rect_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub rect_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub outer_animation: ::prost::alloc::string::String,
}
/// 头像挂件信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPendant {
    /// pid
    #[prost(int64, tag = "1")]
    pub pid: i64,
    /// 名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 图片链接
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    /// 有效期
    #[prost(int64, tag = "4")]
    pub expire: i64,
}
/// 角标信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoBadge {
    /// 文案
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 文案颜色-日间
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    /// 文案颜色-夜间
    #[prost(string, tag = "3")]
    pub text_color_night: ::prost::alloc::string::String,
    /// 背景颜色-日间
    #[prost(string, tag = "4")]
    pub bg_color: ::prost::alloc::string::String,
    /// 背景颜色-夜间
    #[prost(string, tag = "5")]
    pub bg_color_night: ::prost::alloc::string::String,
    /// 边框颜色-日间
    #[prost(string, tag = "6")]
    pub border_color: ::prost::alloc::string::String,
    /// 边框颜色-夜间
    #[prost(string, tag = "7")]
    pub border_color_night: ::prost::alloc::string::String,
    /// 样式
    #[prost(int32, tag = "8")]
    pub bg_style: i32,
    /// 背景透明度-日间
    #[prost(int32, tag = "9")]
    pub bg_alpha: i32,
    /// 背景透明度-夜间
    #[prost(int32, tag = "10")]
    pub bg_alpha_night: i32,
}
/// 大会员信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipInfo {
    /// 大会员类型
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 大会员状态
    #[prost(int32, tag = "2")]
    pub status: i32,
    /// 到期时间
    #[prost(int64, tag = "3")]
    pub due_date: i64,
    /// 标签
    #[prost(message, optional, tag = "4")]
    pub label: ::core::option::Option<VipLabel>,
    /// 主题
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
/// 大会员标签
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipLabel {
    /// 图片地址
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// 文本值
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    /// 对应颜色类型
    #[prost(string, tag = "3")]
    pub label_theme: ::prost::alloc::string::String,
}
/// 提权样式
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Weight {
    /// 提权展示标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 下拉框内容
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<WeightItem>,
    /// icon
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
}
/// 热门默认跳转按钮
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightButton {
    #[prost(string, tag = "1")]
    pub jump_url: ::prost::alloc::string::String,
    /// 展示文案
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
/// 提权不感兴趣
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightDislike {
    /// 负反馈业务类型 作为客户端调用负反馈接口的参数
    #[prost(string, tag = "1")]
    pub feed_back_type: ::prost::alloc::string::String,
    /// 展示文案
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
/// 提权样式
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightItem {
    /// 类型
    #[prost(enumeration = "WeightType", tag = "1")]
    pub r#type: i32,
    #[prost(oneof = "weight_item::Item", tags = "2, 3")]
    pub item: ::core::option::Option<weight_item::Item>,
}
/// Nested message and enum types in `WeightItem`.
pub mod weight_item {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        /// 热门默认跳转按钮
        #[prost(message, tag = "2")]
        Button(super::WeightButton),
        /// 提权不感兴趣
        #[prost(message, tag = "3")]
        Dislike(super::WeightDislike),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WordNode {
    ///
    #[prost(string, tag = "1")]
    pub words: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "2")]
    pub font_size: f64,
    ///
    #[prost(message, optional, tag = "3")]
    pub color: ::core::option::Option<Colors>,
    ///
    #[prost(message, optional, tag = "4")]
    pub style: ::core::option::Option<word_node::WordNodeStyle>,
}
/// Nested message and enum types in `WordNode`.
pub mod word_node {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WordNodeStyle {
        ///
        #[prost(bool, tag = "1")]
        pub bold: bool,
        ///
        #[prost(bool, tag = "2")]
        pub italic: bool,
        ///
        #[prost(bool, tag = "3")]
        pub strikethrough: bool,
        ///
        #[prost(bool, tag = "4")]
        pub underline: bool,
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AddButtonBgStyle {
    /// 默认填充
    Fill = 0,
    /// 描边
    Stroke = 1,
    /// 置灰
    Gray = 2,
}
impl AddButtonBgStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AddButtonBgStyle::Fill => "fill",
            AddButtonBgStyle::Stroke => "stroke",
            AddButtonBgStyle::Gray => "gray",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "fill" => Some(Self::Fill),
            "stroke" => Some(Self::Stroke),
            "gray" => Some(Self::Gray),
            _ => None,
        }
    }
}
/// 按钮类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AddButtonType {
    /// 占位
    BtNone = 0,
    /// 跳转
    BtJump = 1,
    /// 按钮
    BtButton = 2,
}
impl AddButtonType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AddButtonType::BtNone => "bt_none",
            AddButtonType::BtJump => "bt_jump",
            AddButtonType::BtButton => "bt_button",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "bt_none" => Some(Self::BtNone),
            "bt_jump" => Some(Self::BtJump),
            "bt_button" => Some(Self::BtButton),
            _ => None,
        }
    }
}
/// 附加卡按钮点击类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdditionalButtonClickType {
    /// 通用按钮
    ClickNone = 0,
    /// 预约卡按钮
    ClickUp = 1,
}
impl AdditionalButtonClickType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdditionalButtonClickType::ClickNone => "click_none",
            AdditionalButtonClickType::ClickUp => "click_up",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "click_none" => Some(Self::ClickNone),
            "click_up" => Some(Self::ClickUp),
            _ => None,
        }
    }
}
/// 附加卡按钮状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdditionalButtonStatus {
    ///
    None = 0,
    ///
    Uncheck = 1,
    ///
    Check = 2,
}
impl AdditionalButtonStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdditionalButtonStatus::None => "none",
            AdditionalButtonStatus::Uncheck => "uncheck",
            AdditionalButtonStatus::Check => "check",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "none" => Some(Self::None),
            "uncheck" => Some(Self::Uncheck),
            "check" => Some(Self::Check),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdditionalShareShowType {
    ///
    StNone = 0,
    ///
    StShow = 1,
}
impl AdditionalShareShowType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdditionalShareShowType::StNone => "st_none",
            AdditionalShareShowType::StShow => "st_show",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "st_none" => Some(Self::StNone),
            "st_show" => Some(Self::StShow),
            _ => None,
        }
    }
}
/// 枚举-动态附加卡
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdditionalType {
    /// 占位
    AdditionalNone = 0,
    /// 附加卡-追番
    Pgc = 1,
    /// 附加卡-商品
    Goods = 2,
    /// 附加卡投票
    Vote = 3,
    /// 附加通用卡
    Common = 4,
    /// 附加电竞卡
    Esport = 5,
    /// 附加UP主推荐卡
    UpRcmd = 6,
    /// 附加卡-ugc
    Ugc = 7,
    /// UP主预约卡
    UpReservation = 8,
}
impl AdditionalType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdditionalType::AdditionalNone => "additional_none",
            AdditionalType::Pgc => "additional_type_pgc",
            AdditionalType::Goods => "additional_type_goods",
            AdditionalType::Vote => "additional_type_vote",
            AdditionalType::Common => "additional_type_common",
            AdditionalType::Esport => "additional_type_esport",
            AdditionalType::UpRcmd => "additional_type_up_rcmd",
            AdditionalType::Ugc => "additional_type_ugc",
            AdditionalType::UpReservation => "additional_type_up_reservation",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "additional_none" => Some(Self::AdditionalNone),
            "additional_type_pgc" => Some(Self::Pgc),
            "additional_type_goods" => Some(Self::Goods),
            "additional_type_vote" => Some(Self::Vote),
            "additional_type_common" => Some(Self::Common),
            "additional_type_esport" => Some(Self::Esport),
            "additional_type_up_rcmd" => Some(Self::UpRcmd),
            "additional_type_ugc" => Some(Self::Ugc),
            "additional_type_up_reservation" => Some(Self::UpReservation),
            _ => None,
        }
    }
}
/// 投票状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdditionVoteState {
    ///
    None = 0,
    ///
    Open = 1,
    ///
    Close = 2,
}
impl AdditionVoteState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdditionVoteState::None => "addition_vote_state_none",
            AdditionVoteState::Open => "addition_vote_state_open",
            AdditionVoteState::Close => "addition_vote_state_close",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "addition_vote_state_none" => Some(Self::None),
            "addition_vote_state_open" => Some(Self::Open),
            "addition_vote_state_close" => Some(Self::Close),
            _ => None,
        }
    }
}
/// 投票类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdditionVoteType {
    ///
    None = 0,
    ///
    Word = 1,
    ///
    Pic = 2,
    ///
    Default = 3,
}
impl AdditionVoteType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdditionVoteType::None => "addition_vote_type_none",
            AdditionVoteType::Word => "addition_vote_type_word",
            AdditionVoteType::Pic => "addition_vote_type_pic",
            AdditionVoteType::Default => "addition_vote_type_default",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "addition_vote_type_none" => Some(Self::None),
            "addition_vote_type_word" => Some(Self::Word),
            "addition_vote_type_pic" => Some(Self::Pic),
            "addition_vote_type_default" => Some(Self::Default),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CampusEntryType {
    ///
    None = 0,
    ///
    EntryDynamic = 1,
    ///
    EntryHome = 2,
}
impl CampusEntryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CampusEntryType::None => "NONE",
            CampusEntryType::EntryDynamic => "ENTRY_DYNAMIC",
            CampusEntryType::EntryHome => "ENTRY_HOME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "ENTRY_DYNAMIC" => Some(Self::EntryDynamic),
            "ENTRY_HOME" => Some(Self::EntryHome),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CampusRcmdReqFrom {
    CampusRcmdFromUnknown = 0,
    CampusRcmdFromHomeUnOpen = 1,
    CampusRcmdFromVisitOther = 2,
    CampusRcmdFromHomeMoment = 3,
    CampusRcmdFromDynMoment = 4,
    CampusRcmdFromPageSubordinateMoment = 5,
}
impl CampusRcmdReqFrom {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CampusRcmdReqFrom::CampusRcmdFromUnknown => "CAMPUS_RCMD_FROM_UNKNOWN",
            CampusRcmdReqFrom::CampusRcmdFromHomeUnOpen => {
                "CAMPUS_RCMD_FROM_HOME_UN_OPEN"
            }
            CampusRcmdReqFrom::CampusRcmdFromVisitOther => "CAMPUS_RCMD_FROM_VISIT_OTHER",
            CampusRcmdReqFrom::CampusRcmdFromHomeMoment => "CAMPUS_RCMD_FROM_HOME_MOMENT",
            CampusRcmdReqFrom::CampusRcmdFromDynMoment => "CAMPUS_RCMD_FROM_DYN_MOMENT",
            CampusRcmdReqFrom::CampusRcmdFromPageSubordinateMoment => {
                "CAMPUS_RCMD_FROM_PAGE_SUBORDINATE_MOMENT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CAMPUS_RCMD_FROM_UNKNOWN" => Some(Self::CampusRcmdFromUnknown),
            "CAMPUS_RCMD_FROM_HOME_UN_OPEN" => Some(Self::CampusRcmdFromHomeUnOpen),
            "CAMPUS_RCMD_FROM_VISIT_OTHER" => Some(Self::CampusRcmdFromVisitOther),
            "CAMPUS_RCMD_FROM_HOME_MOMENT" => Some(Self::CampusRcmdFromHomeMoment),
            "CAMPUS_RCMD_FROM_DYN_MOMENT" => Some(Self::CampusRcmdFromDynMoment),
            "CAMPUS_RCMD_FROM_PAGE_SUBORDINATE_MOMENT" => {
                Some(Self::CampusRcmdFromPageSubordinateMoment)
            }
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CampusHomePageType {
    ///
    PageMajor = 0,
    ///
    PageSubordinate = 1,
    ///
    PageMajorDetail = 2,
}
impl CampusHomePageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CampusHomePageType::PageMajor => "PAGE_MAJOR",
            CampusHomePageType::PageSubordinate => "PAGE_SUBORDINATE",
            CampusHomePageType::PageMajorDetail => "PAGE_MAJOR_DETAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PAGE_MAJOR" => Some(Self::PageMajor),
            "PAGE_SUBORDINATE" => Some(Self::PageSubordinate),
            "PAGE_MAJOR_DETAIL" => Some(Self::PageMajorDetail),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CampusMngAuditStatus {
    ///
    CampusMngAuditNone = 0,
    ///
    CampusMngAuditInProcess = 1,
    ///
    CampusMngAuditFailed = 2,
}
impl CampusMngAuditStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CampusMngAuditStatus::CampusMngAuditNone => "campus_mng_audit_none",
            CampusMngAuditStatus::CampusMngAuditInProcess => {
                "campus_mng_audit_in_process"
            }
            CampusMngAuditStatus::CampusMngAuditFailed => "campus_mng_audit_failed",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "campus_mng_audit_none" => Some(Self::CampusMngAuditNone),
            "campus_mng_audit_in_process" => Some(Self::CampusMngAuditInProcess),
            "campus_mng_audit_failed" => Some(Self::CampusMngAuditFailed),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CampusMngItemType {
    ///
    CampusMngNone = 0,
    ///
    CampusMngBasicInfo = 1,
    ///
    CampusMngBadge = 2,
    ///
    CampusMngSlogan = 3,
    ///
    CampusMngQuiz = 4,
}
impl CampusMngItemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CampusMngItemType::CampusMngNone => "campus_mng_none",
            CampusMngItemType::CampusMngBasicInfo => "campus_mng_basic_info",
            CampusMngItemType::CampusMngBadge => "campus_mng_badge",
            CampusMngItemType::CampusMngSlogan => "campus_mng_slogan",
            CampusMngItemType::CampusMngQuiz => "campus_mng_quiz",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "campus_mng_none" => Some(Self::CampusMngNone),
            "campus_mng_basic_info" => Some(Self::CampusMngBasicInfo),
            "campus_mng_badge" => Some(Self::CampusMngBadge),
            "campus_mng_slogan" => Some(Self::CampusMngSlogan),
            "campus_mng_quiz" => Some(Self::CampusMngQuiz),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CampusMngQuizAction {
    ///
    CampusMngQuizActList = 0,
    ///
    CampusMngQuizActAdd = 1,
    ///
    CampusMngQuizActDel = 2,
}
impl CampusMngQuizAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CampusMngQuizAction::CampusMngQuizActList => "campus_mng_quiz_act_list",
            CampusMngQuizAction::CampusMngQuizActAdd => "campus_mng_quiz_act_add",
            CampusMngQuizAction::CampusMngQuizActDel => "campus_mng_quiz_act_del",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "campus_mng_quiz_act_list" => Some(Self::CampusMngQuizActList),
            "campus_mng_quiz_act_add" => Some(Self::CampusMngQuizActAdd),
            "campus_mng_quiz_act_del" => Some(Self::CampusMngQuizActDel),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CampusOnlineStatus {
    ///
    CampusOnlineOffline = 0,
    ///
    CampusOnlineOnline = 1,
}
impl CampusOnlineStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CampusOnlineStatus::CampusOnlineOffline => "campus_online_offline",
            CampusOnlineStatus::CampusOnlineOnline => "campus_online_online",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "campus_online_offline" => Some(Self::CampusOnlineOffline),
            "campus_online_online" => Some(Self::CampusOnlineOnline),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CampusReqFromType {
    ///
    Dynamic = 0,
    ///
    Home = 1,
}
impl CampusReqFromType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CampusReqFromType::Dynamic => "DYNAMIC",
            CampusReqFromType::Home => "HOME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DYNAMIC" => Some(Self::Dynamic),
            "HOME" => Some(Self::Home),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CampusTabType {
    ///
    CampusNone = 0,
    ///
    CampusSchool = 1,
    ///
    CampusDynamic = 2,
    ///
    CampusAccount = 3,
    ///
    CampusBillboard = 4,
    ///
    CampusTopic = 5,
    ///
    CampuesOther = 6,
}
impl CampusTabType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CampusTabType::CampusNone => "campus_none",
            CampusTabType::CampusSchool => "campus_school",
            CampusTabType::CampusDynamic => "campus_dynamic",
            CampusTabType::CampusAccount => "campus_account",
            CampusTabType::CampusBillboard => "campus_billboard",
            CampusTabType::CampusTopic => "campus_topic",
            CampusTabType::CampuesOther => "campues_other",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "campus_none" => Some(Self::CampusNone),
            "campus_school" => Some(Self::CampusSchool),
            "campus_dynamic" => Some(Self::CampusDynamic),
            "campus_account" => Some(Self::CampusAccount),
            "campus_billboard" => Some(Self::CampusBillboard),
            "campus_topic" => Some(Self::CampusTopic),
            "campues_other" => Some(Self::CampuesOther),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CoverIcon {
    /// 占位 啥都不展示
    None = 0,
    /// 播放icon
    Play = 1,
    ///
    Danmaku = 2,
    ///
    Up = 3,
    /// ? 竖屏模式 icon
    Vt = 4,
}
impl CoverIcon {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CoverIcon::None => "cover_icon_none",
            CoverIcon::Play => "cover_icon_play",
            CoverIcon::Danmaku => "cover_icon_danmaku",
            CoverIcon::Up => "cover_icon_up",
            CoverIcon::Vt => "cover_icon_vt",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "cover_icon_none" => Some(Self::None),
            "cover_icon_play" => Some(Self::Play),
            "cover_icon_danmaku" => Some(Self::Danmaku),
            "cover_icon_up" => Some(Self::Up),
            "cover_icon_vt" => Some(Self::Vt),
            _ => None,
        }
    }
}
/// 文本类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DescType {
    /// 占位
    None = 0,
    /// 文本
    Text = 1,
    /// @
    Aite = 2,
    /// 抽奖
    Lottery = 3,
    /// 投票
    Vote = 4,
    /// 话题
    Topic = 5,
    /// 商品
    Goods = 6,
    /// bv
    Bv = 7,
    /// av
    Av = 8,
    /// 表情
    Emoji = 9,
    /// 外露用户
    User = 10,
    /// 专栏
    Cv = 11,
    /// 小视频
    Vc = 12,
    /// 网址
    Web = 13,
    /// 淘宝
    Taobao = 14,
    /// 邮箱
    Mail = 15,
    /// 番剧season
    OgvSeason = 16,
    /// 番剧ep
    OgvEp = 17,
    ///
    SearchWord = 18,
}
impl DescType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DescType::None => "desc_type_none",
            DescType::Text => "desc_type_text",
            DescType::Aite => "desc_type_aite",
            DescType::Lottery => "desc_type_lottery",
            DescType::Vote => "desc_type_vote",
            DescType::Topic => "desc_type_topic",
            DescType::Goods => "desc_type_goods",
            DescType::Bv => "desc_type_bv",
            DescType::Av => "desc_type_av",
            DescType::Emoji => "desc_type_emoji",
            DescType::User => "desc_type_user",
            DescType::Cv => "desc_type_cv",
            DescType::Vc => "desc_type_vc",
            DescType::Web => "desc_type_web",
            DescType::Taobao => "desc_type_taobao",
            DescType::Mail => "desc_type_mail",
            DescType::OgvSeason => "desc_type_ogv_season",
            DescType::OgvEp => "desc_type_ogv_ep",
            DescType::SearchWord => "desc_type_search_word",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "desc_type_none" => Some(Self::None),
            "desc_type_text" => Some(Self::Text),
            "desc_type_aite" => Some(Self::Aite),
            "desc_type_lottery" => Some(Self::Lottery),
            "desc_type_vote" => Some(Self::Vote),
            "desc_type_topic" => Some(Self::Topic),
            "desc_type_goods" => Some(Self::Goods),
            "desc_type_bv" => Some(Self::Bv),
            "desc_type_av" => Some(Self::Av),
            "desc_type_emoji" => Some(Self::Emoji),
            "desc_type_user" => Some(Self::User),
            "desc_type_cv" => Some(Self::Cv),
            "desc_type_vc" => Some(Self::Vc),
            "desc_type_web" => Some(Self::Web),
            "desc_type_taobao" => Some(Self::Taobao),
            "desc_type_mail" => Some(Self::Mail),
            "desc_type_ogv_season" => Some(Self::OgvSeason),
            "desc_type_ogv_ep" => Some(Self::OgvEp),
            "desc_type_search_word" => Some(Self::SearchWord),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DisableState {
    /// 高亮
    Highlight = 0,
    /// 置灰(按钮不可点击)
    Gary = 1,
}
impl DisableState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DisableState::Highlight => "highlight",
            DisableState::Gary => "gary",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "highlight" => Some(Self::Highlight),
            "gary" => Some(Self::Gary),
            _ => None,
        }
    }
}
/// 枚举-动态类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DynamicType {
    /// 占位
    DynNone = 0,
    /// 转发
    Forward = 1,
    /// 稿件: ugc、小视频、短视频、UGC转PGC
    Av = 2,
    /// pgc：番剧、PGC番剧、PGC电影、PGC电视剧、PGC国创、PGC纪录片
    Pgc = 3,
    /// 付费更新批次
    Courses = 4,
    /// 折叠
    Fold = 5,
    /// 纯文字
    Word = 6,
    /// 图文
    Draw = 7,
    /// 专栏 原仅phone端
    Article = 8,
    /// 音频 原仅phone端
    Music = 9,
    /// 通用卡 方形
    CommonSquare = 10,
    /// 通用卡 竖形
    CommonVertical = 11,
    /// 直播卡 只有转发态
    Live = 12,
    /// 播单 原仅phone端 只有转发态
    Medialist = 13,
    /// 付费更新批次 只有转发态
    CoursesSeason = 14,
    /// 广告卡
    Ad = 15,
    /// 小程序卡
    Applet = 16,
    /// 订阅卡
    Subscription = 17,
    /// 直播推荐卡
    LiveRcmd = 18,
    /// 通栏
    Banner = 19,
    /// 合集卡
    UgcSeason = 20,
    /// 新订阅卡
    SubscriptionNew = 21,
    ///
    Story = 22,
    ///
    TopicRcmd = 23,
    ///
    CourUp = 24,
    ///
    TopicSet = 25,
    ///
    Notice = 26,
    ///
    TextNotice = 27,
}
impl DynamicType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DynamicType::DynNone => "dyn_none",
            DynamicType::Forward => "forward",
            DynamicType::Av => "av",
            DynamicType::Pgc => "pgc",
            DynamicType::Courses => "courses",
            DynamicType::Fold => "fold",
            DynamicType::Word => "word",
            DynamicType::Draw => "draw",
            DynamicType::Article => "article",
            DynamicType::Music => "music",
            DynamicType::CommonSquare => "common_square",
            DynamicType::CommonVertical => "common_vertical",
            DynamicType::Live => "live",
            DynamicType::Medialist => "medialist",
            DynamicType::CoursesSeason => "courses_season",
            DynamicType::Ad => "ad",
            DynamicType::Applet => "applet",
            DynamicType::Subscription => "subscription",
            DynamicType::LiveRcmd => "live_rcmd",
            DynamicType::Banner => "banner",
            DynamicType::UgcSeason => "ugc_season",
            DynamicType::SubscriptionNew => "subscription_new",
            DynamicType::Story => "story",
            DynamicType::TopicRcmd => "topic_rcmd",
            DynamicType::CourUp => "cour_up",
            DynamicType::TopicSet => "topic_set",
            DynamicType::Notice => "notice",
            DynamicType::TextNotice => "text_notice",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "dyn_none" => Some(Self::DynNone),
            "forward" => Some(Self::Forward),
            "av" => Some(Self::Av),
            "pgc" => Some(Self::Pgc),
            "courses" => Some(Self::Courses),
            "fold" => Some(Self::Fold),
            "word" => Some(Self::Word),
            "draw" => Some(Self::Draw),
            "article" => Some(Self::Article),
            "music" => Some(Self::Music),
            "common_square" => Some(Self::CommonSquare),
            "common_vertical" => Some(Self::CommonVertical),
            "live" => Some(Self::Live),
            "medialist" => Some(Self::Medialist),
            "courses_season" => Some(Self::CoursesSeason),
            "ad" => Some(Self::Ad),
            "applet" => Some(Self::Applet),
            "subscription" => Some(Self::Subscription),
            "live_rcmd" => Some(Self::LiveRcmd),
            "banner" => Some(Self::Banner),
            "ugc_season" => Some(Self::UgcSeason),
            "subscription_new" => Some(Self::SubscriptionNew),
            "story" => Some(Self::Story),
            "topic_rcmd" => Some(Self::TopicRcmd),
            "cour_up" => Some(Self::CourUp),
            "topic_set" => Some(Self::TopicSet),
            "notice" => Some(Self::Notice),
            "text_notice" => Some(Self::TextNotice),
            _ => None,
        }
    }
}
/// 动态小卡类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DynExtendType {
    /// 占位
    DynExtTypeNone = 0,
    /// 话题小卡
    DynExtTypeTopic = 1,
    /// lbs小卡
    DynExtTypeLbs = 2,
    /// 热门小卡
    DynExtTypeHot = 3,
    /// 游戏小卡
    DynExtTypeGame = 4,
    /// 通用小卡
    DynExtTypeCommon = 5,
    /// 必剪小卡
    DynExtTypeBiliCut = 6,
    /// ogv小卡
    DynExtTypeOgv = 7,
    /// 自动附加ogv小卡
    DynExtTypeAutoOgv = 8,
}
impl DynExtendType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DynExtendType::DynExtTypeNone => "dyn_ext_type_none",
            DynExtendType::DynExtTypeTopic => "dyn_ext_type_topic",
            DynExtendType::DynExtTypeLbs => "dyn_ext_type_lbs",
            DynExtendType::DynExtTypeHot => "dyn_ext_type_hot",
            DynExtendType::DynExtTypeGame => "dyn_ext_type_game",
            DynExtendType::DynExtTypeCommon => "dyn_ext_type_common",
            DynExtendType::DynExtTypeBiliCut => "dyn_ext_type_biliCut",
            DynExtendType::DynExtTypeOgv => "dyn_ext_type_ogv",
            DynExtendType::DynExtTypeAutoOgv => "dyn_ext_type_auto_ogv",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "dyn_ext_type_none" => Some(Self::DynExtTypeNone),
            "dyn_ext_type_topic" => Some(Self::DynExtTypeTopic),
            "dyn_ext_type_lbs" => Some(Self::DynExtTypeLbs),
            "dyn_ext_type_hot" => Some(Self::DynExtTypeHot),
            "dyn_ext_type_game" => Some(Self::DynExtTypeGame),
            "dyn_ext_type_common" => Some(Self::DynExtTypeCommon),
            "dyn_ext_type_biliCut" => Some(Self::DynExtTypeBiliCut),
            "dyn_ext_type_ogv" => Some(Self::DynExtTypeOgv),
            "dyn_ext_type_auto_ogv" => Some(Self::DynExtTypeAutoOgv),
            _ => None,
        }
    }
}
/// 动态模块类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DynModuleType {
    /// 占位
    ModuleNone = 0,
    /// 发布人模块
    ModuleAuthor = 1,
    /// 争议小黄条
    ModuleDispute = 2,
    /// 描述文案
    ModuleDesc = 3,
    /// 动态卡片
    ModuleDynamic = 4,
    /// 转发模块
    ModuleForward = 5,
    /// 点赞用户(废弃)
    ModuleLikeUser = 6,
    /// 小卡模块
    ModuleExtend = 7,
    /// 附加卡
    ModuleAdditional = 8,
    /// 计数信息
    ModuleStat = 9,
    /// 折叠
    ModuleFold = 10,
    /// 评论外露(废弃)
    ModuleComment = 11,
    /// 外露交互模块(点赞、评论)
    ModuleInteraction = 12,
    /// 转发卡的发布人模块
    ModuleAuthorForward = 13,
    /// 广告卡模块
    ModuleAd = 14,
    /// 通栏模块
    ModuleBanner = 15,
    /// 获取物料失败模块
    ModuleItemNull = 16,
    /// 分享组件
    ModuleShareInfo = 17,
    /// 相关推荐模块
    ModuleRecommend = 18,
    /// 转发卡计数信息
    ModuleStatForward = 19,
    /// 顶部模块
    ModuleTop = 20,
    /// 底部模块
    ModuleBottom = 21,
    ///
    ModuleStory = 22,
    ///
    ModuleTopic = 23,
    ///
    ModuleTopicDetailsExt = 24,
    ///
    ModuleTopTag = 25,
    ///
    ModuleTopicBrief = 26,
    ///
    ModuleTitle = 27,
    ModuleButton = 28,
    ModuleNotice = 29,
    ModuleOpusSummary = 30,
    ModuleCopyright = 31,
    ModuleParagraph = 32,
    ModuleBlocked = 33,
    ModuleTextNotice = 34,
    ModuleOpusCollection = 35,
}
impl DynModuleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DynModuleType::ModuleNone => "module_none",
            DynModuleType::ModuleAuthor => "module_author",
            DynModuleType::ModuleDispute => "module_dispute",
            DynModuleType::ModuleDesc => "module_desc",
            DynModuleType::ModuleDynamic => "module_dynamic",
            DynModuleType::ModuleForward => "module_forward",
            DynModuleType::ModuleLikeUser => "module_likeUser",
            DynModuleType::ModuleExtend => "module_extend",
            DynModuleType::ModuleAdditional => "module_additional",
            DynModuleType::ModuleStat => "module_stat",
            DynModuleType::ModuleFold => "module_fold",
            DynModuleType::ModuleComment => "module_comment",
            DynModuleType::ModuleInteraction => "module_interaction",
            DynModuleType::ModuleAuthorForward => "module_author_forward",
            DynModuleType::ModuleAd => "module_ad",
            DynModuleType::ModuleBanner => "module_banner",
            DynModuleType::ModuleItemNull => "module_item_null",
            DynModuleType::ModuleShareInfo => "module_share_info",
            DynModuleType::ModuleRecommend => "module_recommend",
            DynModuleType::ModuleStatForward => "module_stat_forward",
            DynModuleType::ModuleTop => "module_top",
            DynModuleType::ModuleBottom => "module_bottom",
            DynModuleType::ModuleStory => "module_story",
            DynModuleType::ModuleTopic => "module_topic",
            DynModuleType::ModuleTopicDetailsExt => "module_topic_details_ext",
            DynModuleType::ModuleTopTag => "module_top_tag",
            DynModuleType::ModuleTopicBrief => "module_topic_brief",
            DynModuleType::ModuleTitle => "module_title",
            DynModuleType::ModuleButton => "module_button",
            DynModuleType::ModuleNotice => "module_notice",
            DynModuleType::ModuleOpusSummary => "module_opus_summary",
            DynModuleType::ModuleCopyright => "module_copyright",
            DynModuleType::ModuleParagraph => "module_paragraph",
            DynModuleType::ModuleBlocked => "module_blocked",
            DynModuleType::ModuleTextNotice => "module_text_notice",
            DynModuleType::ModuleOpusCollection => "module_opus_collection",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "module_none" => Some(Self::ModuleNone),
            "module_author" => Some(Self::ModuleAuthor),
            "module_dispute" => Some(Self::ModuleDispute),
            "module_desc" => Some(Self::ModuleDesc),
            "module_dynamic" => Some(Self::ModuleDynamic),
            "module_forward" => Some(Self::ModuleForward),
            "module_likeUser" => Some(Self::ModuleLikeUser),
            "module_extend" => Some(Self::ModuleExtend),
            "module_additional" => Some(Self::ModuleAdditional),
            "module_stat" => Some(Self::ModuleStat),
            "module_fold" => Some(Self::ModuleFold),
            "module_comment" => Some(Self::ModuleComment),
            "module_interaction" => Some(Self::ModuleInteraction),
            "module_author_forward" => Some(Self::ModuleAuthorForward),
            "module_ad" => Some(Self::ModuleAd),
            "module_banner" => Some(Self::ModuleBanner),
            "module_item_null" => Some(Self::ModuleItemNull),
            "module_share_info" => Some(Self::ModuleShareInfo),
            "module_recommend" => Some(Self::ModuleRecommend),
            "module_stat_forward" => Some(Self::ModuleStatForward),
            "module_top" => Some(Self::ModuleTop),
            "module_bottom" => Some(Self::ModuleBottom),
            "module_story" => Some(Self::ModuleStory),
            "module_topic" => Some(Self::ModuleTopic),
            "module_topic_details_ext" => Some(Self::ModuleTopicDetailsExt),
            "module_top_tag" => Some(Self::ModuleTopTag),
            "module_topic_brief" => Some(Self::ModuleTopicBrief),
            "module_title" => Some(Self::ModuleTitle),
            "module_button" => Some(Self::ModuleButton),
            "module_notice" => Some(Self::ModuleNotice),
            "module_opus_summary" => Some(Self::ModuleOpusSummary),
            "module_copyright" => Some(Self::ModuleCopyright),
            "module_paragraph" => Some(Self::ModuleParagraph),
            "module_blocked" => Some(Self::ModuleBlocked),
            "module_text_notice" => Some(Self::ModuleTextNotice),
            "module_opus_collection" => Some(Self::ModuleOpusCollection),
            _ => None,
        }
    }
}
/// 表情包类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EmojiType {
    /// 占位
    EmojiNone = 0,
    /// emoji旧类型
    EmojiOld = 1,
    /// emoji新类型
    EmojiNew = 2,
    /// 大会员表情
    Vip = 3,
}
impl EmojiType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EmojiType::EmojiNone => "emoji_none",
            EmojiType::EmojiOld => "emoji_old",
            EmojiType::EmojiNew => "emoji_new",
            EmojiType::Vip => "vip",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "emoji_none" => Some(Self::EmojiNone),
            "emoji_old" => Some(Self::EmojiOld),
            "emoji_new" => Some(Self::EmojiNew),
            "vip" => Some(Self::Vip),
            _ => None,
        }
    }
}
/// 附加大卡-电竞卡样式
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EspaceStyle {
    /// moba类
    Moba = 0,
}
impl EspaceStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EspaceStyle::Moba => "moba",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "moba" => Some(Self::Moba),
            _ => None,
        }
    }
}
/// 折叠类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FoldType {
    /// 占位
    Zore = 0,
    /// 用户发布折叠
    Publish = 1,
    /// 转发超频折叠
    Frequent = 2,
    /// 联合投稿折叠
    Unite = 3,
    /// 动态受限折叠
    Limit = 4,
    TopicMerged = 5,
}
impl FoldType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FoldType::Zore => "FoldTypeZore",
            FoldType::Publish => "FoldTypePublish",
            FoldType::Frequent => "FoldTypeFrequent",
            FoldType::Unite => "FoldTypeUnite",
            FoldType::Limit => "FoldTypeLimit",
            FoldType::TopicMerged => "FoldTypeTopicMerged",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FoldTypeZore" => Some(Self::Zore),
            "FoldTypePublish" => Some(Self::Publish),
            "FoldTypeFrequent" => Some(Self::Frequent),
            "FoldTypeUnite" => Some(Self::Unite),
            "FoldTypeLimit" => Some(Self::Limit),
            "FoldTypeTopicMerged" => Some(Self::TopicMerged),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FollowType {
    ///
    FtNotFollow = 0,
    ///
    FtFollow = 1,
}
impl FollowType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FollowType::FtNotFollow => "ft_not_follow",
            FollowType::FtFollow => "ft_follow",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ft_not_follow" => Some(Self::FtNotFollow),
            "ft_follow" => Some(Self::FtFollow),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GoodsJumpType {
    GoodsNone = 0,
    GoodsSchema = 1,
    GoodsUrl = 2,
}
impl GoodsJumpType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GoodsJumpType::GoodsNone => "goods_none",
            GoodsJumpType::GoodsSchema => "goods_schema",
            GoodsJumpType::GoodsUrl => "goods_url",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "goods_none" => Some(Self::GoodsNone),
            "goods_schema" => Some(Self::GoodsSchema),
            "goods_url" => Some(Self::GoodsUrl),
            _ => None,
        }
    }
}
/// 文本高亮枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HighlightTextStyle {
    /// 默认
    StyleNone = 0,
    /// 高亮
    StyleHighlight = 1,
}
impl HighlightTextStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HighlightTextStyle::StyleNone => "style_none",
            HighlightTextStyle::StyleHighlight => "style_highlight",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "style_none" => Some(Self::StyleNone),
            "style_highlight" => Some(Self::StyleHighlight),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HomePageTabSttingStatus {
    SettingInvalid = 0,
    SettingOpen = 1,
    SettingClose = 2,
}
impl HomePageTabSttingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HomePageTabSttingStatus::SettingInvalid => "SETTING_INVALID",
            HomePageTabSttingStatus::SettingOpen => "SETTING_OPEN",
            HomePageTabSttingStatus::SettingClose => "SETTING_CLOSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SETTING_INVALID" => Some(Self::SettingInvalid),
            "SETTING_OPEN" => Some(Self::SettingOpen),
            "SETTING_CLOSE" => Some(Self::SettingClose),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IconResLocal {
    None = 0,
    Live = 1,
}
impl IconResLocal {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IconResLocal::None => "ICON_RES_LOCAL_NONE",
            IconResLocal::Live => "ICON_RES_LOCAL_LIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ICON_RES_LOCAL_NONE" => Some(Self::None),
            "ICON_RES_LOCAL_LIVE" => Some(Self::Live),
            _ => None,
        }
    }
}
/// 枚举-附加卡样式
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ImageStyle {
    ///
    AddStyleVertical = 0,
    ///
    AddStyleSquare = 1,
}
impl ImageStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImageStyle::AddStyleVertical => "add_style_vertical",
            ImageStyle::AddStyleSquare => "add_style_square",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "add_style_vertical" => Some(Self::AddStyleVertical),
            "add_style_square" => Some(Self::AddStyleSquare),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LightFromType {
    ///
    FromLogin = 0,
    ///
    FromUnlogin = 1,
}
impl LightFromType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LightFromType::FromLogin => "from_login",
            LightFromType::FromUnlogin => "from_unlogin",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "from_login" => Some(Self::FromLogin),
            "from_unlogin" => Some(Self::FromUnlogin),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LinkNodeType {
    Invalid = 0,
    Video = 1,
    Reserve = 2,
    Vote = 3,
    Live = 4,
    Lottery = 5,
    Match = 6,
    Goods = 7,
    OgvSs = 8,
    OgvEp = 9,
    Manga = 10,
    Cheese = 11,
    VideoTs = 12,
    At = 13,
    HashTag = 14,
    Article = 15,
    Url = 16,
    Mail = 17,
    Lbs = 18,
    Activity = 19,
    AttachCardOfficialActivity = 20,
    Game = 21,
    Decoration = 22,
    UpTopic = 23,
    UpActivity = 24,
    UpMaoer = 25,
    MemberGoods = 26,
    OpenmallUpItems = 27,
    Search = 28,
}
impl LinkNodeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LinkNodeType::Invalid => "INVALID",
            LinkNodeType::Video => "VIDEO",
            LinkNodeType::Reserve => "RESERVE",
            LinkNodeType::Vote => "VOTE",
            LinkNodeType::Live => "LIVE",
            LinkNodeType::Lottery => "LOTTERY",
            LinkNodeType::Match => "MATCH",
            LinkNodeType::Goods => "GOODS",
            LinkNodeType::OgvSs => "OGV_SS",
            LinkNodeType::OgvEp => "OGV_EP",
            LinkNodeType::Manga => "MANGA",
            LinkNodeType::Cheese => "CHEESE",
            LinkNodeType::VideoTs => "VIDEO_TS",
            LinkNodeType::At => "AT",
            LinkNodeType::HashTag => "HASH_TAG",
            LinkNodeType::Article => "ARTICLE",
            LinkNodeType::Url => "URL",
            LinkNodeType::Mail => "MAIL",
            LinkNodeType::Lbs => "LBS",
            LinkNodeType::Activity => "ACTIVITY",
            LinkNodeType::AttachCardOfficialActivity => "ATTACH_CARD_OFFICIAL_ACTIVITY",
            LinkNodeType::Game => "GAME",
            LinkNodeType::Decoration => "DECORATION",
            LinkNodeType::UpTopic => "UP_TOPIC",
            LinkNodeType::UpActivity => "UP_ACTIVITY",
            LinkNodeType::UpMaoer => "UP_MAOER",
            LinkNodeType::MemberGoods => "MEMBER_GOODS",
            LinkNodeType::OpenmallUpItems => "OPENMALL_UP_ITEMS",
            LinkNodeType::Search => "SEARCH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "VIDEO" => Some(Self::Video),
            "RESERVE" => Some(Self::Reserve),
            "VOTE" => Some(Self::Vote),
            "LIVE" => Some(Self::Live),
            "LOTTERY" => Some(Self::Lottery),
            "MATCH" => Some(Self::Match),
            "GOODS" => Some(Self::Goods),
            "OGV_SS" => Some(Self::OgvSs),
            "OGV_EP" => Some(Self::OgvEp),
            "MANGA" => Some(Self::Manga),
            "CHEESE" => Some(Self::Cheese),
            "VIDEO_TS" => Some(Self::VideoTs),
            "AT" => Some(Self::At),
            "HASH_TAG" => Some(Self::HashTag),
            "ARTICLE" => Some(Self::Article),
            "URL" => Some(Self::Url),
            "MAIL" => Some(Self::Mail),
            "LBS" => Some(Self::Lbs),
            "ACTIVITY" => Some(Self::Activity),
            "ATTACH_CARD_OFFICIAL_ACTIVITY" => Some(Self::AttachCardOfficialActivity),
            "GAME" => Some(Self::Game),
            "DECORATION" => Some(Self::Decoration),
            "UP_TOPIC" => Some(Self::UpTopic),
            "UP_ACTIVITY" => Some(Self::UpActivity),
            "UP_MAOER" => Some(Self::UpMaoer),
            "MEMBER_GOODS" => Some(Self::MemberGoods),
            "OPENMALL_UP_ITEMS" => Some(Self::OpenmallUpItems),
            "SEARCH" => Some(Self::Search),
            _ => None,
        }
    }
}
/// 直播状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LiveState {
    /// 未直播
    LiveNone = 0,
    /// 直播中
    LiveLive = 1,
    /// 轮播中
    LiveRotation = 2,
}
impl LiveState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LiveState::LiveNone => "live_none",
            LiveState::LiveLive => "live_live",
            LiveState::LiveRotation => "live_rotation",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "live_none" => Some(Self::LiveNone),
            "live_live" => Some(Self::LiveLive),
            "live_rotation" => Some(Self::LiveRotation),
            _ => None,
        }
    }
}
/// 外露模块类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocalIconType {
    ///
    LocalIconComment = 0,
    ///
    LocalIconLike = 1,
    LocalIconAvatar = 2,
    LocalIconCover = 3,
    LocalIconLikeAndForward = 4,
}
impl LocalIconType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LocalIconType::LocalIconComment => "local_icon_comment",
            LocalIconType::LocalIconLike => "local_icon_like",
            LocalIconType::LocalIconAvatar => "local_icon_avatar",
            LocalIconType::LocalIconCover => "local_icon_cover",
            LocalIconType::LocalIconLikeAndForward => "local_icon_like_and_forward",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "local_icon_comment" => Some(Self::LocalIconComment),
            "local_icon_like" => Some(Self::LocalIconLike),
            "local_icon_avatar" => Some(Self::LocalIconAvatar),
            "local_icon_cover" => Some(Self::LocalIconCover),
            "local_icon_like_and_forward" => Some(Self::LocalIconLikeAndForward),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MdlBlockedStyle {
    BlockedStyleDefault = 0,
    BlockedStyleInAudit = 1,
    BlockedStyleOnlyFansList = 2,
    BlockedStyleOnlyFansVideo = 3,
}
impl MdlBlockedStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MdlBlockedStyle::BlockedStyleDefault => "BLOCKED_STYLE_DEFAULT",
            MdlBlockedStyle::BlockedStyleInAudit => "BLOCKED_STYLE_IN_AUDIT",
            MdlBlockedStyle::BlockedStyleOnlyFansList => "BLOCKED_STYLE_ONLY_FANS_LIST",
            MdlBlockedStyle::BlockedStyleOnlyFansVideo => "BLOCKED_STYLE_ONLY_FANS_VIDEO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLOCKED_STYLE_DEFAULT" => Some(Self::BlockedStyleDefault),
            "BLOCKED_STYLE_IN_AUDIT" => Some(Self::BlockedStyleInAudit),
            "BLOCKED_STYLE_ONLY_FANS_LIST" => Some(Self::BlockedStyleOnlyFansList),
            "BLOCKED_STYLE_ONLY_FANS_VIDEO" => Some(Self::BlockedStyleOnlyFansVideo),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MdlDynCommonType {
    ///
    MdlDynCommonNone = 0,
    ///
    MdlDynCommonSquare = 1,
    ///
    MdlDynCommonVertica = 2,
}
impl MdlDynCommonType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MdlDynCommonType::MdlDynCommonNone => "mdl_dyn_common_none",
            MdlDynCommonType::MdlDynCommonSquare => "mdl_dyn_common_square",
            MdlDynCommonType::MdlDynCommonVertica => "mdl_dyn_common_vertica",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "mdl_dyn_common_none" => Some(Self::MdlDynCommonNone),
            "mdl_dyn_common_square" => Some(Self::MdlDynCommonSquare),
            "mdl_dyn_common_vertica" => Some(Self::MdlDynCommonVertica),
            _ => None,
        }
    }
}
/// 图文标签类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MdlDynDrawTagType {
    /// 占位
    MdlDrawTagNone = 0,
    /// 普通标签
    MdlDrawTagCommon = 1,
    /// 商品标签
    MdlDrawTagGoods = 2,
    /// 用户昵称
    MdlDrawTagUser = 3,
    /// 话题名称
    MdlDrawTagTopic = 4,
    /// lbs标签
    MdlDrawTagLbs = 5,
}
impl MdlDynDrawTagType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MdlDynDrawTagType::MdlDrawTagNone => "mdl_draw_tag_none",
            MdlDynDrawTagType::MdlDrawTagCommon => "mdl_draw_tag_common",
            MdlDynDrawTagType::MdlDrawTagGoods => "mdl_draw_tag_goods",
            MdlDynDrawTagType::MdlDrawTagUser => "mdl_draw_tag_user",
            MdlDynDrawTagType::MdlDrawTagTopic => "mdl_draw_tag_topic",
            MdlDynDrawTagType::MdlDrawTagLbs => "mdl_draw_tag_lbs",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "mdl_draw_tag_none" => Some(Self::MdlDrawTagNone),
            "mdl_draw_tag_common" => Some(Self::MdlDrawTagCommon),
            "mdl_draw_tag_goods" => Some(Self::MdlDrawTagGoods),
            "mdl_draw_tag_user" => Some(Self::MdlDrawTagUser),
            "mdl_draw_tag_topic" => Some(Self::MdlDrawTagTopic),
            "mdl_draw_tag_lbs" => Some(Self::MdlDrawTagLbs),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MdlDynSubscriptionNewStyle {
    /// 占位
    Nont = 0,
    /// 直播
    Live = 1,
    /// 图文
    Draw = 2,
}
impl MdlDynSubscriptionNewStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MdlDynSubscriptionNewStyle::Nont => "mdl_dyn_subscription_new_style_nont",
            MdlDynSubscriptionNewStyle::Live => "mdl_dyn_subscription_new_style_live",
            MdlDynSubscriptionNewStyle::Draw => "mdl_dyn_subscription_new_style_draw",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "mdl_dyn_subscription_new_style_nont" => Some(Self::Nont),
            "mdl_dyn_subscription_new_style_live" => Some(Self::Live),
            "mdl_dyn_subscription_new_style_draw" => Some(Self::Draw),
            _ => None,
        }
    }
}
/// 播放器类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MediaType {
    /// 本地
    None = 0,
    /// UGC
    Ugc = 1,
    /// PGC
    Pgc = 2,
    /// 直播
    Live = 3,
    /// 小视频
    Vcs = 4,
}
impl MediaType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MediaType::None => "MediaTypeNone",
            MediaType::Ugc => "MediaTypeUGC",
            MediaType::Pgc => "MediaTypePGC",
            MediaType::Live => "MediaTypeLive",
            MediaType::Vcs => "MediaTypeVCS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MediaTypeNone" => Some(Self::None),
            "MediaTypeUGC" => Some(Self::Ugc),
            "MediaTypePGC" => Some(Self::Pgc),
            "MediaTypeLive" => Some(Self::Live),
            "MediaTypeVCS" => Some(Self::Vcs),
            _ => None,
        }
    }
}
/// 右侧操作区域样式枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModuleAuthorBadgeType {
    /// 占位
    None = 0,
    /// 三点
    ThreePoint = 1,
    /// 按钮类型
    Button = 2,
    /// 提权
    Weight = 3,
}
impl ModuleAuthorBadgeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModuleAuthorBadgeType::None => "module_author_badge_type_none",
            ModuleAuthorBadgeType::ThreePoint => "module_author_badge_type_threePoint",
            ModuleAuthorBadgeType::Button => "module_author_badge_type_button",
            ModuleAuthorBadgeType::Weight => "module_author_badge_type_weight",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "module_author_badge_type_none" => Some(Self::None),
            "module_author_badge_type_threePoint" => Some(Self::ThreePoint),
            "module_author_badge_type_button" => Some(Self::Button),
            "module_author_badge_type_weight" => Some(Self::Weight),
            _ => None,
        }
    }
}
/// 动态列表-通栏类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModuleBannerType {
    ///
    None = 0,
    ///
    User = 1,
}
impl ModuleBannerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModuleBannerType::None => "module_banner_type_none",
            ModuleBannerType::User => "module_banner_type_user",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "module_banner_type_none" => Some(Self::None),
            "module_banner_type_user" => Some(Self::User),
            _ => None,
        }
    }
}
/// 动态详情模块类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModuleDynamicType {
    /// 稿件
    MdlDynArchive = 0,
    /// pgc
    MdlDynPgc = 1,
    /// 付费课程-系列
    MdlDynCourSeason = 2,
    /// 付费课程-批次
    MdlDynCourBatch = 3,
    /// 转发卡
    MdlDynForward = 4,
    /// 图文
    MdlDynDraw = 5,
    /// 专栏
    MdlDynArticle = 6,
    /// 音频
    MdlDynMusic = 7,
    /// 通用卡方
    MdlDynCommon = 8,
    /// 直播卡
    MdlDynLive = 9,
    /// 播单
    MdlDynMedialist = 10,
    /// 小程序卡
    MdlDynApplet = 11,
    /// 订阅卡
    MdlDynSubscription = 12,
    /// 直播推荐卡
    MdlDynLiveRcmd = 13,
    /// UGC合集
    MdlDynUgcSeason = 14,
    /// 订阅卡
    MdlDynSubscriptionNew = 15,
    /// 课程
    MdlDynCourBatchUp = 16,
    /// 话题集合
    MdlDynTopicSet = 17,
}
impl ModuleDynamicType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModuleDynamicType::MdlDynArchive => "mdl_dyn_archive",
            ModuleDynamicType::MdlDynPgc => "mdl_dyn_pgc",
            ModuleDynamicType::MdlDynCourSeason => "mdl_dyn_cour_season",
            ModuleDynamicType::MdlDynCourBatch => "mdl_dyn_cour_batch",
            ModuleDynamicType::MdlDynForward => "mdl_dyn_forward",
            ModuleDynamicType::MdlDynDraw => "mdl_dyn_draw",
            ModuleDynamicType::MdlDynArticle => "mdl_dyn_article",
            ModuleDynamicType::MdlDynMusic => "mdl_dyn_music",
            ModuleDynamicType::MdlDynCommon => "mdl_dyn_common",
            ModuleDynamicType::MdlDynLive => "mdl_dyn_live",
            ModuleDynamicType::MdlDynMedialist => "mdl_dyn_medialist",
            ModuleDynamicType::MdlDynApplet => "mdl_dyn_applet",
            ModuleDynamicType::MdlDynSubscription => "mdl_dyn_subscription",
            ModuleDynamicType::MdlDynLiveRcmd => "mdl_dyn_live_rcmd",
            ModuleDynamicType::MdlDynUgcSeason => "mdl_dyn_ugc_season",
            ModuleDynamicType::MdlDynSubscriptionNew => "mdl_dyn_subscription_new",
            ModuleDynamicType::MdlDynCourBatchUp => "mdl_dyn_cour_batch_up",
            ModuleDynamicType::MdlDynTopicSet => "mdl_dyn_topic_set",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "mdl_dyn_archive" => Some(Self::MdlDynArchive),
            "mdl_dyn_pgc" => Some(Self::MdlDynPgc),
            "mdl_dyn_cour_season" => Some(Self::MdlDynCourSeason),
            "mdl_dyn_cour_batch" => Some(Self::MdlDynCourBatch),
            "mdl_dyn_forward" => Some(Self::MdlDynForward),
            "mdl_dyn_draw" => Some(Self::MdlDynDraw),
            "mdl_dyn_article" => Some(Self::MdlDynArticle),
            "mdl_dyn_music" => Some(Self::MdlDynMusic),
            "mdl_dyn_common" => Some(Self::MdlDynCommon),
            "mdl_dyn_live" => Some(Self::MdlDynLive),
            "mdl_dyn_medialist" => Some(Self::MdlDynMedialist),
            "mdl_dyn_applet" => Some(Self::MdlDynApplet),
            "mdl_dyn_subscription" => Some(Self::MdlDynSubscription),
            "mdl_dyn_live_rcmd" => Some(Self::MdlDynLiveRcmd),
            "mdl_dyn_ugc_season" => Some(Self::MdlDynUgcSeason),
            "mdl_dyn_subscription_new" => Some(Self::MdlDynSubscriptionNew),
            "mdl_dyn_cour_batch_up" => Some(Self::MdlDynCourBatchUp),
            "mdl_dyn_topic_set" => Some(Self::MdlDynTopicSet),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NetworkType {
    ///
    NtUnknown = 0,
    ///
    Wifi = 1,
    ///
    Cellular = 2,
    ///
    Offline = 3,
    ///
    Othernet = 4,
    ///
    Ethernet = 5,
}
impl NetworkType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NetworkType::NtUnknown => "NT_UNKNOWN",
            NetworkType::Wifi => "WIFI",
            NetworkType::Cellular => "CELLULAR",
            NetworkType::Offline => "OFFLINE",
            NetworkType::Othernet => "OTHERNET",
            NetworkType::Ethernet => "ETHERNET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NT_UNKNOWN" => Some(Self::NtUnknown),
            "WIFI" => Some(Self::Wifi),
            "CELLULAR" => Some(Self::Cellular),
            "OFFLINE" => Some(Self::Offline),
            "OTHERNET" => Some(Self::Othernet),
            "ETHERNET" => Some(Self::Ethernet),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NftRegionType {
    NftRegionDefault = 0,
    NftRegionMainlang = 1,
    NftRegionGat = 2,
}
impl NftRegionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NftRegionType::NftRegionDefault => "nft_region_default",
            NftRegionType::NftRegionMainlang => "nft_region_mainlang",
            NftRegionType::NftRegionGat => "nft_region_gat",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "nft_region_default" => Some(Self::NftRegionDefault),
            "nft_region_mainlang" => Some(Self::NftRegionMainlang),
            "nft_region_gat" => Some(Self::NftRegionGat),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NftShowStatus {
    NftShowDefault = 0,
    NftShowZoominmainlang = 1,
    NftShowRaw = 2,
}
impl NftShowStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NftShowStatus::NftShowDefault => "nft_show_default",
            NftShowStatus::NftShowZoominmainlang => "nft_show_zoominmainlang",
            NftShowStatus::NftShowRaw => "nft_show_raw",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "nft_show_default" => Some(Self::NftShowDefault),
            "nft_show_zoominmainlang" => Some(Self::NftShowZoominmainlang),
            "nft_show_raw" => Some(Self::NftShowRaw),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RcmdReasonStyle {
    None = 0,
    CampusNearby = 1,
    CampusUp = 2,
    CampusNearUpMix = 3,
}
impl RcmdReasonStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RcmdReasonStyle::None => "rcmd_reason_style_none",
            RcmdReasonStyle::CampusNearby => "rcmd_reason_style_campus_nearby",
            RcmdReasonStyle::CampusUp => "rcmd_reason_style_campus_up",
            RcmdReasonStyle::CampusNearUpMix => "rcmd_reason_style_campus_near_up_mix",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "rcmd_reason_style_none" => Some(Self::None),
            "rcmd_reason_style_campus_nearby" => Some(Self::CampusNearby),
            "rcmd_reason_style_campus_up" => Some(Self::CampusUp),
            "rcmd_reason_style_campus_near_up_mix" => Some(Self::CampusNearUpMix),
            _ => None,
        }
    }
}
/// 推荐模块数据类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RcmdType {
    /// 稿件
    RcmdArchive = 0,
    /// 动态
    RcmdDynamic = 1,
}
impl RcmdType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RcmdType::RcmdArchive => "rcmd_archive",
            RcmdType::RcmdDynamic => "rcmd_dynamic",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "rcmd_archive" => Some(Self::RcmdArchive),
            "rcmd_dynamic" => Some(Self::RcmdDynamic),
            _ => None,
        }
    }
}
/// 刷新方式
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Refresh {
    /// 刷新列表
    New = 0,
    /// 请求历史
    History = 1,
}
impl Refresh {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Refresh::New => "refresh_new",
            Refresh::History => "refresh_history",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "refresh_new" => Some(Self::New),
            "refresh_history" => Some(Self::History),
            _ => None,
        }
    }
}
/// 关注关系 枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RelationStatus {
    /// 1-未关注 2-关注 3-被关注 4-互相关注 5-特别关注
    None = 0,
    Nofollow = 1,
    Follow = 2,
    Followed = 3,
    MutualConcern = 4,
    Special = 5,
}
impl RelationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RelationStatus::None => "relation_status_none",
            RelationStatus::Nofollow => "relation_status_nofollow",
            RelationStatus::Follow => "relation_status_follow",
            RelationStatus::Followed => "relation_status_followed",
            RelationStatus::MutualConcern => "relation_status_mutual_concern",
            RelationStatus::Special => "relation_status_special",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "relation_status_none" => Some(Self::None),
            "relation_status_nofollow" => Some(Self::Nofollow),
            "relation_status_follow" => Some(Self::Follow),
            "relation_status_followed" => Some(Self::Followed),
            "relation_status_mutual_concern" => Some(Self::MutualConcern),
            "relation_status_special" => Some(Self::Special),
            _ => None,
        }
    }
}
/// 评论类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RepostType {
    /// 热门评论
    RepostHot = 0,
    /// 普通评论
    RepostGeneral = 1,
}
impl RepostType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RepostType::RepostHot => "repost_hot",
            RepostType::RepostGeneral => "repost_general",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "repost_hot" => Some(Self::RepostHot),
            "repost_general" => Some(Self::RepostGeneral),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReserveRelationLotteryType {
    ///
    Default = 0,
    ///
    Cron = 1,
}
impl ReserveRelationLotteryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReserveRelationLotteryType::Default => {
                "reserve_relation_lottery_type_default"
            }
            ReserveRelationLotteryType::Cron => "reserve_relation_lottery_type_cron",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "reserve_relation_lottery_type_default" => Some(Self::Default),
            "reserve_relation_lottery_type_cron" => Some(Self::Cron),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReserveType {
    /// 占位
    ReserveNone = 0,
    /// 预约召回
    ReserveRecall = 1,
}
impl ReserveType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReserveType::ReserveNone => "reserve_none",
            ReserveType::ReserveRecall => "reserve_recall",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "reserve_none" => Some(Self::ReserveNone),
            "reserve_recall" => Some(Self::ReserveRecall),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouterAction {
    Open = 0,
    Embed = 1,
}
impl RouterAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RouterAction::Open => "OPEN",
            RouterAction::Embed => "EMBED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPEN" => Some(Self::Open),
            "EMBED" => Some(Self::Embed),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShowType {
    ///
    None = 0,
    ///
    Backup = 1,
}
impl ShowType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShowType::None => "show_type_none",
            ShowType::Backup => "show_type_backup",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "show_type_none" => Some(Self::None),
            "show_type_backup" => Some(Self::Backup),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StyleType {
    ///
    None = 0,
    ///
    Live = 1,
    ///
    DynUp = 2,
}
impl StyleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StyleType::None => "STYLE_TYPE_NONE",
            StyleType::Live => "STYLE_TYPE_LIVE",
            StyleType::DynUp => "STYLE_TYPE_DYN_UP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STYLE_TYPE_NONE" => Some(Self::None),
            "STYLE_TYPE_LIVE" => Some(Self::Live),
            "STYLE_TYPE_DYN_UP" => Some(Self::DynUp),
            _ => None,
        }
    }
}
/// 免流类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TfType {
    /// 未知
    TfUnknown = 0,
    /// 联通卡
    UCard = 1,
    /// 联通免流包
    UPkg = 2,
    /// 移动卡
    CCard = 3,
    /// 移动免流包
    CPkg = 4,
    /// 电信卡
    TCard = 5,
    /// 电信免流包
    TPkg = 6,
}
impl TfType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TfType::TfUnknown => "TF_UNKNOWN",
            TfType::UCard => "U_CARD",
            TfType::UPkg => "U_PKG",
            TfType::CCard => "C_CARD",
            TfType::CPkg => "C_PKG",
            TfType::TCard => "T_CARD",
            TfType::TPkg => "T_PKG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TF_UNKNOWN" => Some(Self::TfUnknown),
            "U_CARD" => Some(Self::UCard),
            "U_PKG" => Some(Self::UPkg),
            "C_CARD" => Some(Self::CCard),
            "C_PKG" => Some(Self::CPkg),
            "T_CARD" => Some(Self::TCard),
            "T_PKG" => Some(Self::TPkg),
            _ => None,
        }
    }
}
/// 枚举-三点关注状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ThreePointAttentionStatus {
    ///
    TpNotAttention = 0,
    ///
    TpAttention = 1,
}
impl ThreePointAttentionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ThreePointAttentionStatus::TpNotAttention => "tp_not_attention",
            ThreePointAttentionStatus::TpAttention => "tp_attention",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "tp_not_attention" => Some(Self::TpNotAttention),
            "tp_attention" => Some(Self::TpAttention),
            _ => None,
        }
    }
}
/// 三点类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ThreePointType {
    /// 占位
    TpNone = 0,
    /// 使用此背景
    Background = 1,
    /// 自动播放
    AutoPlay = 2,
    /// 分享
    Share = 3,
    /// 稍后再播
    Wait = 4,
    /// 关注
    Attention = 5,
    /// 举报
    Report = 6,
    /// 删除
    Delete = 7,
    /// 不感兴趣
    Dislike = 8,
    /// 收藏
    Favorite = 9,
    /// 置顶
    Top = 10,
    /// 评论
    Comment = 11,
    ///
    Hide = 12,
    ///
    CampusDelete = 13,
    ///
    TopicIrrelevant = 14,
}
impl ThreePointType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ThreePointType::TpNone => "tp_none",
            ThreePointType::Background => "background",
            ThreePointType::AutoPlay => "auto_play",
            ThreePointType::Share => "share",
            ThreePointType::Wait => "wait",
            ThreePointType::Attention => "attention",
            ThreePointType::Report => "report",
            ThreePointType::Delete => "delete",
            ThreePointType::Dislike => "dislike",
            ThreePointType::Favorite => "favorite",
            ThreePointType::Top => "top",
            ThreePointType::Comment => "comment",
            ThreePointType::Hide => "hide",
            ThreePointType::CampusDelete => "campus_delete",
            ThreePointType::TopicIrrelevant => "topic_irrelevant",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "tp_none" => Some(Self::TpNone),
            "background" => Some(Self::Background),
            "auto_play" => Some(Self::AutoPlay),
            "share" => Some(Self::Share),
            "wait" => Some(Self::Wait),
            "attention" => Some(Self::Attention),
            "report" => Some(Self::Report),
            "delete" => Some(Self::Delete),
            "dislike" => Some(Self::Dislike),
            "favorite" => Some(Self::Favorite),
            "top" => Some(Self::Top),
            "comment" => Some(Self::Comment),
            "hide" => Some(Self::Hide),
            "campus_delete" => Some(Self::CampusDelete),
            "topic_irrelevant" => Some(Self::TopicIrrelevant),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ThumbType {
    ///
    Cancel = 0,
    ///
    Thumb = 1,
}
impl ThumbType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ThumbType::Cancel => "cancel",
            ThumbType::Thumb => "thumb",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "cancel" => Some(Self::Cancel),
            "thumb" => Some(Self::Thumb),
            _ => None,
        }
    }
}
/// 状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TopType {
    /// 默认 置顶
    TopNone = 0,
    /// 取消置顶
    TopCancel = 1,
}
impl TopType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TopType::TopNone => "top_none",
            TopType::TopCancel => "top_cancel",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "top_none" => Some(Self::TopNone),
            "top_cancel" => Some(Self::TopCancel),
            _ => None,
        }
    }
}
/// 用户类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserItemType {
    ///
    None = 0,
    ///
    Live = 1,
    ///
    LiveCustom = 2,
    ///
    Normal = 3,
    ///
    Extend = 4,
    PremiereReserve = 5,
    Premiere = 6,
    LiveCard = 7,
    OgvSeason = 8,
    UgcSeason = 9,
}
impl UserItemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserItemType::None => "user_item_type_none",
            UserItemType::Live => "user_item_type_live",
            UserItemType::LiveCustom => "user_item_type_live_custom",
            UserItemType::Normal => "user_item_type_normal",
            UserItemType::Extend => "user_item_type_extend",
            UserItemType::PremiereReserve => "user_item_type_premiere_reserve",
            UserItemType::Premiere => "user_item_type_premiere",
            UserItemType::LiveCard => "user_item_type_live_card",
            UserItemType::OgvSeason => "user_item_type_ogv_season",
            UserItemType::UgcSeason => "user_item_type_ugc_season",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "user_item_type_none" => Some(Self::None),
            "user_item_type_live" => Some(Self::Live),
            "user_item_type_live_custom" => Some(Self::LiveCustom),
            "user_item_type_normal" => Some(Self::Normal),
            "user_item_type_extend" => Some(Self::Extend),
            "user_item_type_premiere_reserve" => Some(Self::PremiereReserve),
            "user_item_type_premiere" => Some(Self::Premiere),
            "user_item_type_live_card" => Some(Self::LiveCard),
            "user_item_type_ogv_season" => Some(Self::OgvSeason),
            "user_item_type_ugc_season" => Some(Self::UgcSeason),
            _ => None,
        }
    }
}
/// 番剧类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoSubType {
    /// 没有子类型
    None = 0,
    /// 番剧
    Bangumi = 1,
    /// 电影
    Movie = 2,
    /// 纪录片
    Documentary = 3,
    /// 国创
    Domestic = 4,
    /// 电视剧
    Teleplay = 5,
}
impl VideoSubType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoSubType::None => "VideoSubTypeNone",
            VideoSubType::Bangumi => "VideoSubTypeBangumi",
            VideoSubType::Movie => "VideoSubTypeMovie",
            VideoSubType::Documentary => "VideoSubTypeDocumentary",
            VideoSubType::Domestic => "VideoSubTypeDomestic",
            VideoSubType::Teleplay => "VideoSubTypeTeleplay",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VideoSubTypeNone" => Some(Self::None),
            "VideoSubTypeBangumi" => Some(Self::Bangumi),
            "VideoSubTypeMovie" => Some(Self::Movie),
            "VideoSubTypeDocumentary" => Some(Self::Documentary),
            "VideoSubTypeDomestic" => Some(Self::Domestic),
            "VideoSubTypeTeleplay" => Some(Self::Teleplay),
            _ => None,
        }
    }
}
/// 视频类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoType {
    /// 普通视频
    General = 0,
    /// 动态视频
    Dynamic = 1,
    /// 直播回放视频
    Playback = 2,
    ///
    Story = 3,
}
impl VideoType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoType::General => "video_type_general",
            VideoType::Dynamic => "video_type_dynamic",
            VideoType::Playback => "video_type_playback",
            VideoType::Story => "video_type_story",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "video_type_general" => Some(Self::General),
            "video_type_dynamic" => Some(Self::Dynamic),
            "video_type_playback" => Some(Self::Playback),
            "video_type_story" => Some(Self::Story),
            _ => None,
        }
    }
}
/// 状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoteStatus {
    /// 正常
    Normal = 0,
    /// 匿名
    Anonymous = 1,
}
impl VoteStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoteStatus::Normal => "normal",
            VoteStatus::Anonymous => "anonymous",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "normal" => Some(Self::Normal),
            "anonymous" => Some(Self::Anonymous),
            _ => None,
        }
    }
}
/// 枚举-提权类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WeightType {
    /// 默认 占位
    WeightNone = 0,
    /// 不感兴趣
    WeightDislike = 1,
    /// 跳链
    WeightJump = 2,
}
impl WeightType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WeightType::WeightNone => "weight_none",
            WeightType::WeightDislike => "weight_dislike",
            WeightType::WeightJump => "weight_jump",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "weight_none" => Some(Self::WeightNone),
            "weight_dislike" => Some(Self::WeightDislike),
            "weight_jump" => Some(Self::WeightJump),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WfItemType {
    WaterFlowTypeNone = 0,
    WaterFlowTypeArchive = 1,
    WaterFlowTypeDynamic = 2,
}
impl WfItemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WfItemType::WaterFlowTypeNone => "WATER_FLOW_TYPE_NONE",
            WfItemType::WaterFlowTypeArchive => "WATER_FLOW_TYPE_ARCHIVE",
            WfItemType::WaterFlowTypeDynamic => "WATER_FLOW_TYPE_DYNAMIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WATER_FLOW_TYPE_NONE" => Some(Self::WaterFlowTypeNone),
            "WATER_FLOW_TYPE_ARCHIVE" => Some(Self::WaterFlowTypeArchive),
            "WATER_FLOW_TYPE_DYNAMIC" => Some(Self::WaterFlowTypeDynamic),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod dynamic_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// v2动态, rpc 按字母顺序排列
    #[derive(Debug, Clone)]
    pub struct DynamicClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DynamicClient<tonic::transport::Channel> {
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
    impl<T> DynamicClient<T>
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
        ) -> DynamicClient<InterceptedService<T, F>>
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
            DynamicClient::new(InterceptedService::new(inner, interceptor))
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
        ///
        pub async fn alumni_dynamics(
            &mut self,
            request: impl tonic::IntoRequest<super::AlumniDynamicsReq>,
        ) -> std::result::Result<
            tonic::Response<super::AlumniDynamicsReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/AlumniDynamics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "AlumniDynamics"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_bill_board(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusBillBoardReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusBillBoardReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusBillBoard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusBillBoard"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_entry_tab(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusEntryTabReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusEntryTabResp>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusEntryTab",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusEntryTab"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_feedback(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusFeedbackReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusFeedbackReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusFeedback",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusFeedback"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_home_pages(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusHomePagesReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusHomePagesReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusHomePages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusHomePages"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_mate_like_list(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusMateLikeListReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusMateLikeListReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusMateLikeList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "CampusMateLikeList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_mng_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusMngDetailReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusMngDetailReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusMngDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusMngDetail"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_mng_quiz_operate(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusMngQuizOperateReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusMngQuizOperateReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusMngQuizOperate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "CampusMngQuizOperate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_mng_submit(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusMngSubmitReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusMngSubmitReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusMngSubmit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusMngSubmit"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_rcmd(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusRcmdReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusRcmdReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusRcmd",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusRcmd"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_rcmd_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusRcmdFeedReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusRcmdFeedReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusRcmdFeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusRcmdFeed"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_recommend(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusRecommendReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusRecommendReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusRecommend",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusRecommend"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_red_dot(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusRedDotReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusRedDotReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusRedDot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusRedDot"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_square(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusSquareReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusSquareReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusSquare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "CampusSquare"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn campus_topic_rcmd_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::CampusTopicRcmdFeedReq>,
        ) -> std::result::Result<
            tonic::Response<super::CampusTopicRcmdFeedReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/CampusTopicRcmdFeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "CampusTopicRcmdFeed",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 动态通用附加卡-follow/取消follow
        pub async fn dyn_addition_common_follow(
            &mut self,
            request: impl tonic::IntoRequest<super::DynAdditionCommonFollowReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynAdditionCommonFollowReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/DynAdditionCommonFollow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "DynAdditionCommonFollow",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 动态综合页
        pub async fn dyn_all(
            &mut self,
            request: impl tonic::IntoRequest<super::DynAllReq>,
        ) -> std::result::Result<tonic::Response<super::DynAllReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynAll",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynAll"));
            self.inner.unary(req, path, codec).await
        }
        /// 综合页最近访问 - 个人feed流
        pub async fn dyn_all_personal(
            &mut self,
            request: impl tonic::IntoRequest<super::DynAllPersonalReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynAllPersonalReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/DynAllPersonal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynAllPersonal"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 综合页最近访问 - 标记已读
        pub async fn dyn_all_upd_offset(
            &mut self,
            request: impl tonic::IntoRequest<super::DynAllUpdOffsetReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynAllUpdOffset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynAllUpdOffset"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 动态详情页
        pub async fn dyn_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::DynDetailReq>,
        ) -> std::result::Result<tonic::Response<super::DynDetailReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynDetail"));
            self.inner.unary(req, path, codec).await
        }
        /// 批量动态id获取动态详情
        pub async fn dyn_details(
            &mut self,
            request: impl tonic::IntoRequest<super::DynDetailsReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynDetailsReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/DynDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynDetails"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 动态发布生成临时卡
        pub async fn dyn_fake_card(
            &mut self,
            request: impl tonic::IntoRequest<super::DynFakeCardReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynFakeCardReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/DynFakeCard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynFakeCard"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn dyn_friend(
            &mut self,
            request: impl tonic::IntoRequest<super::DynFriendReq>,
        ) -> std::result::Result<tonic::Response<super::DynFriendReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynFriend",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynFriend"));
            self.inner.unary(req, path, codec).await
        }
        /// 轻浏览
        pub async fn dyn_light(
            &mut self,
            request: impl tonic::IntoRequest<super::DynLightReq>,
        ) -> std::result::Result<tonic::Response<super::DynLightReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynLight",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynLight"));
            self.inner.unary(req, path, codec).await
        }
        /// 网关调用 - 查看更多-列表
        pub async fn dyn_mix_up_list_view_more(
            &mut self,
            request: impl tonic::IntoRequest<super::DynMixUpListViewMoreReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynMixUpListViewMoreReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/DynMixUpListViewMore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "DynMixUpListViewMore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 关注推荐up主换一换
        pub async fn dyn_rcmd_up_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::DynRcmdUpExchangeReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynRcmdUpExchangeReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/DynRcmdUpExchange",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "DynRcmdUpExchange",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn dyn_search(
            &mut self,
            request: impl tonic::IntoRequest<super::DynSearchReq>,
        ) -> std::result::Result<tonic::Response<super::DynSearchReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynSearch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynSearch"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn dyn_server_details(
            &mut self,
            request: impl tonic::IntoRequest<super::DynServerDetailsReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynServerDetailsReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/DynServerDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "DynServerDetails",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 空间页动态
        pub async fn dyn_space(
            &mut self,
            request: impl tonic::IntoRequest<super::DynSpaceReq>,
        ) -> std::result::Result<tonic::Response<super::DynSpaceRsp>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynSpace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynSpace"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn dyn_space_search_details(
            &mut self,
            request: impl tonic::IntoRequest<super::DynSpaceSearchDetailsReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynSpaceSearchDetailsReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/DynSpaceSearchDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "DynSpaceSearchDetails",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn dyn_tab(
            &mut self,
            request: impl tonic::IntoRequest<super::DynTabReq>,
        ) -> std::result::Result<tonic::Response<super::DynTabReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynTab",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynTab"));
            self.inner.unary(req, path, codec).await
        }
        /// 动态点赞
        pub async fn dyn_thumb(
            &mut self,
            request: impl tonic::IntoRequest<super::DynThumbReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynThumb",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynThumb"));
            self.inner.unary(req, path, codec).await
        }
        /// 未登录页分区UP主推荐
        pub async fn dyn_un_login_rcmd(
            &mut self,
            request: impl tonic::IntoRequest<super::DynRcmdReq>,
        ) -> std::result::Result<tonic::Response<super::DynRcmdReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynUnLoginRcmd",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynUnLoginRcmd"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 动态视频页
        pub async fn dyn_video(
            &mut self,
            request: impl tonic::IntoRequest<super::DynVideoReq>,
        ) -> std::result::Result<tonic::Response<super::DynVideoReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynVideo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynVideo"));
            self.inner.unary(req, path, codec).await
        }
        /// 视频页最近访问 - 个人feed流
        pub async fn dyn_video_personal(
            &mut self,
            request: impl tonic::IntoRequest<super::DynVideoPersonalReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynVideoPersonalReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/DynVideoPersonal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "DynVideoPersonal",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 视频页最近访问 - 标记已读
        pub async fn dyn_video_upd_offset(
            &mut self,
            request: impl tonic::IntoRequest<super::DynVideoUpdOffsetReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynVideoUpdOffset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "DynVideoUpdOffset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn dyn_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::DynVoteReq>,
        ) -> std::result::Result<tonic::Response<super::DynVoteReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/DynVote",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "DynVote"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn feed_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::FeedFilterReq>,
        ) -> std::result::Result<
            tonic::Response<super::FeedFilterReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/FeedFilter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "FeedFilter"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn fetch_tab_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::NoReq>,
        ) -> std::result::Result<
            tonic::Response<super::FetchTabSettingReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/FetchTabSetting",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "FetchTabSetting"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn home_subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::HomeSubscribeReq>,
        ) -> std::result::Result<
            tonic::Response<super::HomeSubscribeReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/HomeSubscribe",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "HomeSubscribe"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn lbs_poi(
            &mut self,
            request: impl tonic::IntoRequest<super::LbsPoiReq>,
        ) -> std::result::Result<tonic::Response<super::LbsPoiReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/LbsPoi",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "LbsPoi"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn legacy_topic_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::LegacyTopicFeedReq>,
        ) -> std::result::Result<
            tonic::Response<super::LegacyTopicFeedReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/LegacyTopicFeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "LegacyTopicFeed"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 点赞列表
        pub async fn like_list(
            &mut self,
            request: impl tonic::IntoRequest<super::LikeListReq>,
        ) -> std::result::Result<tonic::Response<super::LikeListReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/LikeList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "LikeList"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn official_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::OfficialAccountsReq>,
        ) -> std::result::Result<
            tonic::Response<super::OfficialAccountsReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/OfficialAccounts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "OfficialAccounts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn official_dynamics(
            &mut self,
            request: impl tonic::IntoRequest<super::OfficialDynamicsReq>,
        ) -> std::result::Result<
            tonic::Response<super::OfficialDynamicsReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/OfficialDynamics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "OfficialDynamics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 新版动态转发点赞列表 需要登录
        pub async fn reaction_list(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactionListReq>,
        ) -> std::result::Result<
            tonic::Response<super::ReactionListReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/ReactionList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "ReactionList"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 转发列表
        pub async fn repost_list(
            &mut self,
            request: impl tonic::IntoRequest<super::RepostListReq>,
        ) -> std::result::Result<tonic::Response<super::RepostListRsp>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/RepostList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "RepostList"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn school_recommend(
            &mut self,
            request: impl tonic::IntoRequest<super::SchoolRecommendReq>,
        ) -> std::result::Result<
            tonic::Response<super::SchoolRecommendReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/SchoolRecommend",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "SchoolRecommend"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn school_search(
            &mut self,
            request: impl tonic::IntoRequest<super::SchoolSearchReq>,
        ) -> std::result::Result<
            tonic::Response<super::SchoolSearchReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/SchoolSearch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "SchoolSearch"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn set_decision(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDecisionReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/SetDecision",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "SetDecision"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn set_recent_campus(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRecentCampusReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/SetRecentCampus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "SetRecentCampus"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn subscribe_campus(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCampusReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/SubscribeCampus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "SubscribeCampus"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn topic_list(
            &mut self,
            request: impl tonic::IntoRequest<super::TopicListReq>,
        ) -> std::result::Result<tonic::Response<super::TopicListReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/TopicList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "TopicList"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn topic_square(
            &mut self,
            request: impl tonic::IntoRequest<super::TopicSquareReq>,
        ) -> std::result::Result<
            tonic::Response<super::TopicSquareReply>,
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
                "/bilibili.app.dynamic.v2.Dynamic/TopicSquare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "TopicSquare"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn unfollow_match(
            &mut self,
            request: impl tonic::IntoRequest<super::UnfollowMatchReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/UnfollowMatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Dynamic", "UnfollowMatch"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn update_tab_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTabSettingReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Dynamic/UpdateTabSetting",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v2.Dynamic",
                        "UpdateTabSetting",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpusDetailReq {
    ///
    #[prost(enumeration = "OpusType", tag = "1")]
    pub opus_type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub oid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub dyn_type: i64,
    ///
    #[prost(string, tag = "4")]
    pub share_id: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub share_mode: i32,
    ///
    #[prost(int32, tag = "10")]
    pub local_time: i32,
    ///
    #[prost(message, optional, tag = "11")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(message, optional, tag = "12")]
    pub config: ::core::option::Option<Config>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpusDetailResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub opus_item: ::core::option::Option<OpusItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpusItem {
    ///
    #[prost(int64, tag = "1")]
    pub opus_id: i64,
    ///
    #[prost(enumeration = "OpusType", tag = "2")]
    pub opus_type: i32,
    ///
    #[prost(int64, tag = "3")]
    pub oid: i64,
    ///
    #[prost(message, repeated, tag = "4")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
    ///
    #[prost(message, optional, tag = "5")]
    pub extend: ::core::option::Option<Extend>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OpusType {
    Dyn = 0,
    Article = 1,
    Note = 2,
    Word = 3,
}
impl OpusType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OpusType::Dyn => "OPUS_TYPE_DYN",
            OpusType::Article => "OPUS_TYPE_ARTICLE",
            OpusType::Note => "OPUS_TYPE_NOTE",
            OpusType::Word => "OPUS_TYPE_WORD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPUS_TYPE_DYN" => Some(Self::Dyn),
            "OPUS_TYPE_ARTICLE" => Some(Self::Article),
            "OPUS_TYPE_NOTE" => Some(Self::Note),
            "OPUS_TYPE_WORD" => Some(Self::Word),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod opus_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OpusClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OpusClient<tonic::transport::Channel> {
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
    impl<T> OpusClient<T>
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
        ) -> OpusClient<InterceptedService<T, F>>
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
            OpusClient::new(InterceptedService::new(inner, interceptor))
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
        ///
        pub async fn opus_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::OpusDetailReq>,
        ) -> std::result::Result<tonic::Response<super::OpusDetailResp>, tonic::Status> {
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
                "/bilibili.app.dynamic.v2.Opus/OpusDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v2.Opus", "OpusDetail"));
            self.inner.unary(req, path, codec).await
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampusWaterFlowItem {
    ///
    #[prost(int32, tag = "1")]
    pub item_type: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub wh_ratio: ::core::option::Option<super::common::ItemWhRatio>,
    ///
    #[prost(oneof = "campus_water_flow_item::Item", tags = "3")]
    pub item: ::core::option::Option<campus_water_flow_item::Item>,
}
/// Nested message and enum types in `CampusWaterFlowItem`.
pub mod campus_water_flow_item {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        #[prost(message, tag = "3")]
        ItemDefault(super::WfItemDefault),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaterFlowRcmdReq {
    ///
    #[prost(int64, tag = "1")]
    pub campus_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub page: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(enumeration = "CampusRcmdReqFrom", tag = "4")]
    pub from: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaterFlowRcmdResp {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<CampusWaterFlowItem>,
    ///
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<
        super::super::super::pagination::FeedPaginationReply,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WfItemDefault {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub bottom_left_1: ::core::option::Option<CoverIconWithText>,
    ///
    #[prost(message, optional, tag = "4")]
    pub bottom_left_2: ::core::option::Option<CoverIconWithText>,
    ///
    #[prost(message, optional, tag = "5")]
    pub bottom_right_1: ::core::option::Option<CoverIconWithText>,
    ///
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub rcmd_reason: ::core::option::Option<RcmdReason>,
    ///
    #[prost(map = "string, string", tag = "8")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Generated client implementations.
pub mod campus_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CampusClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CampusClient<tonic::transport::Channel> {
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
    impl<T> CampusClient<T>
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
        ) -> CampusClient<InterceptedService<T, F>>
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
            CampusClient::new(InterceptedService::new(inner, interceptor))
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
        ///
        pub async fn water_flow_rcmd(
            &mut self,
            request: impl tonic::IntoRequest<super::WaterFlowRcmdReq>,
        ) -> std::result::Result<
            tonic::Response<super::WaterFlowRcmdResp>,
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
                "/bilibili.app.dynamic.v2.Campus/WaterFlowRcmd",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v2.Campus", "WaterFlowRcmd"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}