#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PgcAnyModel {
    #[prost(message, optional, tag = "3")]
    pub business: ::core::option::Option<
        super::super::super::pgc::gateway::player::v2::PlayViewBusinessInfo,
    >,
    #[prost(message, optional, tag = "4")]
    pub event: ::core::option::Option<
        super::super::super::pgc::gateway::player::v2::Event,
    >,
    #[prost(message, optional, tag = "5")]
    pub view_info: ::core::option::Option<
        super::super::super::pgc::gateway::player::v2::ViewInfo,
    >,
    #[prost(message, optional, tag = "6")]
    pub play_ext_conf: ::core::option::Option<
        super::super::super::pgc::gateway::player::v2::PlayAbilityExtConf,
    >,
    #[prost(message, optional, tag = "7")]
    pub play_ext_info: ::core::option::Option<
        super::super::super::pgc::gateway::player::v2::PlayExtInfo,
    >,
}
