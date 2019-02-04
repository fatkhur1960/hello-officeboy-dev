//! Ansvia Payment Framework Library.
//!
//! Library untuk membuat sistem payment.
//!
//! # Fitur
//!
//! * Service & Http rest API management.
//! * Authentication & Authorization.
//! * Multi service implementation.
//!
#![deny(missing_docs)]
#![allow(unused_imports, unused_variables, dead_code)]
#![allow(clippy::new_without_default)]

// extern crate actix;
extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate futures;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate env_logger;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate regex;

#[macro_use]
extern crate apf_proc_macro;

extern crate byteorder;
extern crate hex;
extern crate rsnowflake;
pub(crate) extern crate sodiumoxide;

pub(crate) use sodiumoxide as crypto_impl;

#[macro_use]
mod macros;
pub mod api;
pub mod auth;
pub mod crypto;
mod db;
pub mod error;
pub mod models;
mod protos;
mod result;
mod schema;
pub mod schema_op;
pub mod service;
pub mod token;
pub mod tx;
pub(crate) mod util;
mod valid;
pub mod web;

/// Common use (prelude) exports.
pub mod prelude {
    pub use super::{
        api::{
            self, ApiAccess, ApiAggregator, ApiBuilder, ApiServer, AppState, ServiceApiBuilder,
            ServiceApiConfig, ServiceApiScope,
        },
        result::Result,
        schema_op::{Schema, ID},
        service::{PaymentService, Service},
        valid::Validable,
    };
}
