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

// Unwrapping methods
impl<D, FD, Msg, ED> RJSend<D, FD, Msg, ED> {
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> D
    where
        FD: fmt::Debug,
        Msg: fmt::Debug,
        ED: fmt::Debug,
    {
        match self {
            Self::Success { data } => data,
            Self::Fail { data } => {
                unwrap_failed("called `RJSend::unwrap()` on a `Fail` value", &data)
            }
            Self::Error {
                message,
                code,
                data,
            } => unwrap_failed(
                "called `RJSend::unwrap()` on an `Error` value",
                &ErrorFields {
                    message,
                    code,
                    data,
                },
            ),
        }
    }

    #[inline]
    #[track_caller]
    pub fn unwrap_fail(self) -> FD
    where
        D: fmt::Debug,
        Msg: fmt::Debug,
        ED: fmt::Debug,
    {
        match self {
            Self::Fail { data } => data,
            Self::Success { data } => {
                unwrap_failed("called `RJSend::unwrap_fail()` on a `Success` value", &data)
            }
            Self::Error {
                message,
                code,
                data,
            } => unwrap_failed(
                "called `RJSend::unwrap_fail` on an `Error` value",
                &ErrorFields {
                    message,
                    code,
                    data,
                },
            ),
        }
    }

    #[inline]
    #[track_caller]
    pub fn unwrap_error(self) -> ErrorFields<Msg, ED>
    where
        D: fmt::Debug,
        FD: fmt::Debug,
    {
        match self {
            Self::Error {
                message,
                code,
                data,
            } => ErrorFields {
                message,
                code,
                data,
            },
            Self::Success { data } => unwrap_failed(
                "called `RJSend::unwrap_error()` on a `Success` value",
                &data,
            ),
            Self::Fail { data } => {
                unwrap_failed("called `RJSend::unwrap_error()` on a `Fail` value", &data)
            }
        }
    }
}

#[inline(never)]
#[cold]
#[track_caller]
fn unwrap_failed(msg: &str, error: &dyn fmt::Debug) -> ! {
    panic!("{}: {:?}", msg, error)
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
