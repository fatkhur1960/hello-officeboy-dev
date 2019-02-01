//! Ansvia Payment Framework Library.
//!
//! Ini merupakan payment framework dalam bentuk library yang bisa digunakan untuk
//! mem-build backend payment.
//!
#![deny(missing_docs)]
#![allow(unused_imports, unused_variables, dead_code)]

extern crate actix;
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

pub mod api;
mod db;
pub mod error;
pub mod models;
mod result;
mod schema;
pub mod schema_op;
pub mod service;
pub mod web;

/// Common use (prelude) exports.
pub mod prelude {
    pub use super::api::{
        self, ApiAccess, ApiAggregator, ApiBuilder, ApiServer, AppState, ServiceApiBuilder,
        ServiceApiConfig, ServiceApiScope,
    };
    pub use super::result::Result;
    pub use super::{
        // models::Account,
        schema_op::Schema,
        schema_op::ID,
        service::{PaymentService, Service},
    };
}
