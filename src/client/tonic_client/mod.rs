use crate::{
    grpc::bilibili::metadata::{
        device::Device,
        fawkes::FawkesReq,
        locale::Locale,
        network::{self, Network, NetworkType, TfType},
        parabox::Exps,
        restriction::{self, Restriction},
        Metadata,
    },
    utils::*,
};
mod buvid;
mod fp;
mod metadata;

pub struct Bin<T> {
    inner: T,
    bin: Vec<u8>,
}

impl<T> std::ops::Deref for Bin<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.inner;
    }
}

impl<T: Message> From<T> for Bin<T> {
    fn from(value: T) -> Bin<T> {
        let bin = value.encode_to_vec();
        Self { inner: value, bin }
    }
}
impl<T: Message> Bin<T> {
    pub fn new(value: T) -> Self {
        Self::from(value)
    }
    pub fn modify(&mut self, f: impl Fn(&mut T)) {
        f(&mut self.inner);
        self.bin = self.inner.encode_to_vec()
    }
    pub fn get_bin(&self) -> &[u8] {
        &self.bin
    }
}
pub struct TonicClient {
    device: Bin<Device>,
    network: Bin<Network>,
    restriction: Bin<Restriction>,
    metadata: Bin<Metadata>,
    locale: Bin<Locale>,
    exps: Bin<Exps>,
    fawkes_req: Bin<FawkesReq>,
    login_info: Option<LoginInfo>,
}

pub struct LoginInfo {
    bid: u64,
    eid: String,
    access_key: String,
}
use std::sync::OnceLock;

use prost::Message;
impl Default for TonicClient {
    fn default() -> Self {
        Self::new()
    }
}

static APP_BUVID: OnceLock<String> = OnceLock::new();
impl TonicClient {
    /// 原生 gRPC 接口
    pub const GRPC_RAW_HOST_URL: &str = "grpc.biliapi.net";
    /// Failover gRPC 接口
    pub const GRPC_FAILOVER_HOST_URL: &str = "app.bilibili.com";
    pub fn new() -> Self {
        Self {
            device: Bin::from(Self::get_default_device()),
            network: Bin::from(Self::get_default_network()),
            restriction: Bin::from(Self::get_default_restriction()),
            metadata: Bin::from(Self::get_default_metadata()),
            locale: Bin::from(Self::get_default_locale()),
            exps: Bin::from(Exps::default()),
            fawkes_req: Bin::from(Self::get_default_fawkes_req()),
            login_info: None,
        }
    }
    pub fn set_login_info(&mut self, login_info: LoginInfo) -> Option<LoginInfo> {
        self.metadata.modify(|x| {
            x.access_key = login_info.access_key.clone();
        });
        self.login_info.replace(login_info)
    }
    pub fn get_buvid() -> &'static str {
        APP_BUVID.get_or_init(|| {
            let buvid = buvid::Buvid::AndroidID(&random_hex_string(64)).gen();
            buvid
        })
    }

    pub fn gen_request<T>(&self, request: T) -> tonic::Request<T> {
        let mut req = tonic::Request::new(request);
        use crate::consts::grpc::*;
        let mid = self
            .login_info
            .as_ref()
            .map(|x| x.bid)
            .unwrap_or_default()
            .to_string();
        let eid = self
            .login_info
            .as_ref()
            .map(|x| x.eid.as_str())
            .unwrap_or_default();
        let trace_id = gen_trace_id();
        let mut ascii_keys = vec![
            (X_BILI_GAIA_VTOKEN, ""),
            (X_BILI_AURORA_EID, &eid),
            (X_BILI_MID, &mid),
            (X_BILI_AURORA_ZONE, ""),
            (X_BILI_TRACE_ID, &trace_id),
            (BILUVID, &self.device.buvid),
            (BILI_HTTP_ENGINE, "cronet"),
            (TE, "trailers"),
        ];
        if let Some(login_info) = self.login_info.as_ref() {
            ascii_keys.extend_from_slice(&[(AUTHORIZATION, &login_info.access_key)]);
        }
        let bin_beys = vec![
            (X_BILI_FAWKES_REQ_BIN, self.fawkes_req.get_bin()),
            (X_BILI_METADATA_BIN, self.metadata.get_bin()),
            (X_BILI_DEVICE_BIN, self.device.get_bin()),
            (X_BILI_NETWORK_BIN, self.network.get_bin()),
            (X_BILI_RESTRICTION_BIN, self.restriction.get_bin()),
            (X_BILI_LOCALE_BIN, self.locale.get_bin()),
            (X_BILI_EXPS_BIN, self.exps.get_bin()),
        ];
        let mut metadata =
            tonic::metadata::MetadataMap::with_capacity(ascii_keys.len() + bin_beys.len());
        for (key, value) in ascii_keys {
            let key = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()).unwrap();
            let value = value.parse().unwrap();
            metadata.append(key, value);
        }

        for (key, value) in bin_beys {
            let key = tonic::metadata::BinaryMetadataKey::from_bytes(key.as_bytes()).unwrap();
            let value = tonic::metadata::BinaryMetadataValue::from_bytes(value);
            metadata.append_bin(key, value);
        }
        *req.metadata_mut() = metadata;
        req
    }
    /*
       DEFAULT VALUES
    */
    const DEFAULT_DEVICE_MODEL: &str = "MI 10";
    const DEFAULT_ANDORID_VER: &str = "10";
    const DEFAULT_PLATFORM: &str = "android";
    const DEFAULT_MOBI_APP: &str = "android";
    const DEFAULT_CHANNEL: &str = "bili";
    const DEFAULT_DEVICE: &str = "phone";
    const DEFAULT_BUILD: i32 = 6830300;

    pub fn get_default_metadata() -> Metadata {
        let buvid = Self::get_buvid();
        Metadata {
            access_key: "".to_string(),
            mobi_app: Self::DEFAULT_MOBI_APP.to_string(),
            device: Self::DEFAULT_DEVICE.to_string(),
            build: Self::DEFAULT_BUILD,
            channel: Self::DEFAULT_CHANNEL.to_string(),
            buvid: buvid.to_string(),
            platform: Self::DEFAULT_PLATFORM.to_string(),
        }
    }
    pub fn get_default_device() -> Device {
        let buvid = Self::get_buvid();
        let fp = fp::Fp {
            buvid_auth: buvid,
            device_model: Self::DEFAULT_DEVICE_MODEL,
            device_radio_ver: Self::DEFAULT_ANDORID_VER,
        }
        .gen();
        Device {
            app_id: 1,
            build: Self::DEFAULT_BUILD,
            buvid: buvid.to_string(),
            mobi_app: Self::DEFAULT_MOBI_APP.to_string(),
            platform: Self::DEFAULT_PLATFORM.to_string(),
            device: Self::DEFAULT_DEVICE.to_string(),
            channel: Self::DEFAULT_CHANNEL.to_string(),
            brand: "Xiaomi".to_string(),
            model: Self::DEFAULT_DEVICE_MODEL.to_string(),
            osver: Self::DEFAULT_ANDORID_VER.to_string(),
            fp_local: fp.clone(),
            fp_remote: fp.clone(),
            version_name: "".to_string(),
            fp,
            fts: chrono::Utc::now().timestamp_millis(),
        }
    }
    pub fn get_default_network() -> Network {
        Network {
            r#type: NetworkType::Wifi.into(),
            tf: TfType::TfUnknown.into(),
            ..Default::default()
        }
    }
    pub fn get_default_restriction() -> Restriction {
        Restriction {
            ..Default::default()
        }
    }
    pub fn get_default_locale() -> Locale {
        Locale {
            timezone: chrono::Utc::now().timezone().to_string(),
            sim_code: "+86".to_string(),
            ..Default::default()
        }
    }
    pub fn get_default_fawkes_req() -> FawkesReq {
        FawkesReq {
            appkey: "android64".to_string(),
            env: "prod".to_string(),
            session_id: random_alphanumeric_string(8).to_lowercase(),
        }
    }
}
