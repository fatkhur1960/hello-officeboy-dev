//! Core implementasi untuk Service Payment
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
// use protobuf;
use serde::Serialize;

// use crate::api::SuccessReturn;
use crate::crypto::{self, SecretKey};
use crate::{
    api::{Error as ApiError, HttpRequest as ApiHttpRequest, Result as ApiResult},
    auth, models,
    prelude::*,
    schema_op, tx,
};

use crate::api::payment::{PrivateApi, PublicApi, PublicApiAccount};

/// Core basis service apf.
/// Service ini yang men-serve beberapa endpoint transaksional seperti:
/// /credit, /transfer, /debit, /balance
pub struct PaymentService {}

impl PaymentService {
    #[doc(hidden)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Service for PaymentService {
    fn name(&self) -> &'static str {
        "payment"
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        builder.public_scope().link(PublicApi::wire);
        builder.public_scope().link(PublicApiAccount::wire);
        builder.private_scope().link(PrivateApi::wire);
    }
}
