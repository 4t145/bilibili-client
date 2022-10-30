use std::{sync::Arc};
use tokio::sync::{watch};
use crate::{ReqwestClient, api::live::msg::{LiveDanmaku, send::LiveSend}, ClientError};

use super::{Transaction, Task};

pub struct SendDanmakuToLive {
    pub roomid: u64,
    pub danmaku: LiveDanmaku
}

impl Transaction for SendDanmakuToLive {
    type State = SendDanmakuToLiveState;

    fn excute_on(self, client: Arc<ReqwestClient>) -> Task<Self> {
        use SendDanmakuToLiveState::*;
        let (tx, state) = watch::channel(Sending);
        let handle = tokio::spawn(async move {
            let jct = match client.get_cookie_jct() {
                Some(jct) => jct,
                None => {
                    tx.send(FailOffline).unwrap();
                    return Ok(())
                }
            };
            let req = self.danmaku.as_send_req(self.roomid, jct);
            let resp = client.urlencoded_req::<LiveSend>(req).await.map_err(ClientError::Api)?;
            if resp.code == 0 {
                tx.send(Success).unwrap();
            } else {
                tx.send(Fail).unwrap();
            }
            Ok(())
        });
        Task {
            state,
            handle
        }
    }
}

#[derive(Debug)]
pub enum SendDanmakuToLiveState {
    Sending,
    FailOffline,
    Fail,
    Success,
}

// impl Client {
//     /// send danmaku
//     pub async fn send_danmaku_to_live(&mut self, roomid: u64, danmaku: LiveDanmaku) -> Result<<LiveSend as Api>::Response, ClientError> {
//         let online = self.status.map_online_mut().ok_or(ClientError::Offline)?;
//         let req = online.make_live_send_req(roomid, danmaku);
//         self.debug(serde_json::json!(req).to_string().as_str());
//         let resp = self.urlencoded_req::<LiveSend>(req).await.map_err(ClientError::Api)?;
//         return Ok(resp)
//     }
// }