//! Core implementasi untuk Service Payment
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
// use protobuf;
use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::crypto::{self, SecretKey};
use crate::{
    api::{Error as ApiError, SuccessReturn},
    auth, models,
    prelude::*,
    schema_op, tx,
};

// use crate::api::payment::{PrivateApi, PublicApi};

/// Core basis service apf.
/// Service ini yang men-serve beberapa endpoint transaksional seperti:
/// /credit, /transfer, /debit, /balance
pub struct AuthService {}

impl AuthService {
    #[doc(hidden)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Service for AuthService {
    fn name(&self) -> &'static str {
        "auth"
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint_mut("v1/authorize", PublicApi::authorize)
            .endpoint("v1/get_key", PublicApi::account_get_key);

        builder.private_scope();
    }
}

use crate::models::AccessToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorize {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub passhash: String,
}

struct PublicApi {}

impl PublicApi {
    /// Meng-otorisasi akun yang telah teregister
    /// User bisa melakukan otorisasi menggunakan email / nomor telp.
    #[api_endpoint(path = "/authorize", auth = "none", mutable)]
    pub fn authorize(state: &mut AppState, query: Authorize) -> AccessToken {
        let account = {
            let schema = Schema::new(state.db());
            if let Some(email) = query.email {
                schema.get_account_by_email(&email)?
            } else if let Some(phone) = query.phone {
                schema.get_account_by_phone_num(&phone)?
            } else {
                Err(ApiError::InvalidParameter("No email/phone parameter".to_string()))?
            }
        };

        {
            let schema = auth::Schema::new(state.db());

            if !schema.valid_passhash(account.id, &query.passhash) {
                warn!("account `{}` try to authorize using wrong password", &account.id);
                Err(ApiError::Unauthorized)?
            }

            schema.generate_access_token(account.id).map_err(From::from)
        }
    }

    #[api_endpoint(path = "/get_key", auth = "required")]
    fn account_get_key(query: ()) -> SuccessReturn<JsonValue> {
        let schema = Schema::new(state.db());
        let account_key = schema.get_account_key(current_account.id)?;

        Ok(SuccessReturn::new(
            json!({"pub_key": account_key.pub_key, "secret_key": account_key.secret_key}),
        ))
    }
}
