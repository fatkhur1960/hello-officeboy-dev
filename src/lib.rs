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
extern crate diesel;
extern crate futures;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod api;
mod db;
pub mod service;
pub mod web;
