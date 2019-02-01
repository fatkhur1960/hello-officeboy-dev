#![allow(unused_imports)]

extern crate payment;

#[macro_use]
extern crate log;
extern crate env_logger;

use actix_web::server::HttpServer;
use actix_web::{http::Method, App, AsyncResponder, Error, Path, Result};
use futures::{
    future::{ok, result, Future, FutureResult},
    stream::once,
};

use payment::prelude::*;
use payment::service;

use std::{cell::Cell, thread::sleep, time::Duration};

// struct LocalAppState {
//     counter: Cell<usize>,
// }

// fn index(req: &actix_web::HttpRequest<LocalAppState>) -> String {
//     let count = req.state().counter.get() + 1;
//     req.state().counter.set(count);

//     format!("request number: {}", count)
// }

// fn index_async(
//     req: &actix_web::HttpRequest<LocalAppState>,
// ) -> Box<Future<Item = actix_web::HttpResponse, Error = Error>> {
//     let count = req.state().counter.get() + 1;
//     req.state().counter.set(count);
//     result(Ok(actix_web::HttpResponse::Ok()
//         .content_type("text/html")
//         .body(format!("<h1>request counter: {}</h1>", count))))
//     .responder()
// }

// fn user_path(
//     req: &actix_web::HttpRequest<LocalAppState>,
//     info: Path<(u32, String)>,
// ) -> Result<String> {
//     Ok(format!("Welcome {}! {}", info.1, info.0))
// }

fn main() {
    env_logger::init();

    trace!("starting up...");

    let service = Box::new(service::ExampleService);

    let config = ServiceApiConfig::new(vec![
        ApiServer::new(ApiAccess::Public, "127.0.0.1:8081".to_string()),
        ApiServer::new(ApiAccess::Private, "127.0.0.1:8082".to_string()),
    ]);

    api::start(ApiAggregator::new(vec![service]), config);

    // server::new(|| {
    // App::with_state(LocalAppState {
    //     counter: Cell::new(0),
    // })
    // .resource("/index.html", |r| r.f(index));
    //     .resource("/index_async.html", |r| r.f(index_async))
    //     // .resource("/user/{userid}/{name}.html", |r| {
    //     //     r.method(http::Method::GET).with(user_path)
    //     //     // r.f(user_path)
    //     // })
    //     .finish()
    // })
    // .bind("127.0.0.1:8080")
    // .unwrap()
    // .run();
}
