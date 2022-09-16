use std::{sync::Arc};
use tokio::sync::{watch};
use crate::{Client, api::passport::qrcode::{GetLoginUrl, GetLoginInfo, GetLoginInfoReq, GetLoginInfoRespData}, ClientError};

use super::{Transaction, Task};

pub struct Login {}

impl Transaction for Login {
    type State = LoginState;

    fn excute_on(self, client: Arc<Client>) -> Task<Self> {
        use LoginState::*;
        let (tx, state) = watch::channel(FetchingQrcode);
        let handle = tokio::spawn(async move {
            let resp = client.urlencoded_req::<GetLoginUrl>(()).await.map_err(ClientError::Api)?;
            let url = resp.data.url;
            let oauth_key = resp.data.oauth_key;
            tx.send(ScaningQrcode(url.clone())).unwrap();
            loop {
                // 500毫秒的等待
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                let resp = client.urlencoded_req::<GetLoginInfo>(
                    GetLoginInfoReq {oauth_key: oauth_key.clone()}
                ).await.map_err(ClientError::Api)?;
                match resp.data {
                    GetLoginInfoRespData::Code(code) => {
                        match code {
                            -2 => {
                                tx.send(QrcodeExpired).unwrap();
                            }
                            -4 => {
                                continue;
                            },
                            -5 => {
                                tx.send(QrcodeScanFinished).unwrap();
                                continue;
                            },
                            other => {
                                tx.send(UnexpectedCode(other)).unwrap();
                                return Ok(());
                            }
                        }
                    },
                    GetLoginInfoRespData::Body { url } => {
                        tx.send(Success{url}).unwrap();
                        return Ok(());
                    },
                }
            }
        });
        Task {
            state,
            handle
        }
    }
}

#[derive(Debug)]
pub enum LoginState {
    FetchingQrcode,
    ScaningQrcode(String),
    QrcodeExpired,
    QrcodeScanFinished,

    // terminated
    UnexpectedCode(i8),
    Success {
        url: String
    },
}