use crate::{
    api::live::msg::LiveDanmaku,
    reqwest_client::{Client, ClientError},
};

use super::Business;

impl Business for SendDanmakuToLive {
    type Output = ();
    async fn execute_on(self, client: &Client) -> Result<Self::Output, ClientError> {
        let Some(jct) = client.get_login_info_from_cookie().bili_jct else {
            return Err(ClientError::Offline);
        };
        let req = self.danmaku.as_send_req(self.roomid, &jct);
        client.live_send(req).await
    }
}

pub struct SendDanmakuToLive {
    pub roomid: u64,
    pub danmaku: LiveDanmaku,
}

impl Client {
    pub async fn send_danmaku_to_live(
        &self,
        business: SendDanmakuToLive,
    ) -> Result<(), ClientError> {
        let Some(jct) = self.get_login_info_from_cookie().bili_jct else {
            return Err(ClientError::Offline);
        };
        let req = business.danmaku.as_send_req(business.roomid, &jct);
        self.live_send(req).await
    }
}
