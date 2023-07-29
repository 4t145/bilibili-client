// pub(crate) const API_HOST_URL: &'static str = "https://api.bilibili.com";
// pub(crate) const PASSPORT_HOST_URL: &'static str = "https://passport.bilibili.com";
pub(crate) const AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)";


/*
GRPC METADATA
*/
pub mod grpc {
    // const USER_AGENT: &str = "Dalvik/2.1.0 (Linux; U; Android 12; {{device_model}} Build/{{device_build}}) {{app_ver}} os/android model/{{device_model}} mobi_app/{{mobi_app}} build/{{app_build}} channel/master innerVer/{{app_build_inner}} osVer/12 network/2 grpc-java-cronet/1.36.1";
    pub const X_BILI_GAIA_VTOKEN: &str = "x-bili-gaia-vtoken";
    pub const X_BILI_AURORA_EID: &str = "x-bili-aurora-eid";
    pub const X_BILI_MID: &str = "x-bili-mid";
    pub const X_BILI_AURORA_ZONE: &str = "x-bili-aurora-zone";
    pub const X_BILI_TRACE_ID: &str = "x-bili-trace-id";
    pub const AUTHORIZATION: &str = "authorization";
    pub const BILUVID: &str = "buvid";
    pub const BILI_HTTP_ENGINE: &str = "bili-http-engine";
    pub const TE: &str = "te";
    pub const X_BILI_FAWKES_REQ_BIN: &str = "x-bili-fawkes-req-bin";
    pub const X_BILI_METADATA_BIN: &str = "x-bili-metadata-bin";
    pub const X_BILI_DEVICE_BIN: &str = "x-bili-device-bin";
    pub const X_BILI_NETWORK_BIN: &str = "x-bili-network-bin";
    pub const X_BILI_RESTRICTION_BIN: &str = "x-bili-restriction-bin";
    pub const X_BILI_LOCALE_BIN: &str = "x-bili-locale-bin";
    pub const X_BILI_EXPS_BIN: &str = "x-bili-exps-bin";
}