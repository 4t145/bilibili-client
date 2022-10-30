use std::collections::HashMap;
use std::sync::RwLock;
use expire::MaybeExpired;
use http_api_util::{cache::{FifoCache, ApiCache}, Api};

use crate::api::user::info::UserInfo;


pub struct Cache {
    pub user_info: RwLock<FifoCache<<UserInfo as Api>::Request, MaybeExpired<<UserInfo as Api>::Response>>>
}

