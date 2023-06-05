#![no_std]

extern crate alloc;

use core::{fmt, marker::PhantomData};

use alloc::string::String;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status")]
#[serde(rename_all = "lowercase")]
pub enum RJSend<D, FD, Msg = String, ED = serde_json::Value> {
    Success {
        data: D,
    },
    Fail {
        data: FD,
    },
    #[serde(bound(
        deserialize = "ErrorData<Msg, ED>: Deserialize<'de>",
        serialize = "ErrorData<Msg, ED>: Serialize"
    ))]
    Error(ErrorData<Msg, ED>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ErrorData<Msg = String, ED = serde_json::Value> {
    #[serde(bound(deserialize = "Msg: fmt::Display + Deserialize<'de>"))]
    pub message: Msg,
    pub code: Option<serde_json::Number>,
    pub data: Option<ED>,
}

// Honestly, the derived impl for `Serialize` is a complete mess for `ErrorData`
// so I prefered to write my own implementation...
impl<Msg, ED> Serialize for ErrorData<Msg, ED>
where
    Msg: AsRef<str>,
    ED: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self {
            message,
            code,
            data,
        } = self;

        // By all accounts, casting `bool`s to `usize`s is a wonky approach,
        // but it's the most concise option in this case...
        let some_count = code.is_some() as usize + data.is_some() as usize;
        let mut state = serializer.serialize_struct("ErrorData", 1 + some_count)?;

        state.serialize_field("message", message.as_ref())?;

        match code {
            // Because we want to skip the `code` field entirely
            // if its value is `None`, we can avoid the overhead
            // of serializing the `Option`,
            // and instead, serialize just the inner value.
            Some(code) => state.serialize_field("code", code),
            None => state.skip_field("code"),
        }?;

        match data {
            // Similarly to above, we want to skip the `data` field entirely,
            // if its value is `None`.
            Some(data) => state.serialize_field("data", data),
            None => state.skip_field("data"),
        }?;

        state.end()
    }
}

#[derive(Debug, PartialEq)]
pub struct ErrorBuilder<D, FD, Msg, ED> {
    inner: ErrorData<Msg, ED>,
    others: PhantomData<(D, FD)>,
}
