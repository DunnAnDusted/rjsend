#![no_std]

extern crate alloc;

use core::fmt;

use alloc::string::String;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ErrorData<Msg = String, ED = serde_json::Value> {
    #[serde(bound(deserialize = "Msg: fmt::Display + Deserialize<'de>"))]
    pub message: Msg,
    pub code: Option<serde_json::Number>,
    pub data: Option<ED>,
}
