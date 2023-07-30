//! see https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/misc/device_identity.md
pub struct Fp<'fp> {
    /// Usually XU Prefix BUVID, for DrmId will not change in most cases.
    pub buvid_auth: &'fp str,
    /// `Build.MODEL`
    pub device_model: &'fp str,
    /// `Build.getRadioVersion()`
    pub device_radio_ver: &'fp str,
}


impl<'fp> Fp<'fp> {
    #[allow(dead_code)]
    #[inline]
    fn new(buvid_auth: &'fp str, device_model: &'fp str, device_radio_ver: &'fp str) -> Self {
        Self {
            buvid_auth,
            device_model,
            device_radio_ver,
        }
    }
    #[inline]
    #[allow(clippy::all)]
    pub fn gen(self) -> String {
        let mut fp_raw = String::with_capacity(128);
        let device_fp_md5 = {
            let mut device_fp = String::with_capacity(256);
            device_fp.push_str(self.buvid_auth);
            device_fp.push_str(self.device_model);
            device_fp.push_str(self.device_radio_ver);
            format!("{:0128X}", md5::compute(&device_fp))
        };
        fp_raw.push_str(&device_fp_md5);
        fp_raw.push_str(&chrono::Local::now().format("%Y%m%d%H%M%S").to_string());
        fp_raw.push_str(&crate::utils::random_alphanumeric_string(16).to_lowercase());
        let veri_code_str = {
            let mut veri_code = 0;
            let fp_raw_sub_str = fp_raw
                .as_bytes()
                .chunks(2)
                .map(|s| unsafe { ::std::str::from_utf8_unchecked(s) })
                .collect::<Vec<_>>();
            for i in 0..({
                if fp_raw.len() < 62 {
                    fp_raw.len() - fp_raw.len() % 2 // 取偶数
                } else {
                    62
                }
            } / 2)
            {
                veri_code += i32::from_str_radix(fp_raw_sub_str[i], 16).unwrap_or(0);
            }
            format!("{:0>2x}", veri_code % 256)
        };
        fp_raw.push_str(&veri_code_str);
        fp_raw
    }
}