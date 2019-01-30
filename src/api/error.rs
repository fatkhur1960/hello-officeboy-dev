// Workaround for `failure` see https://github.com/rust-lang-nursery/failure/issues/223 and
// ECR-1771 for the details.
#![allow(bare_trait_objects)]

//! The set of errors for the API module.

use serde::Serialize;

use failure;
use std::io;

/// List of possible API errors.
#[derive(Fail, Debug)]
pub enum Error {
    // /// Storage error. This type includes errors related to the database, caused
    // /// by, for example, serialization issues.
    // #[fail(display = "Storage error: {}", _0)]
    // Storage(#[cause] storage::Error),
    /// Input/output error. This type includes errors related to files that are not
    /// a part of the Exonum storage.
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),

    /// Bad request. This error occurs when the request contains invalid syntax.
    #[fail(display = "Bad request: {}", _0)]
    BadRequest(String),

    /// Not found. This error occurs when the server cannot locate the requested
    /// resource.
    #[fail(display = "Not found: {}", _0)]
    NotFound(String),

    /// Internal server error. This type can return any internal server error to the user.
    #[fail(display = "Internal server error: {}", _0)]
    InternalError(failure::Error),

    /// Unauthorized error. This error occurs when the request lacks valid
    /// authentication credentials.
    #[fail(display = "Unauthorized")]
    Unauthorized,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<failure::Error> for Error {
    fn from(e: failure::Error) -> Self {
        Error::InternalError(e)
    }
}

#[derive(Serialize)]
struct ApiErrorJson {
    pub error: String,
}

impl ApiErrorJson {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

use actix_web::{HttpResponse, ResponseError};

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest(err) => {
                HttpResponse::BadRequest().json(ApiErrorJson::new(err.to_owned()))
            }
            Error::InternalError(err) => {
                HttpResponse::InternalServerError().json(ApiErrorJson::new(err.to_string()))
            }
            Error::Io(err) => {
                HttpResponse::InternalServerError().json(ApiErrorJson::new(err.to_string()))
            }
            // Error::Storage(err) => {
            //     HttpResponse::InternalServerError().json(ApiErrorJson::new(err.to_string()))
            // }
            Error::NotFound(err) => {
                HttpResponse::NotFound().json(ApiErrorJson::new(err.to_string()))
            }
            Error::Unauthorized => HttpResponse::Unauthorized().finish(),
        }
    }
}
