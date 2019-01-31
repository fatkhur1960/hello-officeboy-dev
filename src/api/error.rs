// Workaround for `failure` see https://github.com/rust-lang-nursery/failure/issues/223 and
// ECR-1771 for the details.
#![allow(bare_trait_objects)]

//! The set of errors for the API module.

use actix_web::http::StatusCode;
use serde::Serialize;

use crate::api::ApiResult;

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

    /// Error yang muncul apabila user menginputkan parameter yang tidak sesuai
    #[fail(display = "Invalid parameter: {}", _0)]
    InvalidParameter(String),

    /// Error yang bisa digunakan untuk menampilkan kode dan deskripsi secara custom.
    #[fail(display = "error code {}: {}", _1, _0)]
    CustomError(String, i32),

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
                // HttpResponse::BadRequest().json(ApiErrorJson::new(err.to_owned()))
                HttpResponse::BadRequest().json(ApiResult::error(400, "Bad request".to_owned()))
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
            Error::InvalidParameter(d) => {
                HttpResponse::BadRequest().json(ApiResult::error(5, d.to_owned()))
            }
            Error::CustomError(d, code) => HttpResponse::build(StatusCode::from_u16(406).unwrap())
                .json(ApiResult::error(*code, d.to_owned())),
            Error::Unauthorized => HttpResponse::Unauthorized().finish(),
        }
    }
}
