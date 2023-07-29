/// 自动播放视频
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdAutoPlayVideoDto {
    /// avid
    #[prost(int64, tag = "1")]
    pub avid: i64,
    /// cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 分P
    #[prost(int64, tag = "3")]
    pub page: i64,
    ///
    #[prost(string, tag = "4")]
    pub from: ::prost::alloc::string::String,
    /// 是否自动播放
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
    /// 是否自动播放
    #[prost(string, tag = "6")]
    pub cover: ::prost::alloc::string::String,
    /// 是否自动播放
    #[prost(bool, tag = "7")]
    pub auto_play: bool,
    /// 按钮是否动态变色
    #[prost(bool, tag = "8")]
    pub btn_dyc_color: bool,
    /// 按钮动态变色时间 ms
    #[prost(int32, tag = "9")]
    pub btn_dyc_time: i32,
    /// 用于做联播是否是同一个视频的id
    #[prost(int64, tag = "10")]
    pub biz_id: i64,
    /// 开始播放三方监控
    #[prost(string, repeated, tag = "11")]
    pub process0_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 播放3S三方监控
    #[prost(string, repeated, tag = "12")]
    pub play_3s_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 播放5S三方监控
    #[prost(string, repeated, tag = "13")]
    pub play_5s_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 横竖屏
    #[prost(int32, tag = "14")]
    pub orientation: i32,
}
/// 商业标信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdBusinessMarkDto {
    /// 商业标样式
    /// 0:不展示标 1:实心+文字 2:空心框+文字 3:纯文字标 4:纯图片标
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 商业标文案
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    /// 商业标文案颜色,如#80FFFFFF RGBA
    #[prost(string, tag = "3")]
    pub text_color: ::prost::alloc::string::String,
    /// 夜间模式文字色
    #[prost(string, tag = "4")]
    pub text_color_night: ::prost::alloc::string::String,
    /// 背景色
    #[prost(string, tag = "5")]
    pub bg_color: ::prost::alloc::string::String,
    /// 夜间模式背景色
    #[prost(string, tag = "6")]
    pub bg_color_night: ::prost::alloc::string::String,
    /// 边框色
    #[prost(string, tag = "7")]
    pub border_color: ::prost::alloc::string::String,
    /// 夜间模式边框色
    #[prost(string, tag = "8")]
    pub border_color_night: ::prost::alloc::string::String,
    /// 图片商业标
    #[prost(string, tag = "9")]
    pub img_url: ::prost::alloc::string::String,
    /// 图片高度
    #[prost(int32, tag = "10")]
    pub img_height: i32,
    /// 图片宽度
    #[prost(int32, tag = "11")]
    pub img_width: i32,
    ///
    #[prost(string, tag = "12")]
    pub bg_border_color: ::prost::alloc::string::String,
}
/// 按钮
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdButtonDto {
    /// 类型
    /// 1:落地页 2:应用唤起 3:应用下载
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 按钮文案
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    /// 按钮跳转地址
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    /// 跳转监测链接
    #[prost(string, tag = "4")]
    pub report_urls: ::prost::alloc::string::String,
    /// 唤起schema
    #[prost(string, tag = "5")]
    pub dlsuc_callup_url: ::prost::alloc::string::String,
    /// 游戏id
    #[prost(int32, tag = "6")]
    pub game_id: i32,
    /// 游戏监控字段
    #[prost(string, tag = "7")]
    pub game_monitor_param: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub game_channel_id: i32,
    ///
    #[prost(string, tag = "9")]
    pub game_channel_extra: ::prost::alloc::string::String,
}
/// 卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdCardDto {
    /// 卡片类型
    #[prost(int32, tag = "1")]
    pub card_type: i32,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 描述
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    /// 额外描述
    #[prost(string, tag = "4")]
    pub extra_desc: ::prost::alloc::string::String,
    /// 长描述
    #[prost(string, tag = "5")]
    pub long_desc: ::prost::alloc::string::String,
    /// 短标题, 弹幕广告目录面板标题
    #[prost(string, tag = "6")]
    pub short_title: ::prost::alloc::string::String,
    /// 弹幕/浮层广告的弹幕标题
    #[prost(string, tag = "7")]
    pub danmu_title: ::prost::alloc::string::String,
    /// 弹幕/浮层广告的弹幕高度，整型，分母为100
    #[prost(int32, tag = "8")]
    pub danmu_height: i32,
    /// 弹幕/浮层广告的弹幕宽度，整型，分母为100
    #[prost(int32, tag = "9")]
    pub danmu_width: i32,
    /// 弹幕/浮层广告生存时间，单位为毫秒
    #[prost(int32, tag = "10")]
    pub danmu_life: i32,
    /// 弹幕/浮层开始时间，单位为毫秒
    #[prost(int32, tag = "11")]
    pub danmu_begin: i32,
    /// 背景色值（含透明度）如#80FFFFFF
    #[prost(string, tag = "12")]
    pub danmu_color: ::prost::alloc::string::String,
    /// 弹幕/浮层广告H5落地页
    #[prost(string, tag = "13")]
    pub danmu_h5url: ::prost::alloc::string::String,
    /// 弹幕/浮层 广告icon
    #[prost(string, tag = "14")]
    pub danmu_icon: ::prost::alloc::string::String,
    /// 折叠时间，永驻浮层折叠时间，单位为毫秒
    #[prost(int32, tag = "15")]
    pub fold_time: i32,
    /// 广告标文案
    #[prost(string, tag = "16")]
    pub ad_tag: ::prost::alloc::string::String,
    /// cover数组
    #[prost(message, repeated, tag = "17")]
    pub covers: ::prost::alloc::vec::Vec<AdCoverDto>,
    /// 卡片跳转链接
    #[prost(string, tag = "18")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "19")]
    pub imax_landing_page_json_string: ::prost::alloc::string::String,
    /// app唤起schema
    #[prost(string, tag = "20")]
    pub callup_url: ::prost::alloc::string::String,
    /// univeral link域名
    #[prost(string, tag = "21")]
    pub universal_app: ::prost::alloc::string::String,
    /// 原价, 单位为分
    #[prost(string, tag = "22")]
    pub ori_price: ::prost::alloc::string::String,
    /// 现价, 同上
    #[prost(int32, tag = "23")]
    pub cur_price: i32,
    /// 券后/现价 价格描述
    #[prost(string, tag = "24")]
    pub price_desc: ::prost::alloc::string::String,
    /// 价格单位符号
    #[prost(string, tag = "25")]
    pub price_symbol: ::prost::alloc::string::String,
    /// 券后价格 "1000"
    #[prost(string, tag = "26")]
    pub goods_cur_price: ::prost::alloc::string::String,
    /// 原价 "¥1002"
    #[prost(string, tag = "27")]
    pub goods_ori_price: ::prost::alloc::string::String,
    /// 开放平台商品
    #[prost(message, optional, tag = "28")]
    pub good: ::core::option::Option<AdGoodDto>,
    /// 打分? 满分为100
    #[prost(int32, tag = "29")]
    pub rank: i32,
    /// 热度
    #[prost(int32, tag = "30")]
    pub hot_score: i32,
    /// 按钮
    #[prost(message, optional, tag = "31")]
    pub button: ::core::option::Option<AdButtonDto>,
    /// 广告主logo
    #[prost(string, tag = "32")]
    pub adver_logo: ::prost::alloc::string::String,
    /// 广告主name
    #[prost(string, tag = "33")]
    pub adver_name: ::prost::alloc::string::String,
    /// 广告主主页链接
    #[prost(string, tag = "34")]
    pub adver_page_url: ::prost::alloc::string::String,
    /// 视频弹幕，视频广告用
    #[prost(string, repeated, tag = "35")]
    pub video_barrage: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 商业标信息
    #[prost(message, optional, tag = "36")]
    pub ad_tag_style: ::core::option::Option<AdBusinessMarkDto>,
    /// 自动播放视频
    #[prost(message, optional, tag = "37")]
    pub video: ::core::option::Option<AdAutoPlayVideoDto>,
    /// 反馈面板功能模块，屏蔽、投诉、广告介绍
    #[prost(message, optional, tag = "38")]
    pub feedback_panel: ::core::option::Option<AdFeedbackPanelDto>,
    ///
    #[prost(int64, tag = "39")]
    pub adver_mid: i64,
    ///
    #[prost(int64, tag = "40")]
    pub adver_account_id: i64,
    ///
    #[prost(string, tag = "41")]
    pub duration: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "42")]
    pub quality_infos: ::prost::alloc::vec::Vec<QualityInfo>,
    /// 动态广告文本
    #[prost(string, tag = "43")]
    pub dynamic_text: ::prost::alloc::string::String,
    /// 广告主信息
    #[prost(message, optional, tag = "44")]
    pub adver: ::core::option::Option<AdverDto>,
    /// 评分
    #[prost(int32, tag = "45")]
    pub grade_level: i32,
    ///
    #[prost(bool, tag = "46")]
    pub support_transition: bool,
    ///
    #[prost(string, tag = "47")]
    pub transition: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "48")]
    pub under_player_interaction_style: i32,
    ///
    #[prost(string, tag = "49")]
    pub imax_landing_page_v2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "50")]
    pub subcard_module: ::core::option::Option<SubCardModule>,
    ///
    #[prost(int32, tag = "51")]
    pub grade_denominator: i32,
    ///
    #[prost(int32, tag = "52")]
    pub star_level: i32,
    ///
    #[prost(message, optional, tag = "53")]
    pub bulletin: ::core::option::Option<Bulletin>,
    ///
    #[prost(message, optional, tag = "54")]
    pub gift: ::core::option::Option<Gift>,
    ///
    #[prost(string, repeated, tag = "55")]
    pub game_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "56")]
    pub ori_mark_hidden: i32,
    ///
    #[prost(bool, tag = "57")]
    pub use_multi_cover: bool,
    ///
    #[prost(message, optional, tag = "58")]
    pub wx_program_info: ::core::option::Option<WxProgramInfo>,
    ///
    #[prost(message, optional, tag = "59")]
    pub android_game_page_res: ::core::option::Option<AndroidGamePageRes>,
    ///
    #[prost(message, optional, tag = "60")]
    pub not_clickable_area: ::core::option::Option<NotClickableArea>,
    ///
    #[prost(message, optional, tag = "61")]
    pub forward_reply: ::core::option::Option<ForwardReply>,
}
/// 额外广告数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdContentExtraDto {
    /// 动态布局
    #[prost(string, tag = "1")]
    pub layout: ::prost::alloc::string::String,
    /// 展现监控url
    #[prost(string, repeated, tag = "2")]
    pub show_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 点击监控url
    #[prost(string, repeated, tag = "3")]
    pub click_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 弹幕创意列表展示第三方上报
    #[prost(string, repeated, tag = "4")]
    pub danmu_list_show_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 弹幕创意列表点击第三方上报
    #[prost(string, repeated, tag = "5")]
    pub danmu_list_click_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 弹幕详情页展示第三方上报
    #[prost(string, repeated, tag = "6")]
    pub danmu_detail_show_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 弹幕商品添加购物车第三方上报
    #[prost(string, repeated, tag = "7")]
    pub danmu_trolley_add_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// useWebView默认false
    #[prost(bool, tag = "8")]
    pub use_ad_web_v2: bool,
    /// app唤起白名单
    #[prost(string, repeated, tag = "9")]
    pub open_whitelist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// app下载白名单
    #[prost(message, optional, tag = "10")]
    pub download_whitelist: ::core::option::Option<AppPackageDto>,
    /// 卡片相关信息
    #[prost(message, optional, tag = "11")]
    pub card: ::core::option::Option<AdCardDto>,
    /// 视频播放和弹幕播放上报控制时间 ms
    #[prost(int32, tag = "12")]
    pub report_time: i32,
    /// 是否优先唤起app store
    #[prost(int32, tag = "13")]
    pub appstore_priority: i32,
    /// 广告售卖类型
    #[prost(int32, tag = "14")]
    pub sales_type: i32,
    /// 落地页是否预加载
    #[prost(int32, tag = "15")]
    pub preload_landingpage: i32,
    /// 是否需要展示风险行业提示
    #[prost(bool, tag = "16")]
    pub special_industry: bool,
    /// 风险行业提示
    #[prost(string, tag = "17")]
    pub special_industry_tips: ::prost::alloc::string::String,
    /// 是否展示下载弹框
    #[prost(bool, tag = "18")]
    pub enable_download_dialog: bool,
    /// 是否允许分享
    #[prost(bool, tag = "19")]
    pub enable_share: bool,
    /// 个人空间广告入口类型
    /// 1:橱窗 2:商品店铺 3:小程序
    #[prost(int32, tag = "20")]
    pub upzone_entrance_type: i32,
    /// 个人空间广告入口上报id,橱窗id(当前用Mid)、店铺id或者小程序id
    #[prost(int32, tag = "21")]
    pub upzone_entrance_report_id: i32,
    /// 分享数据
    #[prost(message, optional, tag = "22")]
    pub share_info: ::core::option::Option<AdShareInfoDto>,
    /// topview图片链接，闪屏预下载用
    #[prost(string, tag = "23")]
    pub topview_pic_url: ::prost::alloc::string::String,
    /// topview视频链接，闪屏预下载用
    #[prost(string, tag = "24")]
    pub topview_video_url: ::prost::alloc::string::String,
    /// 点击区域
    /// 0:表示banner可点击 1:表示素材可点击
    #[prost(int32, tag = "25")]
    pub click_area: i32,
    /// 店铺
    #[prost(int64, tag = "26")]
    pub shop_id: i64,
    /// up主
    #[prost(int64, tag = "27")]
    pub up_mid: i64,
    /// 回传id
    #[prost(string, tag = "28")]
    pub track_id: ::prost::alloc::string::String,
    /// 商店直投
    #[prost(int32, tag = "29")]
    pub enable_store_direct_launch: i32,
    /// DPA2.0商品ID
    #[prost(int64, tag = "30")]
    pub product_id: i64,
    ///
    #[prost(bool, tag = "31")]
    pub enable_double_jump: bool,
    ///
    #[prost(string, repeated, tag = "32")]
    pub show1s_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "33")]
    pub from_track_id: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "34")]
    pub store_callup_card: bool,
    ///
    #[prost(int32, tag = "35")]
    pub landingpage_download_style: i32,
    ///
    #[prost(int32, tag = "36")]
    pub special_industry_style: i32,
    ///
    #[prost(bool, tag = "37")]
    pub enable_h5_alert: bool,
    ///
    #[prost(int32, tag = "38")]
    pub macro_replace_priority: i32,
    ///
    #[prost(int32, tag = "39")]
    pub feedback_panel_style: i32,
    ///
    #[prost(string, tag = "40")]
    pub appstore_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "41")]
    pub enable_h5_pre_load: i32,
    ///
    #[prost(string, tag = "42")]
    pub h5_pre_load_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "43")]
    pub cm_from_track_id: ::prost::alloc::string::String,
}
/// 广告卡片封面数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdCoverDto {
    /// 图片链接
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// 动图循环次数
    /// 0:无限循环
    #[prost(int32, tag = "2")]
    pub r#loop: i32,
    /// 图片点击跳转地址，截至目前为空
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    /// 跳转监测链接， 数组，单个图片的监控，出区别于click_urls，应前端要求。（此字段截至目前为空，使用时需再次确认）
    #[prost(string, repeated, tag = "4")]
    pub report_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 图片高度
    #[prost(int32, tag = "5")]
    pub image_height: i32,
    /// 图片宽度
    #[prost(int32, tag = "6")]
    pub image_width: i32,
}
/// 广告内容
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdDto {
    /// 广告创意ID
    #[prost(int64, tag = "1")]
    pub creative_id: i64,
    /// 广告闭环上报回传数据
    #[prost(string, tag = "2")]
    pub ad_cb: ::prost::alloc::string::String,
    /// 额外广告数据
    #[prost(message, optional, tag = "3")]
    pub extra: ::core::option::Option<AdContentExtraDto>,
    /// 广告标记
    #[prost(int32, tag = "4")]
    pub cm_mark: i32,
    ///
    #[prost(int64, tag = "5")]
    pub top_view_id: i64,
    ///
    #[prost(int32, tag = "6")]
    pub creative_type: i32,
    ///
    #[prost(int32, tag = "7")]
    pub card_type: i32,
    ///
    #[prost(int32, tag = "8")]
    pub creative_style: i32,
    ///
    #[prost(int32, tag = "9")]
    pub is_ad: i32,
    ///
    #[prost(message, optional, tag = "10")]
    pub creative_content: ::core::option::Option<CreativeDto>,
}
/// 反馈面板功能模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdFeedbackPanelDto {
    /// 面板类型，广告、推广
    #[prost(string, tag = "1")]
    pub panel_type_text: ::prost::alloc::string::String,
    /// 反馈面版信息
    #[prost(message, repeated, tag = "2")]
    pub feedback_panel_detail: ::prost::alloc::vec::Vec<AdFeedbackPanelModuleDto>,
    ///
    #[prost(string, tag = "3")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub open_rec_tips: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub close_rec_tips: ::prost::alloc::string::String,
}
/// 反馈面版信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdFeedbackPanelModuleDto {
    /// 模块id
    #[prost(int32, tag = "1")]
    pub module_id: i32,
    /// icon url
    #[prost(string, tag = "2")]
    pub icon_url: ::prost::alloc::string::String,
    /// 跳转类型
    /// 1:气泡 2:H5
    #[prost(int32, tag = "3")]
    pub jump_type: i32,
    /// 跳转地址
    #[prost(string, tag = "4")]
    pub jump_url: ::prost::alloc::string::String,
    /// 文案
    #[prost(string, tag = "5")]
    pub text: ::prost::alloc::string::String,
    /// 二级文案数组
    #[prost(message, repeated, tag = "6")]
    pub secondary_panel: ::prost::alloc::vec::Vec<AdSecondFeedbackPanelDto>,
    ///
    #[prost(string, tag = "7")]
    pub sub_text: ::prost::alloc::string::String,
}
/// 开放平台商品
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGoodDto {
    /// 电商商品ID
    #[prost(int64, tag = "1")]
    pub item_id: i64,
    /// 电商SKU ID
    #[prost(int64, tag = "2")]
    pub sku_id: i64,
    /// 店铺ID
    #[prost(int64, tag = "3")]
    pub shop_id: i64,
    /// SKU库存
    #[prost(int64, tag = "4")]
    pub sku_num: i64,
}
/// 有弹幕的ogv ep
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdOgvEpDto {
    /// 分集epid
    #[prost(int64, tag = "1")]
    pub epid: i64,
    /// 是否显示 "荐"
    #[prost(bool, tag = "2")]
    pub has_recommend: bool,
}
/// 广告控制
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdsControlDto {
    /// 视频是否有弹幕，如有，需请求弹幕广告
    #[prost(int32, tag = "1")]
    pub has_danmu: i32,
    /// 有弹幕的分P视频的cid，已弃用
    #[prost(int64, repeated, tag = "2")]
    pub cids: ::prost::alloc::vec::Vec<i64>,
    /// 有弹幕的ogv ep
    #[prost(message, repeated, tag = "3")]
    pub eps: ::prost::alloc::vec::Vec<AdOgvEpDto>,
}
/// 二级文案
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdSecondFeedbackPanelDto {
    /// 屏蔽理由id
    #[prost(int32, tag = "1")]
    pub reason_id: i32,
    /// 理由文案
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// 分享
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdShareInfoDto {
    /// 分享标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 分享副标题
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    /// 分享图片url
    #[prost(string, tag = "3")]
    pub image_url: ::prost::alloc::string::String,
}
/// 广告主信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdverDto {
    ///
    #[prost(int64, tag = "1")]
    pub adver_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub adver_logo: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub adver_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub adver_type: i32,
    ///
    #[prost(string, tag = "5")]
    pub adver_page_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub adver_desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidGamePageRes {
    ///
    #[prost(message, optional, tag = "1")]
    pub module1: ::core::option::Option<Module1>,
    ///
    #[prost(message, optional, tag = "2")]
    pub module3: ::core::option::Option<Module3>,
    ///
    #[prost(message, optional, tag = "3")]
    pub module4: ::core::option::Option<Module4>,
    ///
    #[prost(message, optional, tag = "4")]
    pub module5: ::core::option::Option<Module5>,
    ///
    #[prost(message, optional, tag = "5")]
    pub module6: ::core::option::Option<Module6>,
    ///
    #[prost(message, optional, tag = "6")]
    pub module7: ::core::option::Option<Module7>,
    ///
    #[prost(message, optional, tag = "7")]
    pub module8: ::core::option::Option<Module8>,
    ///
    #[prost(message, optional, tag = "8")]
    pub module9: ::core::option::Option<Module9>,
    ///
    #[prost(message, optional, tag = "9")]
    pub module10: ::core::option::Option<Module10>,
    ///
    #[prost(message, optional, tag = "10")]
    pub module11: ::core::option::Option<Module11>,
    ///
    #[prost(message, optional, tag = "11")]
    pub module12: ::core::option::Option<Module12>,
    ///
    #[prost(message, optional, tag = "12")]
    pub module13: ::core::option::Option<Module13>,
    ///
    #[prost(int32, repeated, tag = "13")]
    pub module_seq: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(string, tag = "14")]
    pub background_color: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "15")]
    pub module14: ::core::option::Option<Module14>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidTag {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub r#type: i32,
}
/// app下载白名单
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPackageDto {
    /// 包大小(单位bytes)
    #[prost(int64, tag = "1")]
    pub size: i64,
    ///
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub apk_name: ::prost::alloc::string::String,
    /// url
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    /// bili schema url
    #[prost(string, tag = "5")]
    pub bili_url: ::prost::alloc::string::String,
    /// 包md5
    #[prost(string, tag = "6")]
    pub md5: ::prost::alloc::string::String,
    /// 包icon
    #[prost(string, tag = "7")]
    pub icon: ::prost::alloc::string::String,
    /// 开发者姓名
    #[prost(string, tag = "8")]
    pub dev_name: ::prost::alloc::string::String,
    /// 权限地址
    #[prost(string, tag = "9")]
    pub auth_url: ::prost::alloc::string::String,
    /// 权限名，逗号隔开
    #[prost(string, tag = "10")]
    pub auth_name: ::prost::alloc::string::String,
    /// 版本
    #[prost(string, tag = "11")]
    pub version: ::prost::alloc::string::String,
    /// 更新时间,yy-mm-hh格式
    #[prost(string, tag = "12")]
    pub update_time: ::prost::alloc::string::String,
    /// 隐私协议标题
    #[prost(string, tag = "13")]
    pub privacy_name: ::prost::alloc::string::String,
    /// 隐私协议url
    #[prost(string, tag = "14")]
    pub privacy_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bulletin {
    ///
    #[prost(string, tag = "1")]
    pub tag_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    ///
    #[prost(int64, tag = "1")]
    pub game_base_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub user_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub user_face: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub user_level: i32,
    ///
    #[prost(string, tag = "5")]
    pub comment_no: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub grade: i32,
    ///
    #[prost(string, tag = "7")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub up_count: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreativeDto {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub image_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub image_md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub click_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub show_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub video_id: i64,
    ///
    #[prost(string, tag = "9")]
    pub thumbnail_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub thumbnail_url_md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub logo_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub logo_md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub username: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomPlayUrl {
    ///
    #[prost(int32, tag = "1")]
    pub play_time: i32,
    ///
    #[prost(string, repeated, tag = "2")]
    pub urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardReply {
    ///
    #[prost(int64, tag = "1")]
    pub comment_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub highlight_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub highlight_prefix_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub callup_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub jump_type: i32,
    ///
    #[prost(string, tag = "8")]
    pub author_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub author_icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gift {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub night_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosGamePageRes {
    ///
    #[prost(string, tag = "1")]
    pub logo: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sub_titile: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "4")]
    pub image_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "5")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub game_button: ::core::option::Option<AdButtonDto>,
    ///
    #[prost(double, tag = "7")]
    pub grade: f64,
    ///
    #[prost(string, tag = "8")]
    pub rank_num: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub rank_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module1 {
    ///
    #[prost(string, tag = "1")]
    pub game_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub game_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub developer_input_name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub tag_list: ::prost::alloc::vec::Vec<AndroidTag>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module3 {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(message, repeated, tag = "3")]
    pub quality_params: ::prost::alloc::vec::Vec<QualityParmas>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module4 {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(int32, tag = "2")]
    pub gift_num: i32,
    ///
    #[prost(string, tag = "3")]
    pub gift_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub gift_icon_num: i32,
    ///
    #[prost(string, repeated, tag = "5")]
    pub icon_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module5 {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(string, tag = "2")]
    pub game_summary: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module6 {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(string, tag = "2")]
    pub game_desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module7 {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(message, repeated, tag = "2")]
    pub screen_shots: ::prost::alloc::vec::Vec<ScreenShots>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module8 {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(string, repeated, tag = "2")]
    pub tag_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module9 {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(string, tag = "2")]
    pub dev_introduction: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module10 {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(string, tag = "2")]
    pub latest_update: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module11 {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(int32, repeated, tag = "2")]
    pub star_number_list: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(string, tag = "3")]
    pub comment_str: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "4")]
    pub grade: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module12 {
    ///
    #[prost(bool, tag = "1")]
    pub display: bool,
    ///
    #[prost(message, repeated, tag = "2")]
    pub comment_list: ::prost::alloc::vec::Vec<Comment>,
    ///
    #[prost(string, tag = "3")]
    pub comment_num: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "4")]
    pub show_all_comment: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module13 {
    ///
    #[prost(int64, tag = "1")]
    pub pkg_size: i64,
    ///
    #[prost(string, tag = "2")]
    pub customer_service: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub website: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub authority: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub privacy: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub developer_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub update_time: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub game_version: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub android_pkg_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module14 {
    ///
    #[prost(message, repeated, tag = "1")]
    pub reward_list: ::prost::alloc::vec::Vec<Reward>,
    ///
    #[prost(bool, tag = "2")]
    pub display: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotClickableArea {
    ///
    #[prost(int32, tag = "1")]
    pub x: i32,
    ///
    #[prost(int32, tag = "2")]
    pub y: i32,
    ///
    #[prost(int32, tag = "3")]
    pub z: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualityInfo {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "3")]
    pub is_bg: bool,
    ///
    #[prost(string, tag = "4")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub bg_color_night: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualityParmas {
    ///
    #[prost(string, tag = "1")]
    pub first_line: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub second_line: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "3")]
    pub grade: f64,
    ///
    #[prost(string, tag = "4")]
    pub rank_icon: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub quality_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reward {
    ///
    #[prost(int32, tag = "1")]
    pub level: i32,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub pic: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "5")]
    pub reach: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScreenShots {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub height: i32,
    ///
    #[prost(int32, tag = "3")]
    pub width: i32,
    ///
    #[prost(int32, tag = "4")]
    pub seq: i32,
}
/// 广告数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceContentDto {
    /// 广告请求id
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// 广告资源位source ID
    #[prost(int32, tag = "2")]
    pub source_id: i32,
    /// 广告资源位resource ID
    #[prost(int32, tag = "3")]
    pub resource_id: i32,
    /// 广告位上报标记,对广告返回数据恒为true
    #[prost(bool, tag = "4")]
    pub is_ad_loc: bool,
    /// 与天马现有逻辑一致, 0有含义
    /// 0:内容 1:广告
    #[prost(message, optional, tag = "5")]
    pub server_type: ::core::option::Option<
        super::super::super::google::protobuf::Int32Value,
    >,
    /// 客户端IP回传拼接
    #[prost(string, tag = "6")]
    pub client_ip: ::prost::alloc::string::String,
    /// 广告卡片位置在一刷中的位置, 天马用, 0有含义
    #[prost(message, optional, tag = "7")]
    pub card_index: ::core::option::Option<
        super::super::super::google::protobuf::Int32Value,
    >,
    /// 广告资源位source 位次
    #[prost(int32, tag = "8")]
    pub index: i32,
    /// 广告内容
    #[prost(message, optional, tag = "9")]
    pub ad_content: ::core::option::Option<AdDto>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubCardModule {
    ///
    #[prost(string, tag = "1")]
    pub subcard_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub rank_stars: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub amount_number: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub avatar: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub button: ::core::option::Option<AdButtonDto>,
    ///
    #[prost(message, repeated, tag = "9")]
    pub tag_infos: ::prost::alloc::vec::Vec<TagInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tab2ExtraDto {
    ///
    #[prost(string, tag = "1")]
    pub cover_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub button: ::core::option::Option<AdButtonDto>,
    ///
    #[prost(int32, tag = "6")]
    pub auto_animate_time_ms: i32,
    ///
    #[prost(bool, tag = "7")]
    pub enable_click: bool,
    ///
    #[prost(string, tag = "8")]
    pub panel_url: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "9")]
    pub download_whitelist: ::prost::alloc::vec::Vec<AppPackageDto>,
    ///
    #[prost(string, repeated, tag = "10")]
    pub open_whitelist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(bool, tag = "11")]
    pub use_ad_web_v2: bool,
    ///
    #[prost(bool, tag = "12")]
    pub enable_store_direct_launch: bool,
    ///
    #[prost(int32, tag = "13")]
    pub sales_type: i32,
    ///
    #[prost(int32, tag = "15")]
    pub landingpage_download_style: i32,
    ///
    #[prost(int32, tag = "16")]
    pub appstore_priority: i32,
    ///
    #[prost(string, tag = "17")]
    pub appstore_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "18")]
    pub appstore_delay_time: i32,
    ///
    #[prost(int32, tag = "19")]
    pub page_cover_type: i32,
    ///
    #[prost(int32, tag = "20")]
    pub page_pull_type: i32,
    ///
    #[prost(message, optional, tag = "21")]
    pub android_game_page_res: ::core::option::Option<AndroidGamePageRes>,
    ///
    #[prost(message, optional, tag = "22")]
    pub ios_game_page_res: ::core::option::Option<IosGamePageRes>,
    ///
    #[prost(message, optional, tag = "23")]
    pub ad_tag_style: ::core::option::Option<AdBusinessMarkDto>,
    ///
    #[prost(message, optional, tag = "24")]
    pub feedback_panel: ::core::option::Option<AdFeedbackPanelDto>,
    ///
    #[prost(string, tag = "25")]
    pub ad_cb: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "26")]
    pub url_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabExtraDto {
    ///
    #[prost(string, tag = "1")]
    pub tab_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub enable_store_direct_launch: i32,
    ///
    #[prost(int32, tag = "3")]
    pub store_callup_card: i32,
    ///
    #[prost(int32, tag = "4")]
    pub sales_type: i32,
    ///
    #[prost(message, repeated, tag = "5")]
    pub download_whitelist: ::prost::alloc::vec::Vec<AppPackageDto>,
    ///
    #[prost(bool, tag = "6")]
    pub special_industry: bool,
    ///
    #[prost(string, tag = "7")]
    pub special_industry_tips: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "8")]
    pub open_whitelist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "9")]
    pub landingpage_download_style: i32,
    ///
    #[prost(int32, tag = "10")]
    pub appstore_priority: i32,
    ///
    #[prost(bool, tag = "11")]
    pub use_ad_web_v2: bool,
    ///
    #[prost(bool, tag = "12")]
    pub enable_download_dialog: bool,
    ///
    #[prost(string, tag = "13")]
    pub appstore_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "14")]
    pub appstore_delay_time: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabInfoDto {
    ///
    #[prost(string, tag = "1")]
    pub tab_name: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub extra: ::core::option::Option<super::super::super::google::protobuf::Any>,
    ///
    #[prost(int32, tag = "3")]
    pub tab_version: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagInfo {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub bg_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub border_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub border_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub r#type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WxProgramInfo {
    ///
    #[prost(string, tag = "1")]
    pub org_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
}
