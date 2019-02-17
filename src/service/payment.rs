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

use crate::api::payment::{PrivateApi, PublicApi};

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
        builder
            .public_scope()
            .endpoint("v1/info", PublicApi::info)
            .endpoint("v1/me/info", PublicApi::me_info)
            .endpoint("v1/balance", PublicApi::balance)
            .endpoint_mut("v1/transfer", PublicApi::transfer)
            .endpoint("v1/invoice", PublicApi::get_invoice)
            .endpoint_mut("v1/invoice/publish", PublicApi::publish_invoice)
            .endpoint_mut("v1/pay", PublicApi::pay)
            .endpoint_mut("v1/account/register", PublicApi::register_account)
            .endpoint_mut("v1/account/activate", PublicApi::activate_account);

        builder
            .private_scope()
            .endpoint_mut("v1/credit", PrivateApi::credit)
            .endpoint_mut("v1/debit", PrivateApi::debit)
            .endpoint("v1/accounts", PrivateApi::search_accounts)
            .endpoint("v1/account/search", PrivateApi::search_accounts)
            .endpoint("v1/account/count", PrivateApi::account_count)
            .endpoint("v1/account/info", PrivateApi::account_info)
            .endpoint("v1/transactions", PrivateApi::transactions);
    }
}
