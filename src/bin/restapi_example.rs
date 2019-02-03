#![allow(unused_imports)]

extern crate apf;

#[macro_use]
extern crate log;
extern crate env_logger;

use actix_web::{http::Method, server::HttpServer, App, AsyncResponder, Error, Path, Result};
use futures::{
    future::{ok, result, Future, FutureResult},
    stream::once,
};

use apf::{prelude::*, service};

use std::{cell::Cell, thread::sleep, time::Duration};

fn main() {
    env_logger::init();

    trace!("starting up...");

    let service = Box::new(service::ExampleService);

    let config = ServiceApiConfig::new(vec![
        ApiServer::new(ApiAccess::Public, "127.0.0.1:8081".to_string()),
        ApiServer::new(ApiAccess::Private, "127.0.0.1:8082".to_string()),
    ]);

    api::start(ApiAggregator::new(vec![service]), config);
}
