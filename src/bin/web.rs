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

use payment::api::{
    self, ApiAccess, ApiAggregator, AppState, Result as ApiResult, ServiceApiBackend,
    ServiceApiBuilder,
};
use payment::service;

use std::{cell::Cell, thread::sleep, time::Duration};

struct LocalAppState {
    counter: Cell<usize>,
}

fn index(req: &actix_web::HttpRequest<LocalAppState>) -> String {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count);

    format!("request number: {}", count)
}

fn index_async(
    req: &actix_web::HttpRequest<LocalAppState>,
) -> Box<Future<Item = actix_web::HttpResponse, Error = Error>> {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count);
    result(Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<h1>request counter: {}</h1>", count))))
    .responder()
}

fn user_path(
    req: &actix_web::HttpRequest<LocalAppState>,
    info: Path<(u32, String)>,
) -> Result<String> {
    Ok(format!("Welcome {}! {}", info.1, info.0))
}

struct MyService;

impl service::Service for MyService {
    fn name(&self) -> &'static str {
        "doi"
    }
    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        PublicApi::wire(builder);
    }
}

struct PublicApi {}

impl PublicApi {
    pub fn info(state: &AppState, query: ()) -> ApiResult<String> {
        Ok(concat!("version: ", env!("CARGO_PKG_VERSION")).to_owned())
    }

    pub fn info_req(state: &AppState, query: (), req: &api::HttpRequest) -> ApiResult<String> {
        Ok(concat!("version: ", env!("CARGO_PKG_VERSION")).to_owned())
    }

    pub fn update(state: &AppState, query: (), req: &api::HttpRequest) -> ApiResult<String> {
        Ok("".to_owned())
    }

    fn user_path(info: Path<(u32, String)>) -> ApiResult<String> {
        Ok(format!("Welcome {}! {}", info.1, info.0))
    }

    fn resource_test(req: &api::HttpRequest) -> Result<String> {
        Ok("resource_test".to_owned())
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        trace!("wiring API...");
        builder
            .public_scope()
            // .endpoint("v1/info", Self::info)
            .endpoint_req("v1/info", Self::info_req)
            .endpoint_req_mut("v1/update", Self::update)
            .resource(|scope| {
                scope.resource("v1/coba", |r| r.method(Method::GET).h(Self::resource_test))
                    .resource("v1/coba2/{userid}/{username}", |r| r.method(Method::GET).with(Self::user_path))
            });

        // .endpoint("v1/user/{number}/{name}", Self::user_path);
    }
}

fn main() {
    env_logger::init();

    trace!("starting up...");

    let my_service = Box::new(MyService);

    // let agg = ;

    // let api_handlers =

    api::start(ApiAggregator::new(vec![my_service]));

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
