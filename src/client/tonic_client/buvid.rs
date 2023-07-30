//!
//!
//!
//! see https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/misc/device_identity.md
//! 
#[allow(dead_code)]
pub enum Buvid<'id> {
    /// 请求资源默认使用
    AndroidID(&'id str),
    /// 设备鉴权相关 API 默认使用
    DrmId(&'id str),
    /// 现已弃用
    Imei(&'id str),
    /// 随机 UUID 值, 现已弃用
    Guid(&'id str),
    /// 新版本弃用
    Mac(&'id str),
    /// BStarA
    GoogleId(&'id str),
    /// BStarA
    FacebookId(&'id str),
}

impl<'id> Buvid<'id> {
    #[inline]
    pub fn gen(self) -> String {
        let (buvid_prefix, input_md5) = match self {
            Self::AndroidID(v) => ("XX", md5::compute(v)),
            Self::DrmId(v) => ("XU", md5::compute(v)),
            Self::Imei(v) => ("XZ", md5::compute(v)),
            Self::Guid(v) => ("XW", md5::compute(v.replace('-', ""))),
            Self::Mac(v) => ("XY", md5::compute(v.replace(':', ""))),
            Self::GoogleId(v) => ("XG", md5::compute(v)),
            Self::FacebookId(v) => ("XF", md5::compute(v)),
        };
        let mut buvid_raw_vec = Vec::with_capacity(37);
        buvid_prefix.as_bytes().clone_into(&mut buvid_raw_vec);
        let md5_str = format!("{:0128X}", input_md5);
        let input_str_md5 = md5_str.as_bytes();
        buvid_raw_vec.push(input_str_md5[2]);
        buvid_raw_vec.push(input_str_md5[12]);
        buvid_raw_vec.push(input_str_md5[22]);
        buvid_raw_vec.extend(input_str_md5.iter());
        unsafe { String::from_utf8_unchecked(buvid_raw_vec) }
    }
}
