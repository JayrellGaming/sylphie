#![allow(non_camel_case_types)]

use std::error;
use std::fmt;

mod internal {
    use *;
    error_chain! {
        foreign_links {
            Fmt(std::fmt::Error);
            Io(std::io::Error);
            ParseIntError(std::num::ParseIntError);
            R2D2(r2d2::Error);
            Reqwest(reqwest::Error);
            Rusqlite(rusqlite::Error);
            Rusqlite_FromSqlError(rusqlite::types::FromSqlError);
            SerdeJson(serde_json::Error);
            Serenity(serenity::Error);
            Str_Utf8Error(std::str::Utf8Error);
            String_FromUtf8Error(std::string::FromUtf8Error);
            SystemTimeError(std::time::SystemTimeError);
        }

        errors {
            CommandError(err: String) {
                description("command encountered an error")
                display("{}", err)
            }

            LZ4Error {
                description("LZ4 error")
            }

            Panicked {
                description("panic encountered")
            }
        }
    }
}
// Reexport these types so IDEs pick up on them correctly.
pub use self::internal::{Error, ErrorKind, Result, ResultExt};

#[derive(Debug)]
struct StringError(String);
impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl error::Error for StringError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl Error {
    pub fn to_sync_error(&self) -> impl error::Error + Send + Sync + 'static {
        StringError(format!("{}", self))
    }
}

macro_rules! cmd_error {
    ($err:expr $(,)*) => {
        bail!($crate::errors::ErrorKind::CommandError(format!("{}", $err)))
    };
    ($err:expr, $($arg:expr),* $(,)*) => {
        bail!($crate::errors::ErrorKind::CommandError(format!($err, $($arg,)*)))
    };
}
macro_rules! cmd_ensure {
    ($cond:expr, $err:expr $(,)*) => {
        ensure!($cond, $crate::errors::ErrorKind::CommandError(format!("{}", $err)))
    };
    ($cond:expr, $err:expr, $($arg:expr),* $(,)*) => {
        ensure!($cond, $crate::errors::ErrorKind::CommandError(format!($err, $($arg,)*)))
    };
}

pub trait ResultCmdExt<T> {
    fn cmd_ok(self) -> Result<()>;
}
impl <T> ResultCmdExt<T> for Result<T> {
    fn cmd_ok(self) -> Result<()> {
        match self {
            Ok(_) | Err(Error(box (ErrorKind::CommandError(_), _))) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

pub trait IntoResultCmdExt<T> {
    fn to_cmd_err<F, R: Into<String>>(self, f: F) -> Result<T> where F: FnOnce() -> R;
}
impl <T, E: ResultExt<T>> IntoResultCmdExt<T> for E {
    fn to_cmd_err<F, R: Into<String>>(self, f: F) -> Result<T> where F: FnOnce() -> R {
        self.chain_err(|| ErrorKind::CommandError(f().into()))
    }
}