/// 地址部件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressComponent {
    /// 国家
    #[prost(string, tag = "1")]
    pub nation: ::prost::alloc::string::String,
    /// 省
    #[prost(string, tag = "2")]
    pub province: ::prost::alloc::string::String,
    /// 市
    #[prost(string, tag = "3")]
    pub city: ::prost::alloc::string::String,
    /// 区，可能为空字串
    #[prost(string, tag = "4")]
    pub district: ::prost::alloc::string::String,
    /// 街道，可能为空字串
    #[prost(string, tag = "5")]
    pub street: ::prost::alloc::string::String,
    /// 门牌，可能为空字串
    #[prost(string, tag = "6")]
    pub street_number: ::prost::alloc::string::String,
}
/// 行政区划信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdInfo {
    /// 国家代码(ISO3166标准3位数字码)
    #[prost(string, tag = "1")]
    pub nation_code: ::prost::alloc::string::String,
    /// 行政区划代码，规则详见：行政区划代码说明
    #[prost(string, tag = "2")]
    pub adcode: ::prost::alloc::string::String,
    /// 城市代码，由国家码+行政区划代码(提出城市级别)组合而来，总共为9位
    #[prost(string, tag = "3")]
    pub city_code: ::prost::alloc::string::String,
    /// 行政区划名称
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// 行政区划中心点坐标
    #[prost(message, optional, tag = "5")]
    pub gps: ::core::option::Option<Gps>,
}
/// 付费课程批次卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardCurrBatch {
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
}
/// 付费课程系列卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardCurrSeason {
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
}
/// PGC视频卡片数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardPgc {
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
    /// 番剧是否为预览视频 0:否，1:是
    #[prost(int32, tag = "13")]
    pub is_preview: i32,
    /// 尺寸信息
    #[prost(message, optional, tag = "14")]
    pub dimension: ::core::option::Option<Dimension>,
    /// 角标
    #[prost(message, repeated, tag = "15")]
    pub badge: ::prost::alloc::vec::Vec<VideoBadge>,
    /// 是否能够自动播放
    #[prost(int32, tag = "16")]
    pub can_play: i32,
    /// PGC单季信息
    #[prost(message, optional, tag = "17")]
    pub season: ::core::option::Option<PgcSeason>,
}
/// UGC视频卡片数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardUgc {
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
    /// 角标
    #[prost(message, repeated, tag = "11")]
    pub badge: ::prost::alloc::vec::Vec<VideoBadge>,
    /// 是否能够自动播放
    #[prost(int32, tag = "12")]
    pub can_play: i32,
}
/// 粉丝样式
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecoCardFan {
    /// 是否是粉丝
    #[prost(int32, tag = "1")]
    pub is_fan: i32,
    /// 数量
    #[prost(int32, tag = "2")]
    pub number: i32,
    /// 颜色
    #[prost(string, tag = "3")]
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
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// 点击跳转链接
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// emoji类型
    #[prost(string, tag = "4")]
    pub emoji_type: ::prost::alloc::string::String,
    /// 商品类型
    #[prost(string, tag = "5")]
    pub goods_type: ::prost::alloc::string::String,
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
/// 动态卡片项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicItem {
    /// 卡片类型
    /// forward:转发 av:稿件视频 fold:折叠 pgc:pgc内容 courses:付费视频 upList:最近访问列表 followList:我的追番列表
    #[prost(string, tag = "1")]
    pub card_type: ::prost::alloc::string::String,
    /// 转发类型下，items的类型
    #[prost(string, tag = "2")]
    pub item_type: ::prost::alloc::string::String,
    /// 模块内容
    #[prost(message, repeated, tag = "3")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
    /// 动态ID str
    #[prost(string, tag = "4")]
    pub dyn_id_str: ::prost::alloc::string::String,
    /// 转发动态ID str
    #[prost(string, tag = "5")]
    pub orig_dyn_id_str: ::prost::alloc::string::String,
    /// r_type
    #[prost(int32, tag = "6")]
    pub r_type: i32,
    /// 该卡片下面是否含有折叠卡
    #[prost(int32, tag = "7")]
    pub has_fold: i32,
}
/// 批量动态id获取动态详情返回值
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynDetailsReply {
    /// 动态列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
}
/// 批量动态id获取动态详情请求参数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynDetailsReq {
    /// 青少年模式
    #[prost(int32, tag = "1")]
    pub teenagers_mode: i32,
    /// 动态id
    #[prost(string, tag = "2")]
    pub dynamic_ids: ::prost::alloc::string::String,
    /// 清晰度
    #[prost(int32, tag = "3")]
    pub qn: i32,
    /// 流版本
    #[prost(int32, tag = "4")]
    pub fnver: i32,
    /// 流功能
    #[prost(int32, tag = "5")]
    pub fnval: i32,
    /// 是否强制使用域名
    #[prost(int32, tag = "6")]
    pub force_host: i32,
    /// 是否4k
    #[prost(int32, tag = "7")]
    pub fourk: i32,
}
/// 查看更多-搜索-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynMixUpListSearchReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<MixUpListItem>,
}
/// 查看更多-搜索-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynMixUpListSearchReq {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// 查看更多-列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynMixUpListViewMoreReply {
    /// 关注up主列表信息
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<MixUpListItem>,
    /// 默认搜索文案
    #[prost(string, tag = "2")]
    pub search_default_text: ::prost::alloc::string::String,
}
/// 动态同城物料
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynOurCityItem {
    /// 卡片类型
    /// av:稿件 draw:图文
    #[prost(string, tag = "1")]
    pub card_type: ::prost::alloc::string::String,
    /// 动态ID
    #[prost(int64, tag = "2")]
    pub dyn_id: i64,
    /// 跳转地址
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 模块列表
    #[prost(message, repeated, tag = "4")]
    pub modules: ::prost::alloc::vec::Vec<DynOurCityModule>,
    /// 资源ID
    #[prost(int64, tag = "5")]
    pub rid: i64,
    /// 透传服务端魔镜参数
    #[prost(string, tag = "6")]
    pub debug_info: ::prost::alloc::string::String,
}
/// 动态同城物料模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynOurCityModule {
    /// 类型
    /// cover:封面 desc:描述 author:发布人 extend:扩展部分
    #[prost(string, tag = "1")]
    pub module_type: ::prost::alloc::string::String,
    ///
    #[prost(oneof = "dyn_our_city_module::ModuleItem", tags = "2, 3, 4, 5")]
    pub module_item: ::core::option::Option<dyn_our_city_module::ModuleItem>,
}
/// Nested message and enum types in `DynOurCityModule`.
pub mod dyn_our_city_module {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModuleItem {
        /// 封面
        #[prost(message, tag = "2")]
        ModuleCover(super::DynOurCityModuleCover),
        /// 描述
        #[prost(message, tag = "3")]
        ModuleDesc(super::DynOurCityModuleDesc),
        /// 发布人
        #[prost(message, tag = "4")]
        ModuleAuthor(super::DynOurCityModuleAuthor),
        /// 扩展部分
        #[prost(message, tag = "5")]
        ModuleExtend(super::DynOurCityModuleExtend),
    }
}
/// 动态同城物料-发布人模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynOurCityModuleAuthor {
    /// 用户Mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 用户昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 用户头像
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
}
/// 动态同城物料-封面模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynOurCityModuleCover {
    /// 封面图 单图样式取第一个元素
    #[prost(string, repeated, tag = "1")]
    pub covers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 封面样式
    /// 1:横图 2:竖图 3:方图
    #[prost(int32, tag = "2")]
    pub style: i32,
    /// 视频封面展示项图标 1
    #[prost(int32, tag = "3")]
    pub cover_left_icon_1: i32,
    /// 视频封面展示项 1
    #[prost(string, tag = "4")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    /// 视频封面展示项图标 2
    #[prost(int32, tag = "5")]
    pub cover_left_icon_2: i32,
    /// 视频封面展示项 2
    #[prost(string, tag = "6")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    /// 封面视频展示项 3
    #[prost(string, tag = "7")]
    pub cover_left_text_3: ::prost::alloc::string::String,
    /// 角标
    #[prost(message, repeated, tag = "8")]
    pub badge: ::prost::alloc::vec::Vec<VideoBadge>,
}
/// 动态同城物料-描述模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynOurCityModuleDesc {
    /// 描述信息
    #[prost(string, tag = "1")]
    pub desc: ::prost::alloc::string::String,
}
/// 动态同城物料-扩展部分模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynOurCityModuleExtend {
    /// 类型
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(oneof = "dyn_our_city_module_extend::Extend", tags = "2")]
    pub extend: ::core::option::Option<dyn_our_city_module_extend::Extend>,
}
/// Nested message and enum types in `DynOurCityModuleExtend`.
pub mod dyn_our_city_module_extend {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Extend {
        /// LBS模块
        #[prost(message, tag = "2")]
        ExtendLbs(super::DynOurCityModuleExtendLbs),
    }
}
/// 动态同城物料extent-LBS模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynOurCityModuleExtendLbs {
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
/// 动态同城-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynOurCityReply {
    /// 翻页游标
    #[prost(string, tag = "1")]
    pub offset: ::prost::alloc::string::String,
    /// 是否还有更多数据
    /// 1:有
    #[prost(int32, tag = "2")]
    pub has_more: i32,
    /// 样式类型
    /// 1:双列 2:瀑布流
    #[prost(int32, tag = "3")]
    pub style: i32,
    /// 顶导信息
    #[prost(string, tag = "4")]
    pub top_label: ::prost::alloc::string::String,
    /// 列表详情
    #[prost(message, repeated, tag = "5")]
    pub list: ::prost::alloc::vec::Vec<DynOurCityItem>,
    /// 顶导按钮信息
    #[prost(string, tag = "6")]
    pub top_button_label: ::prost::alloc::string::String,
    /// 城市ID
    #[prost(int32, tag = "7")]
    pub city_id: i32,
    /// 城市名
    #[prost(string, tag = "8")]
    pub city_name: ::prost::alloc::string::String,
}
/// 动态同城页-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynOurCityReq {
    /// 城市ID
    #[prost(int64, tag = "1")]
    pub city_id: i64,
    /// 纬度
    #[prost(double, tag = "2")]
    pub lat: f64,
    /// 经度
    #[prost(double, tag = "3")]
    pub lng: f64,
    /// 透传上一次接口请求返回的offset
    #[prost(string, tag = "4")]
    pub offset: ::prost::alloc::string::String,
    /// 每页元素个数
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// 青少年模式
    /// 1:开启青少年模式
    #[prost(int32, tag = "6")]
    pub teenagers_mode: i32,
    /// 清晰度(旧版)
    #[prost(int32, tag = "7")]
    pub qn: i32,
    /// 流版本(旧版)
    #[prost(int32, tag = "8")]
    pub fnver: i32,
    /// 流类型(旧版)
    #[prost(int32, tag = "9")]
    pub fnval: i32,
    /// 是否强制使用域名(旧版)
    #[prost(int32, tag = "10")]
    pub force_host: i32,
    /// 是否4k(旧版)
    #[prost(int32, tag = "11")]
    pub fourk: i32,
    /// 是否开启lbs
    /// 0:关闭 1:开启
    #[prost(int32, tag = "12")]
    pub lbs_state: i32,
    /// 是否刷新城市
    #[prost(uint32, tag = "13")]
    pub refresh_city: u32,
    /// 魔镜设置
    #[prost(message, optional, tag = "14")]
    pub exp_conf: ::core::option::Option<ExpConf>,
    /// 秒开参数
    #[prost(message, optional, tag = "15")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    /// 城市码
    #[prost(int64, tag = "16")]
    pub city_code: i64,
    /// 构建时间
    #[prost(int64, tag = "17")]
    pub build_time: i64,
}
/// 动态同城开关-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynOurCitySwitchReq {
    /// 开关参数
    /// 0:关闭 1:开启
    #[prost(int32, tag = "1")]
    pub switch: i32,
}
/// 红点接口物料
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRedItem {
    /// 数字红点有效 更新数
    #[prost(uint64, tag = "1")]
    pub count: u64,
}
/// 红点接口-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRedReply {
    /// 类型
    /// count:数字红点 point:普通红点 no_point:没有红点
    #[prost(string, tag = "1")]
    pub red_type: ::prost::alloc::string::String,
    /// 红点具体信息
    #[prost(message, optional, tag = "2")]
    pub dyn_red_item: ::core::option::Option<DynRedItem>,
    /// 默认tab 值对应tab接口下发的anchor
    #[prost(string, tag = "3")]
    pub default_tab: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub red_style: ::core::option::Option<DynRedStyle>,
}
/// 动态红点接口-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRedReq {
    /// 动态红点接口各tab offset信息
    #[prost(message, repeated, tag = "1")]
    pub tab_offset: ::prost::alloc::vec::Vec<TabOffset>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRedStyle {
    ///
    #[prost(int32, tag = "1")]
    pub bg_type: i32,
    ///
    #[prost(int32, tag = "2")]
    pub corner_type: i32,
    ///
    #[prost(int32, tag = "3")]
    pub display_time: i32,
    ///
    #[prost(string, tag = "4")]
    pub corner_mark: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub up: ::core::option::Option<DynRedStyleUp>,
    ///
    #[prost(int32, tag = "6")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRedStyleUp {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(string, tag = "2")]
    pub face: ::prost::alloc::string::String,
}
/// 动态tab详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynTab {
    /// tab标题 优先展示用,未开启状态第一次请求返回同城,后续请求返回对应城市名
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 气泡内容
    #[prost(string, tag = "3")]
    pub bubble: ::prost::alloc::string::String,
    /// 是否推红点
    #[prost(int32, tag = "4")]
    pub red_point: i32,
    /// 城市ID
    #[prost(int64, tag = "5")]
    pub city_id: i64,
    /// 是否弹窗
    #[prost(int32, tag = "6")]
    pub is_popup: i32,
    /// 弹窗内容
    #[prost(message, optional, tag = "7")]
    pub popup: ::core::option::Option<Popup>,
    /// 是否默认tab
    #[prost(bool, tag = "8")]
    pub default_tab: bool,
    /// 副标题 对应城市名
    #[prost(string, tag = "9")]
    pub sub_title: ::prost::alloc::string::String,
    /// 锚点字段
    #[prost(string, tag = "10")]
    pub anchor: ::prost::alloc::string::String,
    /// 内测文案
    #[prost(string, tag = "11")]
    pub internal_test: ::prost::alloc::string::String,
}
/// 动态tab页-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynTabReply {
    /// 动态tab详情列表
    #[prost(message, repeated, tag = "1")]
    pub dyn_tab: ::prost::alloc::vec::Vec<DynTab>,
}
/// 动态tab页-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynTabReq {
    /// 青少年模式
    /// 1:开启青少年模式
    #[prost(int32, tag = "1")]
    pub teenagers_mode: i32,
}
/// 最近访问-标记已读-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynUpdOffsetReq {
    /// 被访问者的UID
    #[prost(int64, tag = "1")]
    pub host_uid: i64,
    /// 用户已读进度
    #[prost(string, tag = "2")]
    pub read_offset: ::prost::alloc::string::String,
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
    #[prost(int32, tag = "3")]
    pub has_more: i32,
    /// 已读进度
    #[prost(string, tag = "4")]
    pub read_offset: ::prost::alloc::string::String,
}
/// 最近访问-个人feed流列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoPersonalReq {
    /// 青少年模式
    /// 1:开启青少年模式
    #[prost(int32, tag = "1")]
    pub teenagers_mode: i32,
    /// 被访问者的mid
    #[prost(int64, tag = "2")]
    pub host_uid: i64,
    /// 偏移量 第一页可传空
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    /// 标明下拉几次
    #[prost(int32, tag = "4")]
    pub page: i32,
    /// 是否是预加载
    #[prost(int32, tag = "5")]
    pub is_preload: i32,
    /// 清晰度
    #[prost(int32, tag = "6")]
    pub qn: i32,
    /// 流版本
    #[prost(int32, tag = "7")]
    pub fnver: i32,
    /// 流类型
    #[prost(int32, tag = "8")]
    pub fnval: i32,
    /// 是否强制使用域名
    #[prost(int32, tag = "9")]
    pub force_host: i32,
    /// 是否4k
    #[prost(int32, tag = "10")]
    pub fourk: i32,
}
/// 动态视频页-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoReq {
    /// 青少年模式
    #[prost(int32, tag = "1")]
    pub teenagers_mode: i32,
    /// 透传 update_baseline
    #[prost(string, tag = "2")]
    pub update_baseline: ::prost::alloc::string::String,
    /// 透传 history_offset
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    /// 向下翻页数
    #[prost(int32, tag = "4")]
    pub page: i32,
    /// 刷新方式
    /// 1:向上刷新 2:向下翻页
    #[prost(int32, tag = "5")]
    pub refresh_type: i32,
    /// 清晰度
    #[prost(int32, tag = "6")]
    pub qn: i32,
    /// 流版本
    #[prost(int32, tag = "7")]
    pub fnver: i32,
    /// 流类型
    #[prost(int32, tag = "8")]
    pub fnval: i32,
    /// 是否强制使用域名
    #[prost(int32, tag = "9")]
    pub force_host: i32,
    /// 是否4K
    #[prost(int32, tag = "10")]
    pub fourk: i32,
}
/// 动态视频页-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoReqReply {
    /// 动态列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DynamicItem>,
    /// 更新的动态数
    #[prost(int32, tag = "2")]
    pub update_num: i32,
    /// 历史偏移
    #[prost(string, tag = "3")]
    pub history_offset: ::prost::alloc::string::String,
    /// 更新基础信息
    #[prost(string, tag = "4")]
    pub update_baseline: ::prost::alloc::string::String,
    /// 是否还有更多数据
    #[prost(int32, tag = "5")]
    pub has_more: i32,
}
/// 魔镜实验配置项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exp {
    /// 实验名
    #[prost(string, tag = "1")]
    pub exp_name: ::prost::alloc::string::String,
    /// 实验组
    #[prost(string, tag = "2")]
    pub exp_group: ::prost::alloc::string::String,
}
/// 魔镜设置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpConf {
    /// 是否是魔镜请求
    #[prost(int32, tag = "1")]
    pub exp_enable: i32,
    /// 实验配置
    #[prost(message, repeated, tag = "2")]
    pub exps: ::prost::alloc::vec::Vec<Exp>,
}
/// 拓展
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extend {
    /// 类型
    /// topic:话题小卡 lbs:lbs hot:热门视频 game:游戏
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// 卡片详情
    #[prost(oneof = "extend::Extend", tags = "2, 3, 4, 5")]
    pub extend: ::core::option::Option<extend::Extend>,
}
/// Nested message and enum types in `Extend`.
pub mod extend {
    /// 卡片详情
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Extend {
        /// 话题小卡
        #[prost(message, tag = "2")]
        ExtInfoTopic(super::ExtInfoTopic),
        /// lbs
        #[prost(message, tag = "3")]
        ExtInfoLbs(super::ExtInfoLbs),
        /// 热门视频
        #[prost(message, tag = "4")]
        ExtInfoHot(super::ExtInfoHot),
        /// 游戏
        #[prost(message, tag = "5")]
        ExtInfoGame(super::ExtInfoGame),
    }
}
/// 拓展信息-游戏小卡
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
/// 拓展信息-热门视频
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
/// 拓展信息-lbs
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
/// 拓展信息-话题小卡
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
/// 我的追番列表Item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowListItem {
    /// season_id
    #[prost(int32, tag = "1")]
    pub season_id: i32,
    /// 标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    /// 最新ep
    #[prost(message, optional, tag = "5")]
    pub new_ep: ::core::option::Option<NewEp>,
}
/// 位置定位-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoCoderReply {
    /// 以行政区划+道路+门牌号等信息组成的标准格式化地址
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// 地址部件，address不满足需求时可自行拼接
    #[prost(message, optional, tag = "2")]
    pub address_component: ::core::option::Option<AddressComponent>,
    /// 行政区划信息
    #[prost(message, optional, tag = "3")]
    pub ad_info: ::core::option::Option<AdInfo>,
}
/// 位置定位-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoCoderReq {
    /// 纬度
    #[prost(double, tag = "1")]
    pub lat: f64,
    /// 经度
    #[prost(double, tag = "2")]
    pub lng: f64,
    /// 页面来源
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
}
/// 行政区划中心点坐标
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gps {
    /// 纬度
    #[prost(double, tag = "1")]
    pub lat: f64,
    /// 经度
    #[prost(double, tag = "2")]
    pub lng: f64,
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
    #[prost(int32, tag = "2")]
    pub is_like: i32,
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
/// 直播信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveInfo {
    /// 是否在直播
    /// 0:未直播 1:正在直播
    #[prost(int32, tag = "1")]
    pub is_living: i32,
    /// 跳转链接
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
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
    pub premiere_state: i32,
    ///
    #[prost(string, tag = "11")]
    pub uri: ::prost::alloc::string::String,
}
/// 直播信息
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
/// 模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// 类型
    /// fold:折叠 author:发布人 dynamic:动态卡片内容 state:计数信息 forward:转发 extend:小卡信息 dispute:争议小黄条 desc:描述信息
    /// likeUser:点赞用户 upList:最近访问列表 followList:我的追番
    #[prost(string, tag = "1")]
    pub module_type: ::prost::alloc::string::String,
    #[prost(oneof = "module::ModuleItem", tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
    pub module_item: ::core::option::Option<module::ModuleItem>,
}
/// Nested message and enum types in `Module`.
pub mod module {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModuleItem {
        /// 折叠
        #[prost(message, tag = "2")]
        ModuleFold(super::ModuleFold),
        /// 发布人
        #[prost(message, tag = "3")]
        ModuleAuthor(super::ModuleAuthor),
        /// 动态卡片内容
        #[prost(message, tag = "4")]
        ModuleDynamic(super::ModuleDynamic),
        /// 计数信息
        #[prost(message, tag = "5")]
        ModuleState(super::ModuleState),
        /// 转发
        #[prost(message, tag = "6")]
        ModuleForward(super::ModuleForward),
        /// 小卡信息
        #[prost(message, tag = "7")]
        ModuleExtend(super::ModuleExtend),
        /// 争议小黄条
        #[prost(message, tag = "8")]
        ModuleDispute(super::ModuleDispute),
        /// 描述信息
        #[prost(message, tag = "9")]
        ModuleDesc(super::ModuleDesc),
        /// 点赞用户
        #[prost(message, tag = "10")]
        ModuleLikeUser(super::ModuleLikeUser),
        /// 最近访问列表
        #[prost(message, tag = "11")]
        ModuleUpList(super::ModuleDynUpList),
        /// 我的追番
        #[prost(message, tag = "12")]
        ModuleFollowList(super::ModuleFollowList),
    }
}
/// 作者信息模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAuthor {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 时间标签
    #[prost(string, tag = "2")]
    pub ptime_label_text: ::prost::alloc::string::String,
    /// 用户详情
    #[prost(message, optional, tag = "3")]
    pub author: ::core::option::Option<UserInfo>,
    /// 装扮卡片
    #[prost(message, optional, tag = "4")]
    pub decorate_card: ::core::option::Option<DecorateCard>,
}
/// 文本内容模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDesc {
    /// 文本描述
    #[prost(message, repeated, tag = "1")]
    pub desc: ::prost::alloc::vec::Vec<Description>,
}
/// 争议小黄条模块
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
/// 动态详情模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDynamic {
    /// 卡片类型
    /// ugc:ugc卡 pgc:pgc卡 currSeason:付费课程系列 currBatch:付费课程批次
    #[prost(string, tag = "1")]
    pub card_type: ::prost::alloc::string::String,
    /// 正文卡片
    #[prost(oneof = "module_dynamic::Card", tags = "2, 3, 4, 5")]
    pub card: ::core::option::Option<module_dynamic::Card>,
}
/// Nested message and enum types in `ModuleDynamic`.
pub mod module_dynamic {
    /// 正文卡片
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Card {
        /// ugc卡
        #[prost(message, tag = "2")]
        CardUgc(super::CardUgc),
        /// pgc卡
        #[prost(message, tag = "3")]
        CardPgc(super::CardPgc),
        /// 付费课程系列
        #[prost(message, tag = "4")]
        CardCurrSeason(super::CardCurrSeason),
        /// 付费课程批次
        #[prost(message, tag = "5")]
        CardCurrBatch(super::CardCurrBatch),
    }
}
/// 最近访问up主列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDynUpList {
    /// 标题展示文案
    #[prost(string, tag = "1")]
    pub module_title: ::prost::alloc::string::String,
    /// “全部”按钮文案
    #[prost(string, tag = "2")]
    pub show_all: ::prost::alloc::string::String,
    /// up主列表
    #[prost(message, repeated, tag = "3")]
    pub list: ::prost::alloc::vec::Vec<UpListItem>,
}
/// 拓展信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleExtend {
    /// 拓展
    #[prost(message, repeated, tag = "1")]
    pub extend: ::prost::alloc::vec::Vec<Extend>,
}
/// 折叠模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleFold {
    /// 折叠分类(该字段废弃)
    #[prost(int32, tag = "1")]
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
    /// 折叠分类
    #[prost(enumeration = "FoldType", tag = "5")]
    pub fold_type_v2: i32,
}
/// 我的追番列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleFollowList {
    /// 查看全部的跳转链接
    #[prost(string, tag = "1")]
    pub view_all_link: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub list: ::prost::alloc::vec::Vec<FollowListItem>,
}
/// 转发模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleForward {
    /// 卡片类型
    #[prost(string, tag = "1")]
    pub card_type: ::prost::alloc::string::String,
    /// 嵌套模型
    #[prost(message, repeated, tag = "2")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
}
/// 点赞用户模块
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
/// 计数信息模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleState {
    /// 转发数
    #[prost(int32, tag = "1")]
    pub repost: i32,
    /// 点赞数
    #[prost(int32, tag = "2")]
    pub like: i32,
    /// 评论数
    #[prost(int32, tag = "3")]
    pub reply: i32,
    /// 点赞拓展信息
    #[prost(message, optional, tag = "4")]
    pub like_info: ::core::option::Option<LikeInfo>,
    /// 禁评
    #[prost(bool, tag = "5")]
    pub no_comment: bool,
    /// 禁转
    #[prost(bool, tag = "6")]
    pub no_forward: bool,
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
/// 空响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoReply {}
/// 空请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoReq {}
/// 认证信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialVerify {
    /// 认证类型
    /// 127:未认证 0:个人 1:机构
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 认证描述
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub is_atten: i32,
}
/// 动态同城点击上报-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OurCityClickReportReply {}
/// 动态同城点击上报-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OurCityClickReportReq {
    /// 动态ID
    #[prost(string, tag = "1")]
    pub dynamic_id: ::prost::alloc::string::String,
    /// 城市ID
    #[prost(int64, tag = "2")]
    pub city_id: i64,
    /// 纬度
    #[prost(double, tag = "3")]
    pub lat: f64,
    /// 经度
    #[prost(double, tag = "4")]
    pub lng: f64,
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
/// 秒开参数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPreloadParams {
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
/// 动态tab弹窗详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Popup {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 文案
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    /// 文案附加跳转地址
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
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
/// 分享需要
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareInfo {
    /// 稿件avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 稿件bvid
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// 副标题
    #[prost(string, tag = "4")]
    pub subtitle: ::prost::alloc::string::String,
    /// 稿件封面
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    /// up mid
    #[prost(int64, tag = "6")]
    pub mid: i64,
    /// up昵称
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
}
/// 小视频卡片项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SVideoItem {
    /// 卡片类型
    /// av:稿件视频
    #[prost(string, tag = "1")]
    pub card_type: ::prost::alloc::string::String,
    /// 模块内容
    #[prost(message, repeated, tag = "2")]
    pub modules: ::prost::alloc::vec::Vec<SVideoModule>,
    /// 动态ID str
    #[prost(string, tag = "3")]
    pub dyn_id_str: ::prost::alloc::string::String,
    /// 卡片游标
    #[prost(int64, tag = "4")]
    pub index: i64,
}
/// 小视频模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SVideoModule {
    /// 类型
    /// author:发布人 player:播放器内容 desc:描述信息 stat:计数信息
    #[prost(string, tag = "1")]
    pub module_type: ::prost::alloc::string::String,
    #[prost(oneof = "s_video_module::ModuleItem", tags = "2, 3, 4, 5")]
    pub module_item: ::core::option::Option<s_video_module::ModuleItem>,
}
/// Nested message and enum types in `SVideoModule`.
pub mod s_video_module {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModuleItem {
        /// 发布人
        #[prost(message, tag = "2")]
        ModuleAuthor(super::SVideoModuleAuthor),
        /// 播放器内容
        #[prost(message, tag = "3")]
        ModulePlayer(super::SVideoModulePlayer),
        /// 描述信息
        #[prost(message, tag = "4")]
        ModuleDesc(super::SVideoModuleDesc),
        /// 计数信息
        #[prost(message, tag = "5")]
        ModuleStat(super::SVideoModuleStat),
    }
}
/// 作者信息模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SVideoModuleAuthor {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 用户昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 用户头像
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    /// 发布描述
    #[prost(string, tag = "4")]
    pub pub_desc: ::prost::alloc::string::String,
    /// 是否关注up
    /// 1:已关注
    #[prost(int32, tag = "5")]
    pub is_attention: i32,
    /// 跳转地址
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
}
/// 文本内容模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SVideoModuleDesc {
    /// 文本内容
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
/// 播放器模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SVideoModulePlayer {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    /// 跳转地址，秒开地址如果有会拼接player_preload可参考天马
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// aid
    #[prost(int64, tag = "4")]
    pub aid: i64,
    /// cid
    #[prost(int64, tag = "5")]
    pub cid: i64,
    /// 视频时长
    #[prost(int64, tag = "6")]
    pub duration: i64,
    /// 尺寸信息
    #[prost(message, optional, tag = "7")]
    pub dimension: ::core::option::Option<Dimension>,
}
/// 计数信息模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SVideoModuleStat {
    /// 计数内容
    #[prost(message, repeated, tag = "1")]
    pub stat_info: ::prost::alloc::vec::Vec<SVideoStatInfo>,
    /// 分享需要
    #[prost(message, optional, tag = "2")]
    pub share_info: ::core::option::Option<ShareInfo>,
}
/// 小视频连播页-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SVideoReply {
    /// 列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<SVideoItem>,
    /// 翻页游标
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    /// 是否还有更多数据
    /// 1:有
    #[prost(int32, tag = "3")]
    pub has_more: i32,
    /// 顶部
    #[prost(message, optional, tag = "4")]
    pub top: ::core::option::Option<SVideoTop>,
}
/// 小视频连播页-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SVideoReq {
    /// 当前素材的id
    #[prost(int64, tag = "1")]
    pub oid: i64,
    /// 当前素材类型
    /// 1:动态(如果有oid则必传) 2:热门分类 3:热点聚合
    #[prost(enumeration = "SVideoType", tag = "2")]
    pub r#type: i32,
    /// 翻页offset
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    /// 清晰度(旧版)
    #[prost(int32, tag = "4")]
    pub qn: i32,
    /// 流版本(旧版)
    #[prost(int32, tag = "5")]
    pub fnver: i32,
    /// 流类型(旧版)
    #[prost(int32, tag = "6")]
    pub fnval: i32,
    /// 是否强制使用域名(旧版)
    #[prost(int32, tag = "7")]
    pub force_host: i32,
    /// 是否4k(旧版)
    #[prost(int32, tag = "8")]
    pub fourk: i32,
    /// 当前页面spm
    #[prost(string, tag = "9")]
    pub spmid: ::prost::alloc::string::String,
    /// 上级页面spm
    #[prost(string, tag = "10")]
    pub from_spmid: ::prost::alloc::string::String,
    /// 秒开参数
    #[prost(message, optional, tag = "11")]
    pub player_preload: ::core::option::Option<PlayerPreloadParams>,
    /// 热门进入联播页的锚点aid
    #[prost(int64, tag = "12")]
    pub focus_aid: i64,
    /// 秒开参数
    #[prost(message, optional, tag = "13")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
/// 计数内容
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SVideoStatInfo {
    /// 计数icon
    /// 1:分享符号 2:评论符号 3:点赞符号
    #[prost(int32, tag = "1")]
    pub icon: i32,
    /// 计数值
    #[prost(int64, tag = "2")]
    pub num: i64,
    /// 选中状态
    /// 1:选中
    #[prost(int32, tag = "3")]
    pub selected: i32,
    /// 跳转链接(如评论)
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
}
/// 顶部
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SVideoTop {
    /// 联播页标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 联播页导语
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
/// 动态红点接口各tab offset信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabOffset {
    /// 1:综合页 2:视频页
    #[prost(int32, tag = "1")]
    pub tab: i32,
    /// 上一次对应列表页offset
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
}
/// up主列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpListItem {
    /// 是否有更新
    /// 0:没有 1:有
    #[prost(int32, tag = "1")]
    pub has_update: i32,
    /// up主头像
    #[prost(string, tag = "2")]
    pub face: ::prost::alloc::string::String,
    /// up主昵称
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// up主uid
    #[prost(int64, tag = "4")]
    pub uid: i64,
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
}
/// 大会员标签
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipLabel {
    /// 图片地址
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BgType {
    ///
    Default = 0,
    ///
    Face = 1,
}
impl BgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BgType::Default => "bg_type_default",
            BgType::Face => "bg_type_face",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "bg_type_default" => Some(Self::Default),
            "bg_type_face" => Some(Self::Face),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CornerType {
    ///
    None = 0,
    ///
    Text = 1,
    ///
    Animation = 2,
}
impl CornerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CornerType::None => "corner_type_none",
            CornerType::Text => "corner_type_text",
            CornerType::Animation => "corner_type_animation",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "corner_type_none" => Some(Self::None),
            "corner_type_text" => Some(Self::Text),
            "corner_type_animation" => Some(Self::Animation),
            _ => None,
        }
    }
}
/// 折叠分类
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FoldType {
    /// 占位
    Zero = 0,
    /// 用户发布折叠
    Publish = 1,
    /// 转发超频折叠
    Frequent = 2,
    /// 联合投稿折叠
    Unite = 3,
    /// 动态受限折叠
    Limit = 4,
}
impl FoldType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FoldType::Zero => "FoldTypeZero",
            FoldType::Publish => "FoldTypePublish",
            FoldType::Frequent => "FoldTypeFrequent",
            FoldType::Unite => "FoldTypeUnite",
            FoldType::Limit => "FoldTypeLimit",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FoldTypeZero" => Some(Self::Zero),
            "FoldTypePublish" => Some(Self::Publish),
            "FoldTypeFrequent" => Some(Self::Frequent),
            "FoldTypeUnite" => Some(Self::Unite),
            "FoldTypeLimit" => Some(Self::Limit),
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
/// 关注状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RelationStatus {
    ///
    None = 0,
    /// 未关注
    Nofollow = 1,
    /// 关注
    Follow = 2,
    /// 被关注
    Followed = 3,
    /// 互相关注
    MutualConcern = 4,
    /// 特别关注
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
/// 入口联播页类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SVideoType {
    /// 无类型
    TypeNone = 0,
    /// 动态
    TypeDynamic = 1,
    /// 热门分类
    TypePopularIndex = 2,
    /// 热点聚合
    TypePopularHotword = 3,
}
impl SVideoType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SVideoType::TypeNone => "TypeNone",
            SVideoType::TypeDynamic => "TypeDynamic",
            SVideoType::TypePopularIndex => "TypePopularIndex",
            SVideoType::TypePopularHotword => "TypePopularHotword",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TypeNone" => Some(Self::TypeNone),
            "TypeDynamic" => Some(Self::TypeDynamic),
            "TypePopularIndex" => Some(Self::TypePopularIndex),
            "TypePopularHotword" => Some(Self::TypePopularHotword),
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
/// Generated client implementations.
pub mod dynamic_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// v1动态
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
        /// 动态视频页
        pub async fn dyn_video(
            &mut self,
            request: impl tonic::IntoRequest<super::DynVideoReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynVideoReqReply>,
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
                "/bilibili.app.dynamic.v1.Dynamic/DynVideo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v1.Dynamic", "DynVideo"));
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
                "/bilibili.app.dynamic.v1.Dynamic/DynDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v1.Dynamic", "DynDetails"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 小视频连播页
        pub async fn s_video(
            &mut self,
            request: impl tonic::IntoRequest<super::SVideoReq>,
        ) -> std::result::Result<tonic::Response<super::SVideoReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v1.Dynamic/SVideo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v1.Dynamic", "SVideo"));
            self.inner.unary(req, path, codec).await
        }
        /// 动态tab页
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
                "/bilibili.app.dynamic.v1.Dynamic/DynTab",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v1.Dynamic", "DynTab"));
            self.inner.unary(req, path, codec).await
        }
        /// 同城接口开关
        pub async fn dyn_our_city_switch(
            &mut self,
            request: impl tonic::IntoRequest<super::DynOurCitySwitchReq>,
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
                "/bilibili.app.dynamic.v1.Dynamic/DynOurCitySwitch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v1.Dynamic",
                        "DynOurCitySwitch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 动态同城页
        pub async fn dyn_our_city(
            &mut self,
            request: impl tonic::IntoRequest<super::DynOurCityReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynOurCityReply>,
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
                "/bilibili.app.dynamic.v1.Dynamic/DynOurCity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v1.Dynamic", "DynOurCity"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 最近访问-个人视频feed流
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
                "/bilibili.app.dynamic.v1.Dynamic/DynVideoPersonal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v1.Dynamic",
                        "DynVideoPersonal",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 最近访问-标记已读
        pub async fn dyn_upd_offset(
            &mut self,
            request: impl tonic::IntoRequest<super::DynUpdOffsetReq>,
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
                "/bilibili.app.dynamic.v1.Dynamic/DynUpdOffset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.dynamic.v1.Dynamic", "DynUpdOffset"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 动态红点接口
        pub async fn dyn_red(
            &mut self,
            request: impl tonic::IntoRequest<super::DynRedReq>,
        ) -> std::result::Result<tonic::Response<super::DynRedReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v1.Dynamic/DynRed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v1.Dynamic", "DynRed"));
            self.inner.unary(req, path, codec).await
        }
        /// 查看更多-列表
        pub async fn dyn_mix_up_list_view_more(
            &mut self,
            request: impl tonic::IntoRequest<super::NoReq>,
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
                "/bilibili.app.dynamic.v1.Dynamic/DynMixUpListViewMore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v1.Dynamic",
                        "DynMixUpListViewMore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 查看更多-搜索
        pub async fn dyn_mix_up_list_search(
            &mut self,
            request: impl tonic::IntoRequest<super::DynMixUpListSearchReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynMixUpListSearchReply>,
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
                "/bilibili.app.dynamic.v1.Dynamic/DynMixUpListSearch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v1.Dynamic",
                        "DynMixUpListSearch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 同城点击上报
        pub async fn our_city_click_report(
            &mut self,
            request: impl tonic::IntoRequest<super::OurCityClickReportReq>,
        ) -> std::result::Result<
            tonic::Response<super::OurCityClickReportReply>,
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
                "/bilibili.app.dynamic.v1.Dynamic/OurCityClickReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.dynamic.v1.Dynamic",
                        "OurCityClickReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 位置定位
        pub async fn geo_coder(
            &mut self,
            request: impl tonic::IntoRequest<super::GeoCoderReq>,
        ) -> std::result::Result<tonic::Response<super::GeoCoderReply>, tonic::Status> {
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
                "/bilibili.app.dynamic.v1.Dynamic/GeoCoder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.dynamic.v1.Dynamic", "GeoCoder"));
            self.inner.unary(req, path, codec).await
        }
    }
}
