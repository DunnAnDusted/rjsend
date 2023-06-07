#![no_std]

extern crate alloc;

use core::fmt;

use alloc::string::String;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "status")]
#[serde(rename_all = "lowercase")]
pub enum RJSend<D, FD, Msg = String, ED = serde_json::Value> {
    Success {
        data: D,
    },
    Fail {
        data: FD,
    },
    Error {
        #[serde(bound(deserialize = "Msg: fmt::Display + Deserialize<'de>",))]
        message: Msg,
        code: Option<serde_json::Number>,
        data: Option<ED>,
    },
}

// Constructors functions
impl<D, FD, Msg, ED> RJSend<D, FD, Msg, ED> {
    #[inline]
    pub const fn new_success(data: D) -> Self {
        Self::Success { data }
    }

    #[inline]
    pub const fn new_fail(data: FD) -> Self {
        Self::Fail { data }
    }

    #[inline]
    pub const fn new_error(message: Msg) -> Self {
        Self::Error {
            message,
            code: None,
            data: None,
        }
    }

    #[inline]
    pub fn from_fields(
        ErrorFields {
            message,
            code,
            data,
        }: ErrorFields<Msg, ED>,
    ) -> Self {
        Self::Error {
            message,
            code,
            data,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorFields<Msg, ED> {
    pub message: Msg,
    pub code: Option<serde_json::Number>,
    pub data: Option<ED>,
}
