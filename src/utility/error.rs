use hyper;
use hyper::http::uri;
use serde_json;
use std::{fmt, io, result};

#[derive(Debug)]
pub enum HentaiError {
    IOError(io::Error),
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
        HentaiError::IOError(err)
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

pub type Result<T> = result::Result<T, HentaiError>;
