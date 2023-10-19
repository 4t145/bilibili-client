use crate::reqwest_client::{ClientError, Client};

pub mod login;
pub mod send_danmaku_to_live;

pub trait Business {
    type Output;
    async fn execute_on(
        self,
        client: &Client,
    ) -> Result<Self::Output, ClientError>;
}


impl Client {
    pub async fn execute<B: Business>(
        &self,
        business: B,
    ) -> Result<B::Output, ClientError> {
        business.execute_on(self).await
    }

    pub async fn execute_default<B: Business + Default>(
        &self,
    ) -> Result<B::Output, ClientError> {
        B::default().execute_on(self).await
    }
}