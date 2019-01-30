#![allow(unused_imports)]

extern crate payment;

use actix_web::server::HttpServer;
use actix_web::*;
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

fn index(req: &HttpRequest<LocalAppState>) -> String {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count);

    format!("request number: {}", count)
}

fn index_async(
    req: &HttpRequest<LocalAppState>,
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count);
    result(Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<h1>request counter: {}</h1>", count))))
    .responder()
}

fn user_path(req: &HttpRequest<LocalAppState>, info: Path<(u32, String)>) -> Result<String> {
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

    fn user_path(state: &AppState, info: Path<(u32, String)>) -> ApiResult<String> {
        Ok(format!("Welcome {}! {}", info.1, info.0))
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/info", Self::info)
            .endpoint_req("v1/info", Self::info_req)
            .endpoint_req_mut("v2/update", Self::update);

        // .endpoint("v1/user/{number}/{name}", Self::user_path);
    }
}

fn main() {
    let my_service = Box::new(MyService);

    let agg = ApiAggregator::new(vec![my_service]);

    // let api_handlers =


    api::start(&agg);

    // server::new(|| {
    //     App::with_state(LocalAppState {
    //         counter: Cell::new(0),
    //     })
    //     .resource("/index.html", |r| r.f(index))
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
