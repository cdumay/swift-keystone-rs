#![deny(warnings)]
extern crate chrono;
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

pub mod authv1;
pub mod authv2;
pub mod error;

type Result<T> = std::result::Result<T, error::Error>;
