#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

use serde::{ser::SerializeStruct, Deserialize, Serialize};

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


// Derived implementation falls back on some funky old tricks,
// due to the version of Rust `serde` uses,
// which I dislike, and would prefer to streamline.
impl<D, FD, Msg, ED> Serialize for RJSend<D, FD, Msg, ED>
where
    D: Serialize,
    FD: Serialize,
    Msg: AsRef<str>,
    ED: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Success { data } => {
                let mut state = serializer.serialize_struct("RJSend", 2)?;
                // JSend resresents the different response variants
                // via the `"status"` field, which is why during serialization,
                // we're serializing the name of the variant as a struct field
                // rather than serializing the enum normally...
                state.serialize_field("status", "success")?;
                state.serialize_field("data", data)?;
                state.end()
            }
            Self::Fail { data } => {
                let mut state = serializer.serialize_struct("RJSend", 2)?;
                // Simlarly to above, we need to serialize
                // the name of the variant as a struct field,
                // to comply with the JSend standard...
                state.serialize_field("status", "fail")?;
                state.serialize_field("data", data)?;
                state.end()
            }
            // This is the variant this custom implementation
            // pretty much exclusively exists for,
            // because I hate the way `serde` has to handle
            // the `skip_serializing_if` attribute...
            Self::Error {
                message,
                code,
                data,
            } => {
                // Casting `bool` values as `usize` is kind of a dumb approach,
                // but it's the most concise option in this case...
                let some_count = code.is_some() as usize + data.is_some() as usize;
                let mut state = serializer.serialize_struct("RJSend", 2 + some_count)?;

                state.serialize_field("status", "error")?;
                state.serialize_field("message", message.as_ref())?;

                match code {
                    // We can extract the contents using patten matching,
                    // rather than serializing the option directly in this case,
                    // because we want to skip serializing
                    // in the case of a `None` value,
                    // not encode the `None` state.
                    Some(code) => state.serialize_field("code", code)?,
                    None => state.skip_field("code")?,
                }

                match data {
                    // Similarly to above, we want to skip serialization
                    // of this field in the case a value is `None`,
                    // rather than encode that state...
                    Some(data) => state.serialize_field("data", data)?,
                    None => state.skip_field("data")?,
                }

                state.end()
            }
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
