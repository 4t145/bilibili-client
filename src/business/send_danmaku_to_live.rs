use crate::{
    api::live::msg::{send::LiveSend, LiveDanmaku},
    reqwest_client::{ClientError, ReqwestClient},
};

use super::Business;

impl Business for SendDanmakuToLive {
    type Output = ();
    async fn execute_on(self, client: &ReqwestClient) -> Result<Self::Output, ClientError> {
        let Some(jct) = client.get_login_info().bili_jct else {
            return Err(ClientError::Offline);
        };
        let req = self.danmaku.as_send_req(self.roomid, jct.to_owned());
        client.send_form::<LiveSend>(&req).await?.into()
    }
}

pub struct SendDanmakuToLive {
    pub roomid: u64,
    pub danmaku: LiveDanmaku,
}

impl ReqwestClient {
    pub async fn send_danmaku_to_live(
        &self,
        business: SendDanmakuToLive,
    ) -> Result<(), ClientError> {
        let Some(jct) = self.get_login_info().bili_jct else {
            return Err(ClientError::Offline);
        };
        let req = business
            .danmaku
            .as_send_req(business.roomid, jct.to_owned());
        self.send_form::<LiveSend>(&req).await?.into()
    }
}
