#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(async_fn_in_trait)]
mod consts;
pub(crate) mod client;
pub mod model;
pub mod api;
pub mod business;
// pub mod logger;

// re-export
pub use client::*;
pub use reqwest;
// pub mod transaction;
#[cfg(features = "grpc")]
#[allow(clippy::all, non_snake_case)]
/// 这里的代码由tonic自动生成
pub mod grpc;

pub mod utils;