#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "status")]
#[serde(rename_all = "lowercase")]
pub enum RJSend<D, FD, Msg = &'static str, ED = serde_json::Value> {
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

// Extractor methods
impl<D, FD, Msg, ED> RJSend<D, FD, Msg, ED> {
    #[inline]
    pub fn success(self) -> Option<D> {
        match self {
            Self::Success { data } => Some(data),
            _ => None,
        }
    }

    #[inline]
    pub fn fail(self) -> Option<FD> {
        match self {
            Self::Fail { data } => Some(data),
            _ => None,
        }
    }

    #[inline]
    pub fn error(self) -> Option<ErrorFields<Msg, ED>> {
        match self {
            Self::Error {
                message,
                code,
                data,
            } => Some(ErrorFields {
                message,
                code,
                data,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorFields<Msg, ED> {
    pub message: Msg,
    pub code: Option<serde_json::Number>,
    pub data: Option<ED>,
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!(
    "rjsend requires that either the `std` feature (default) or `alloc` feature is enabled"
);
