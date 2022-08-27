pub mod live;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct CommonResp<T> {
    pub code: i32,
    pub message: Option<String>,
    pub data: T
}