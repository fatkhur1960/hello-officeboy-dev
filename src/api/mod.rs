//! Module yang berkaitan dengan kebutuhan API

use actix_web::{
    actix::System, server, AsyncResponder, FromRequest, HttpMessage, HttpResponse, Query,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use futures::future::{Future, IntoFuture};

mod error;
mod with;

pub use self::with::Result;
use self::with::{Immutable, ImmutableReq, Mutable, MutableReq, NamedWith, With};

use crate::db;
use crate::service::Service;

use std::{collections::BTreeMap, convert::From, env, fmt, marker::PhantomData, sync::Arc};

/// Jenis API access, kita bagikan menjadi 2 macam:
///     1. Public
///     2. Private
///
/// Public adalah apabila kita ingin akses API-nya boleh digunakan oleh publik.
/// Sementara Private adalah apabila kita ingin akses API-nya hanya untuk internal,
/// nantinya masing-masing akses ini di-serve pada port yang berbeda
/// sehingga perlu dilakukan settingan firewall oleh system administrator
/// agar port untuk private API hanya boleh diakses dari jaringan internal.
pub enum ApiAccess {
    /// Akses publik
    Public,

    /// Akses privat
    Private,
}

impl fmt::Display for ApiAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiAccess::Public => f.write_str("public"),
            ApiAccess::Private => f.write_str("private"),
        }
    }
}

use serde::{de::DeserializeOwned, Serialize};

#[doc(hidden)]
pub struct QueryForm<T> {
    inner: T,
}

impl<T> From<T> for QueryForm<T>
where
    T: DeserializeOwned + 'static,
{
    fn from(d: T) -> Self {
        QueryForm { inner: d }
    }
}

/// Defines an object that could be used as an API backend.
///
/// This trait is used to implement an API backend for Exonum.
pub trait ServiceApiBackend: Sized {
    /// Concrete endpoint handler in the backend.
    type Handler;
    /// Concrete backend API builder.
    type Backend;

    /// Adds the given endpoint handler to the backend.
    fn endpoint<N, Q, I, R, F, E, K>(&mut self, name: N, endpoint: E) -> &mut Self
    where
        N: Into<String>,
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        // F: for<'r> Fn(&'r AppState, Q) -> R + 'static + Clone,
        // F: Into<FuncHandler<Func2<Q, R>>>,
        E: Into<With<Q, I, R, F>>,
        Self::Handler: From<NamedWith<Q, I, R, F, K>>,
    {
        let named_with = NamedWith::new(name, endpoint);
        self.raw_handler(Self::Handler::from(named_with))
    }

    /// Adds the given mutable endpoint handler to the backend.
    fn endpoint_mut<N, Q, I, R, F, E, K>(&mut self, name: N, endpoint: E) -> &mut Self
    where
        N: Into<String>,
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        // F: for<'r> Fn(&'r AppState, Q) -> R + 'static + Clone,
        E: Into<With<Q, I, R, F>>,
        Self::Handler: From<NamedWith<Q, I, R, F, K>>,
    {
        let named_with = NamedWith::new(name, endpoint);
        self.raw_handler(Self::Handler::from(named_with))
    }

    /// Adds the raw endpoint handler for the given backend.
    fn raw_handler(&mut self, handler: Self::Handler) -> &mut Self;

    /// Binds API handlers to the given backend.
    fn wire(&self, output: Self::Backend) -> Self::Backend;
}

/// Type alias for the concrete `actix-web` HTTP response.
pub type FutureResponse = actix_web::FutureResponse<HttpResponse, actix_web::Error>;
/// Type alias for the concrete `actix-web` HTTP request.
pub type HttpRequest = actix_web::HttpRequest<AppState>;
/// Type alias for the inner `actix-web` HTTP requests handler.
pub type RawHandler = dyn Fn(HttpRequest) -> FutureResponse + 'static + Send + Sync;
/// Type alias for the `actix-web::App` with the `AppState`.
pub type App = actix_web::App<AppState>;
/// Type alias for actix `Scope` with `AppState`.
pub type Scope = actix_web::Scope<AppState>;
/// Type alias for the `actix-web::App` configuration.
pub type AppConfig = Arc<dyn Fn(App) -> App + 'static + Send + Sync>;

/// Raw `actix-web` backend requests handler.
#[derive(Clone)]
pub struct RequestHandler {
    /// Endpoint name.
    pub name: String,
    /// Endpoint HTTP method.
    pub method: actix_web::http::Method,
    /// Inner handler.
    pub inner: Arc<RawHandler>,
}

impl fmt::Debug for RequestHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RequestHandler")
            .field("name", &self.name)
            .field("method", &self.method)
            .finish()
    }
}

/// API builder for the `actix-web` backend.
#[derive(Debug, Clone, Default)]
pub struct ApiBuilder {
    handlers: Vec<RequestHandler>,
}

impl ApiBuilder {
    /// Constructs a new backend builder instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ServiceApiBackend for ApiBuilder {
    type Handler = RequestHandler;
    type Backend = actix_web::Scope<AppState>;

    fn raw_handler(&mut self, handler: Self::Handler) -> &mut Self {
        self.handlers.push(handler);
        self
    }

    fn wire(&self, mut output: Self::Backend) -> Self::Backend {
        for handler in self.handlers.clone() {
            let inner = handler.inner;
            output = output.route(&handler.name, handler.method.clone(), move |request| {
                inner(request)
            });
        }
        output
    }
}

impl<Q, I, F> From<NamedWith<Q, I, Result<I>, F, Immutable>> for RequestHandler
where
    F: for<'r> Fn(&'r AppState, Q) -> Result<I> + 'static + Send + Sync + Clone,
    // F: Into<FuncHandler<Func2<Q, Result<I>>>>,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedWith<Q, I, Result<I>, F, Immutable>) -> Self {
        // let handler = f.inner.handler.into().inner;
        let handler = f.inner.handler;
        let index = move |request: HttpRequest| -> FutureResponse {
            let context = request.state();
            let future = Query::from_request(&request, &Default::default())
                .map(|query: Query<Q>| query.into_inner())
                .and_then(|query| handler(context, query).map_err(From::from))
                .and_then(|value| Ok(HttpResponse::Ok().json(value)))
                .into_future();
            Box::new(future)
        };

        Self {
            name: f.name,
            method: actix_web::http::Method::GET,
            inner: Arc::from(index) as Arc<RawHandler>,
        }
    }
}

impl<Q, I, F> From<NamedWith<Q, I, Result<I>, F, ImmutableReq>> for RequestHandler
where
    F: for<'r> Fn(&'r AppState, Q, &HttpRequest) -> Result<I> + 'static + Send + Sync + Clone,
    // F: Into<FuncHandler<Func2<Q, Result<I>>>>,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedWith<Q, I, Result<I>, F, ImmutableReq>) -> Self {
        // let handler = f.inner.handler.into().inner;
        let handler = f.inner.handler;
        let index = move |request: HttpRequest| -> FutureResponse {
            let context = request.state();
            let future = Query::from_request(&request, &Default::default())
                .map(|query: Query<Q>| query.into_inner())
                .and_then(|query| handler(context, query, &request).map_err(From::from))
                .and_then(|value| Ok(HttpResponse::Ok().json(value)))
                .into_future();
            Box::new(future)
        };

        Self {
            name: f.name,
            method: actix_web::http::Method::GET,
            inner: Arc::from(index) as Arc<RawHandler>,
        }
    }
}

impl<Q, I, F> From<NamedWith<Q, I, Result<I>, F, Mutable>> for RequestHandler
where
    F: for<'r> Fn(&'r AppState, Q) -> Result<I> + 'static + Send + Sync + Clone,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedWith<Q, I, Result<I>, F, Mutable>) -> Self {
        let handler = f.inner.handler;
        let index = move |request: HttpRequest| -> FutureResponse {
            let handler = handler.clone();
            let context = request.state().clone();
            request
                .json()
                .from_err()
                .and_then(move |query: Q| {
                    handler(&context, query)
                        .map(|value| HttpResponse::Ok().json(value))
                        .map_err(From::from)
                })
                .responder()
        };

        Self {
            name: f.name,
            method: actix_web::http::Method::POST,
            inner: Arc::from(index) as Arc<RawHandler>,
        }
    }
}

impl<Q, I, F> From<NamedWith<Q, I, Result<I>, F, MutableReq>> for RequestHandler
where
    F: for<'r> Fn(&'r AppState, Q, &HttpRequest) -> Result<I> + 'static + Send + Sync + Clone,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedWith<Q, I, Result<I>, F, MutableReq>) -> Self {
        let handler = f.inner.handler;
        let index = move |request: HttpRequest| -> FutureResponse {
            let handler = handler.clone();
            let context = request.state().clone();

            request
                .json()
                .from_err()
                .and_then(move |query: Q| {
                    handler(&context, query, &request)
                        .map(|value| HttpResponse::Ok().json(value))
                        .map_err(From::from)
                })
                .responder()
        };

        Self {
            name: f.name,
            method: actix_web::http::Method::POST,
            inner: Arc::from(index) as Arc<RawHandler>,
        }
    }
}

// /// Function Handler
// pub struct FuncHandler<F> {
//     inner: F,
//     // _query: PhantomData<Q>,
//     // _resp: PhantomData<R>,
//     // _z: PhantomData<Z>,
// }

// /// Function handler
// impl<T, Q, R, Z> FuncHandler<T, Q, R, Z>
// where
//     T: for<'r> Fn(&'r AppState, Q) -> R + 'static + Clone,
//     Q: Into<QueryForm<Z>>,
// {
//     /// Get function handler
//     fn get(&self) -> &T {
//         &self.inner
//     }
// }

use self::error::Error;

// type Func1<Q, R> = fn(&AppState, Q) -> R;

// type Func2<Q, R> = fn(&AppState, Q, &HttpRequest) -> R;

// // impl<Q, R> From<Func2<Q, R>> for FuncHandler<Func2<Q, R>>
// // where
// //     Q: DeserializeOwned + 'static,
// // {
// //     fn from(f: Func2<Q, R>) -> Self {
// //         FuncHandler {
// //             inner: f,
// //             // _query: PhantomData,
// //             // _resp: PhantomData,
// //         }
// //     }
// // }

// impl<Q, R> From<Func1<Q, R>> for FuncHandler<Func1<Q, R>> {
//     fn from(f: Func1<Q, R>) -> Self {
//         FuncHandler {
//             inner: f,
//             // _query: PhantomData,
//             // _resp: PhantomData,
//         }
//     }
// }

// impl<Q, R> From<Func2<Q, R>> for FuncHandler<Func2<Q, R>> {
//     fn from(f: Func2<Q, R>) -> Self {
//         FuncHandler {
//             inner: f,
//             // _query: PhantomData,
//             // _resp: PhantomData,
//         }
//     }
// }

/// Scope API
#[derive(Default, Clone)]
pub struct ServiceApiScope {
    pub(crate) actix_backend: ApiBuilder,
    pub(crate) resources: Vec<Arc<Box<Fn(Scope) -> Scope + Sync + Send + 'static>>>,
}

impl ServiceApiScope {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self::default()
    }

    fn endpoint_internal<Q, I, R, F, E, K>(&mut self, name: &'static str, endpoint: E) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        // F: Into<FuncHandler<Func2<Q, R>>>,
        E: Into<With<Q, I, R, F>>,
        RequestHandler: From<NamedWith<Q, I, R, F, K>>,
    {
        self.actix_backend.endpoint(name, endpoint);
        self
    }

    /// Adds the given endpoint handler to the API scope. These endpoints
    /// are designed for reading operations.
    ///
    /// For now there is only web backend and it has the following requirements:
    ///
    /// - Query parameters should be decodable via `serde_urlencoded`, i.e. from the
    ///   "first_param=value1&second_param=value2" form.
    /// - Response items should be encodable via `serde_json` crate.
    pub fn endpoint<Q, I, R, F, E>(&mut self, name: &'static str, endpoint: E) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        // F: Into<FuncHandler<Func2<Q, R>>>,
        E: Into<With<Q, I, R, F>>,
        RequestHandler: From<NamedWith<Q, I, R, F, Immutable>>,
    {
        self.endpoint_internal(name, endpoint);
        self
    }

    /// Add endpoint with additional `HttpRequest` object as third parameter.
    pub fn endpoint_req<Q, I, R, F, E>(&mut self, name: &'static str, endpoint: E) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        E: Into<With<Q, I, R, F>>,
        RequestHandler: From<NamedWith<Q, I, R, F, ImmutableReq>>,
    {
        self.endpoint_internal(name, endpoint);
        self
    }

    fn endpoint_internal_mut<Q, I, R, F, E, K>(
        &mut self,
        name: &'static str,
        endpoint: E,
    ) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        // F: for<'r> Fn(&'r AppState, Q) -> R + 'static + Clone,
        E: Into<With<Q, I, R, F>>,
        RequestHandler: From<NamedWith<Q, I, R, F, K>>,
    {
        self.actix_backend.endpoint_mut(name, endpoint);
        self
    }

    /// Adds the given mutable endpoint handler to the API scope. These endpoints
    /// are designed for modification operations.
    ///
    /// For now there is only web backend and it has the following requirements:
    ///
    /// - Query parameters should be decodable via `serde_json`.
    /// - Response items also should be encodable via `serde_json` crate.
    pub fn endpoint_mut<Q, I, R, F, E>(&mut self, name: &'static str, endpoint: E) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        F: for<'r> Fn(&'r AppState, Q) -> R + 'static + Clone,
        E: Into<With<Q, I, R, F>>,
        RequestHandler: From<NamedWith<Q, I, R, F, Mutable>>,
    {
        self.endpoint_internal_mut(name, endpoint);
        self
    }

    /// Adds the given mutable endpoint handler to the API scope. These endpoints
    /// are designed for modification operations.
    ///
    /// With additional `HttpRequest` as third parameter.
    ///
    /// For now there is only web backend and it has the following requirements:
    ///
    /// - Query parameters should be decodable via `serde_json`.
    /// - Response items also should be encodable via `serde_json` crate.
    pub fn endpoint_req_mut<Q, I, R, F, E>(&mut self, name: &'static str, endpoint: E) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        F: for<'r> Fn(&'r AppState, Q, &HttpRequest) -> R + 'static + Clone,
        E: Into<With<Q, I, R, F>>,
        RequestHandler: From<NamedWith<Q, I, R, F, MutableReq>>,
    {
        self.endpoint_internal_mut(name, endpoint);
        self
    }

    /// Add raw Actix web resource
    pub fn resource<'a, F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(Scope) -> Scope + Sync + Send + 'static,
    {
        {
            self.resources.push(Arc::new(Box::new(f)));
        }
        self
    }

    /// Returns a mutable reference to the underlying web backend.
    pub fn web_backend(&mut self) -> &mut ApiBuilder {
        &mut self.actix_backend
    }
}

/// API builder untuk build endpoint berdasarkan scope aksesnya
/// lihat juga [[ApiAccess]]
#[derive(Default, Clone)]
pub struct ServiceApiBuilder {
    public_scope: ServiceApiScope,
    private_scope: ServiceApiScope,
}

impl ServiceApiBuilder {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Returns to a mutable reference to the public API builder.
    pub fn public_scope(&mut self) -> &mut ServiceApiScope {
        &mut self.public_scope
    }

    /// Returs to a mutable reference to the private API builder.
    pub fn private_scope(&mut self) -> &mut ServiceApiScope {
        &mut self.private_scope
    }
}

/// API Aggregator digunakan untuk meng-aggregate requirements untuk keperluan
/// serving rest API-nya.
#[derive(Clone)]
pub struct ApiAggregator {
    inner: BTreeMap<String, ServiceApiBuilder>,
}

impl ApiAggregator {
    /// Create new `ApiAggregator` instance
    pub fn new(services: Vec<Box<dyn Service>>) -> Self {
        let mut inner = BTreeMap::new();

        inner.insert("system".to_owned(), Self::system_api());

        inner.extend(services.iter().map(|service| {
            let prefix = service.name();
            let mut builder = ServiceApiBuilder::new();

            service.wire_api(&mut builder);

            (prefix.to_string(), builder)
        }));

        Self { inner }
    }

    #[inline]
    fn bind<'a, F>(items: F, mut scope: Scope) -> Scope
    where
        F: ::std::iter::IntoIterator<Item = (&'a str, &'a ServiceApiScope)>,
    {
        for item in items {
            scope = scope.nested(&item.0, move |scope| {
                let mut scope = item.1.actix_backend.wire(scope);
                let ress = item.1.resources.iter();
                for ref res in ress {
                    scope = res(scope)
                }
                scope
            });
        }
        scope
    }

    /// Untuk meng-extend scope dengan endpoint yang kita inginkan.
    /// 
    /// # Arguments
    /// 
    /// * `access` - API access kind.
    /// * `scope` - Actix scope instance.
    pub fn extend(&self, access: ApiAccess, scope: Scope) -> Scope {
        match access {
            ApiAccess::Public => {
                let items = self
                    .inner
                    .iter()
                    .map(|(name, builder)| (name.as_ref(), &builder.public_scope));

                Self::bind(items, scope)
            }
            ApiAccess::Private => {
                let items = self
                    .inner
                    .iter()
                    .map(|(name, builder)| (name.as_ref(), &builder.private_scope));

                Self::bind(items, scope)
            }
        }
    }

    /// Build system API
    pub fn system_api() -> ServiceApiBuilder {
        let builder = ServiceApiBuilder::new();
        // TODO: code here
        builder
    }
}

/// State/context yang akan selalu bisa diakses dari handler
/// state ini berisi beberapa object yang mungkin sering digunakan
/// seperti DB connection.
#[derive(Clone)]
pub struct AppState {
    // db: PgConnection,
}

impl AppState {
    #[doc(hidden)]
    pub fn new() -> AppState {
        // let db_url = env::var("DATABASE_URL").expect("no DATABASE_URL env var");
        AppState {
            // db: db::connect(&db_url),
        }
    }
}

pub(crate) fn create_app(agg: &ApiAggregator, access: ApiAccess) -> App {
    let state = AppState::new();
    let mut app = App::with_state(state);
    app = app.scope("api", |scope: Scope| agg.extend(access, scope));
    // app = app.resource("/test", |r| r.f(|r| "test aja"));
    app
}

/// Start API server
pub fn start(agg: ApiAggregator) {
    let system = System::new("http-server");
    // let agg = agg.clone();
    server::new(move || create_app(&agg, ApiAccess::Public))
        .bind("127.0.0.1:8081")
        .unwrap()
        .run();
}
