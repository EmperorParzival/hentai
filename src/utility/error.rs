use hyper::{header, http::uri};
use std::{fmt, io, num, result};

/// Global error type for Hentai that catches all other types of errors that may occur.
/// This still provides the original error message when printed.
///
/// The first type (`IOError`) can only occur when using the `Hentai::from_json()` constructor.
/// Typically, the cause of this is an invalid file path being provided.
///
/// `BaseError` is a standard error used to indicate that an Option<> failed to be converted into
/// a Result<>. This occurs in the `Hentai::random()` constructor after the request is sent but
/// the redirect header cannot be converted into a string.
///
/// `HttpError` and `UriError` only occur when using `Hentai::new()`. `HttpError`s occur when
/// [hyper](https://hyper.rs) is unable to complete the request. `UriError`s are usually caused
/// by an invalid code being provided, thus resulting in an invalid URI.
///
/// `SerdeError` means that the JSON was invalid and it could not be deserialized.
///
/// `ToStrError` happens when using `Hentai::random()` and the redirect header cannot be converted
/// into a random doujin URL.
///
/// `ConversionError` happens when the response from nhentai's [/random](https://nhentai.net/random)
/// endpoint returns a URL from which the six-digit code could not be converted into an integer.
/// The cause of this is currently unknown, but please crate an issue on GitHub if you know how to
/// fix this.
#[derive(Debug)]
pub enum HentaiError {
    IoError(io::Error),
    BaseError(&'static str),
    HttpError(hyper::Error),
    UriError(uri::InvalidUri),
    SerdeError(serde_json::Error),
    ToStrError(header::ToStrError),
    ConversionError(num::ParseIntError),
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

impl From<&'static str> for HentaiError {
    fn from(err: &'static str) -> Self {
        HentaiError::BaseError(err)
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

impl From<header::ToStrError> for HentaiError {
    fn from(err: header::ToStrError) -> Self {
        HentaiError::ToStrError(err)
    }
}

impl From<num::ParseIntError> for HentaiError {
    fn from(err: num::ParseIntError) -> Self {
        HentaiError::ConversionError(err)
    }
}

/// Shorthand for Result<T, HentaiError> meant to make catching errors slightly more simple.
pub type Result<T> = result::Result<T, HentaiError>;
