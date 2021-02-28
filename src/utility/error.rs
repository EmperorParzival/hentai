use hyper::http::uri;
use std::{fmt, io, result};

/// Global error type for Hentai that catches all other types of errors that may occur.
/// This still provides the original error message when printed.
///
/// The first type (`IOError`) can only occur when using the `Hentai::from_json()` constructor.
/// Typically, the cause of this is an invalid file path being provided.
///
/// `HttpError` and `UriError` only occur when using `Hentai::new()`. `HttpError`s occur when
/// [hyper](https://hyper.rs) is unable to complete the request. `UriError`s are usually caused
/// by an invalid code being provided, thus resulting in an invalid URI.
///
/// `SerdeError` means that the JSON was invalid and it could not be deserialized.
#[derive(Debug)]
pub enum HentaiError {
    IoError(io::Error),
    HttpError(hyper::Error),
    UriError(uri::InvalidUri),
    SerdeError(serde_json::Error),
}

impl fmt::Display for HentaiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{:?}", &self))
    }
}

impl From<io::Error> for HentaiError {
    fn from(err: io::Error) -> Self {
        HentaiError::IoError(err)
    }
}

impl From<hyper::Error> for HentaiError {
    fn from(err: hyper::Error) -> Self {
        HentaiError::HttpError(err)
    }
}

impl From<uri::InvalidUri> for HentaiError {
    fn from(err: uri::InvalidUri) -> Self {
        HentaiError::UriError(err)
    }
}

impl From<serde_json::Error> for HentaiError {
    fn from(err: serde_json::Error) -> Self {
        HentaiError::SerdeError(err)
    }
}

/// Shorthand for Result<T, HentaiError> meant to make catching errors slightly more simple.
pub type Result<T> = result::Result<T, HentaiError>;
