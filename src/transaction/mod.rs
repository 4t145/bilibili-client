pub mod login;
pub mod send_danmaku_to_live;


use tokio::{sync::{RwLock, watch}, task::JoinHandle};
use std::{sync::Arc};

use crate::ReqwestClient;
pub struct Executor<T: Transaction> {
    pub state: T::State,
    pub client: Arc<RwLock<ReqwestClient>>
}

pub struct Task<T:Transaction> {
    pub state: watch::Receiver<T::State>,
    pub handle: JoinHandle<Result<(), crate::ClientError>>
}

pub trait Transaction: Sized + Sync + Send + 'static {
    type State;
    fn excute_on(self, client: Arc<ReqwestClient>) -> Task<Self>;
}