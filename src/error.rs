use url;
use std::{fmt, io};
use hyper;
use serde_json;
use hyper_native_tls;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    InvalidURL(url::ParseError),
    HttpError(hyper::Error),
    IOError(io::Error),
    JSONError(serde_json::Error),
    TLSError(hyper_native_tls::native_tls::Error)
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Error { Error::InvalidURL(error) }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Error {
        Error::HttpError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error { Error::IOError(error) }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error { Error::JSONError(error) }
}

impl From<hyper_native_tls::native_tls::Error> for Error {
    fn from(error: hyper_native_tls::native_tls::Error) -> Error { Error::TLSError(error) }
}

impl From<String> for Error {
    fn from(error: String) -> Error { Error::Generic(error) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidURL(ref e) => e.fmt(f),
            Error::Generic(ref e) => e.fmt(f),
            Error::HttpError(ref e) => e.fmt(f),
            Error::IOError(ref e) => e.fmt(f),
            Error::JSONError(ref e) => e.fmt(f),
            Error::TLSError(ref e) => e.fmt(f),
        }
    }
}