pub(crate) const PASSPORT_HOST_URL: &'static str = "https://passport.bilibili.com";
pub mod qrcode;
pub struct Passport {
     
}

impl TraitApi for Passport {
    const BASE_URL:&'static str = PASSPORT_HOST_URL;
}

impl Passport {
    pub fn qrcode() {

    }
}

use crate::api_trait::TraitApi;

// pub struct Qrcode {
//     get_login_url: "getLoginUrl"
// }