//! Definisi error untuk system internal

use diesel;
use failure;
use std::io;

/// Listing dari jenis error yang mungkin muncul pada sistem internal
#[derive(Fail, Debug)]
pub enum Error {
    /// Storage error. This type includes errors related to the database, caused
    /// by, for example, serialization issues.
    #[fail(display = "Storage error: {}", _0)]
    Storage(#[cause] diesel::result::Error),

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

    /// Error yang muncul ketika sebuah object unik telah ada
    /// biasanya dimunculkan oleh operasi creation.
    #[fail(display = "Already exists")]
    AlreadyExists,

    /// Error yang muncul apabila sesuatu tidak mencukupi seperti saldo misalnya.
    #[fail(display = "{}", _0)]
    Insufficient(&'static str),

    /// Error yang bisa digunakan untuk menampilkan kode dan deskripsi secara custom.
    #[fail(display = "error code {}: {}", _1, _0)]
    CustomError(String, i32),

    /// Unauthorized error. This error occurs when the request lacks valid
    /// authentication credentials.
    #[fail(display = "Unauthorized")]
    Unauthorized,
}

// semua error yang berasal dari diesel akan dipropagasi ke sistem error [Error::Storage]
impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Storage(e)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::BadRequest("Invalid data".to_string())
    }
}
