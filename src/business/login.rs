use crate::{
    api::passport::qrcode::GetLoginInfoRespData,
    reqwest_client::{ClientError, Client},
};

use super::Business;
#[derive(Debug, Default)]
pub struct QrLogin<L>(pub L);

impl<L: LoginByQrCode> Business for QrLogin<L> {
    type Output = String;
    async fn execute_on(self, client: &Client) -> Result<Self::Output, ClientError> {
        login(self.0, client).await
    }
}

pub trait LoginByQrCode {
    async fn update_code(&mut self, url: &str);
    async fn finish(self)
    where
        Self: Sized,
    {
        //
    }
    async fn scanned(&mut self);
    async fn expired(&mut self);
    async fn next_poll(&mut self);
}

impl Client {
    pub async fn qr_login<L: LoginByQrCode>(&self, loginer: L) -> Result<String, ClientError> {
        self.execute(QrLogin(loginer)).await
    }
    pub async fn qr_login_default<L: LoginByQrCode + Default>(
        &self,
    ) -> Result<String, ClientError> {
        self.execute_default::<QrLogin<L>>().await
    }
}

async fn login<L: LoginByQrCode>(
    mut loginer: L,
    client: &Client,
) -> Result<String, ClientError> {
    let resp = client.get_login_url().await?;
    loginer.update_code(&resp.data.url).await;
    let mut oauth_key = resp.data.oauth_key;
    loop {
        loginer.next_poll().await;
        let resp = client.get_login_info(&oauth_key).await?;
        match resp.data {
            GetLoginInfoRespData::Code(code) => match code {
                -2 => {
                    loginer.expired().await;
                    let resp = client.get_login_url().await?;
                    loginer.update_code(&resp.data.url);
                    oauth_key = resp.data.oauth_key;
                    continue;
                }
                -4 => {
                    continue;
                }
                -5 => {
                    loginer.scanned().await;
                    continue;
                }
                other => {
                    unimplemented!("unexpected code: {}", other);
                }
            },
            GetLoginInfoRespData::Body { url } => {
                loginer.finish().await;
                return Ok(url);
            }
        }
    }
}
