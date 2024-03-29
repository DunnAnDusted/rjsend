#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;
#[cfg(feature = "std")]
use std::error::Error;

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

// Constructors functions
impl<D, FD, Msg, ED> RJSend<D, FD, Msg, ED> {
    #[inline]
    pub const fn new_error(message: Msg) -> Self {
        Self::Error {
            message,
            code: None,
            data: None,
        }
    }

    #[inline]
    pub fn from_error_fields(
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

// `std` dependant contructor functions
#[cfg(feature = "std")]
impl<D, FD, ED> RJSend<D, FD, String, ED> {
    #[inline]
    pub fn from_error(data: ED) -> Self
    where
        ED: Error,
    {
        let message = data.to_string();

        Self::Error {
            message,
            code: None,
            data: Some(data),
        }
    }
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

    #[inline]
    pub fn unwrap_or(self, default: D) -> D {
        match self {
            Self::Success { data } => data,
            _ => default,
        }
    }

    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> D
    where
        F: FnOnce() -> D,
    {
        match self {
            Self::Success { data } => data,
            _ => f(),
        }
    }

    #[inline]
    #[allow(renamed_and_removed_lints)]
    #[allow(clippy::unwrap_or_else_default)]
    pub fn unwrap_or_default(self) -> D
    where
        D: Default,
    {
        // NOTE: We need to add a linter exception here,
        // because we are *not* using `std::option::Option`,
        // or `std::result::Result` here,
        // and actually *do* want to use `RJSend::unwrap_or_else` here,
        // because we're implementing `RJSend::unwrap_or_default` here... xD
        //
        // Also, `unwrap_or_else_default` was quite recently renamed,
        // making using the old name, and adding an exception to allow it,
        // the easiest solution, whilst retaining the current implementation...
        self.unwrap_or_else(Default::default)
    }
}

// Expect methods
impl<D, FD, Msg, ED> RJSend<D, FD, Msg, ED> {
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> D
    where
        FD: fmt::Debug,
        Msg: fmt::Debug,
        ED: fmt::Debug,
    {
        match self {
            Self::Success { data } => data,
            Self::Fail { data } => unwrap_failed(msg, &data),
            Self::Error {
                message,
                code,
                data,
            } => unwrap_failed(
                msg,
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
    pub fn expect_fail(self, msg: &str) -> FD
    where
        D: fmt::Debug,
        Msg: fmt::Debug,
        ED: fmt::Debug,
    {
        match self {
            Self::Fail { data } => data,
            Self::Success { data } => unwrap_failed(msg, &data),
            Self::Error {
                message,
                code,
                data,
            } => unwrap_failed(
                msg,
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
    pub fn expect_error(self, msg: &str) -> ErrorFields<Msg, ED>
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
            Self::Success { data } => unwrap_failed(msg, &data),
            Self::Fail { data } => unwrap_failed(msg, &data),
        }
    }
}

#[inline(never)]
#[cold]
#[track_caller]
fn unwrap_failed(msg: &str, error: &dyn fmt::Debug) -> ! {
    panic!("{}: {:?}", msg, error)
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

// Variant evaluation methods
impl<D, FD, Msg, ED> RJSend<D, FD, Msg, ED> {
    #[inline]
    #[must_use]
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    #[inline]
    #[must_use]
    pub fn is_success_and<F: FnOnce(D) -> bool>(self, f: F) -> bool {
        match self {
            Self::Success { data } => f(data),
            _ => false,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_fail(&self) -> bool {
        matches!(self, Self::Fail { .. })
    }

    #[inline]
    #[must_use]
    pub fn is_fail_and<F: FnOnce(FD) -> bool>(self, f: F) -> bool {
        match self {
            Self::Fail { data } => f(data),
            _ => false,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    #[inline]
    #[must_use]
    pub fn is_error_and<F: FnOnce(ErrorFields<Msg, ED>) -> bool>(self, f: F) -> bool {
        match self {
            Self::Error {
                message,
                code,
                data,
            } => f(ErrorFields {
                message,
                code,
                data,
            }),
            _ => false,
        }
    }
}

// Because `ErrorFields` is designed to map to `RJSend::Error`
// as directly as possible, it might be useful to have
// an implementation which maps directly back...
//
// This also means `ErrorFields` can be used as an ad hoc builder
// for the variant as well...
impl<D, FD, Msg, ED> From<ErrorFields<Msg, ED>> for RJSend<D, FD, Msg, ED> {
    fn from(fields: ErrorFields<Msg, ED>) -> Self {
        Self::from_error_fields(fields)
    }
}

#[cfg(feature = "std")]
impl<D, FD, ED> From<ED> for RJSend<D, FD, String, ED>
where
    ED: Error,
{
    fn from(data: ED) -> Self {
        Self::from_error(data)
    }
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
