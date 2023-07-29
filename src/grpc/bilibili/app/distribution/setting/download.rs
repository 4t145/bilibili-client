///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadSettingsConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub enable_download_auto_start: ::core::option::Option<super::super::v1::BoolValue>,
}
