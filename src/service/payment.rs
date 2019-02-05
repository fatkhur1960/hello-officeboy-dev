//! Core implementasi untuk Service Payment
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
// use protobuf;
use serde::Serialize;

use crate::api::SuccessReturn;
use crate::crypto::{self, SecretKey};
use crate::{
    api::{Error as ApiError, HttpRequest as ApiHttpRequest, Result as ApiResult},
    auth, models,
    prelude::*,
    schema_op, tx,
};

use crate::api::payment::*;

macro_rules! api_endpoint {
    ($name:ident, $qt:ty, $rv:ty, (|$schema:ident, $query:ident| $( $cs:tt )+ ) ) => {
        fn $name(state: &AppState, $query: $qt) -> ApiResult<$rv> {
            let $schema = Schema::new(state.db());

            {$($cs)+}
        }
    };
}

macro_rules! api_tx_endpoint {
    ($name:ident, $qt:ty, $rv:ty, (|$schema:ident, $query:ident| $( $cs:tt )+ ) ) => {
        fn $name(state: &AppState, $query: TxQuery<$qt>) -> ApiResult<$rv> {
            let $schema = Schema::new(state.db());

            {$($cs)+}
        }
    };
}

// use crate::api::Error as ApiError;

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
            .endpoint_req_mut("v1/transfer", PublicApi::transfer)
            .endpoint_req_mut("v1/invoice/publish", PublicApi::publish_invoice)
            .endpoint_req_mut("v1/pay", PublicApi::pay)
            .endpoint("v1/balance", PublicApi::balance)
            .endpoint_mut("v1/account/register", PublicApi::register_account)
            .endpoint_mut("v1/account/activate", PublicApi::activate_account)
            .endpoint_mut("v1/authorize", PublicApi::authorize);

        builder
            .private_scope()
            .endpoint_req_mut("v1/credit", PrivateApi::credit)
            .endpoint_req_mut("v1/debit", PrivateApi::debit);
    }
}

/// Holder untuk implementasi API endpoint publik.
struct PublicApi;

impl PublicApi {
    /// Rest API endpoint untuk mendaftarkan akun baru.
    fn register_account(state: &AppState, query: RegisterAccount) -> ApiResult<SuccessReturn<String>> {
        let schema = Schema::new(state.db());

        schema
            .register_account(&query.full_name, &query.email, &query.phone_num)
            .map_err(From::from)
            .map(SuccessReturn::new)
    }

    /// Mengaktifkan user yang telah teregister.
    /// Ini nantinya dijadikan link yang akan dikirimkan ke email pendaftar.
    api_endpoint!(
        activate_account,
        ActivateAccount,
        models::Account,
        (|schema, query| {
            let account = schema.activate_registered_account(query.token, 0.0f64)?;
            schema.set_password(account.id, &query.password)?;
            Ok(account)
        })
    );

    /// Rest API endpoint untuk transfer
    #[authorized_only(user)]
    fn transfer(state: &AppState, query: TxQuery<Transfer>, req: &ApiHttpRequest) -> ApiResult<()> {
        trace!("transfer: {:?}", query);
        trace!("current_account: {}", current_account);

        if current_account.id != query.body.from {
            Err(ApiError::Unauthorized)?
        }

        let schema = Schema::new(state.db());
        schema.transfer(query.body.from, query.body.to, query.body.amount)?;

        Ok(())
    }

    /// Rest API endpoint untuk mendapatkan informasi balance pada akun.
    fn balance(state: &AppState, query: BalanceQuery) -> ApiResult<AccountInfo> {
        // @TODO(*): Code here
        Ok(AccountInfo::new(&query.account, 0.0f64))
    }

    /// Mengaktifkan user yang telah teregister
    fn authorize(state: &AppState, query: Authorize) -> ApiResult<models::AccessToken> {
        {
            let schema = Schema::new(state.db());
            let account = schema.get_account(query.account_id);
        }

        {
            let schema = auth::Schema::new(state.db());

            if !schema.valid_passhash(query.account_id, &query.passhash) {
                warn!(
                    "account `{}` try to authorize using wrong password",
                    &query.account_id
                );
                Err(ApiError::Unauthorized)?
            }

            schema.generate_access_token(query.account_id).map_err(From::from)
        }
    }

    /// Hanya digunakan untuk testing
    api_endpoint!(
        info,
        (),
        SuccessReturn<String>,
        (|s, q| Ok(SuccessReturn::new("success".to_owned())))
    );

    /// API endpoint untuk mem-publish invoice (membuat invoice baru).
    #[authorized_only(user)]
    fn publish_invoice(
        state: &AppState,
        query: TxQuery<PublishInvoice>,
        req: &ApiHttpRequest,
    ) -> ApiResult<SuccessReturn<ID>> {
        let schema = tx::Schema::new(state.db());
        let items = query
            .body
            .items
            .iter()
            .map(|a| tx::NewInvoiceItem {
                invoice_id: 0,
                name: &a.name,
                price: a.price,
            })
            .collect();

        let to = {
            let schema = schema_op::Schema::new(state.db());
            schema.get_account(query.body.to)?
        };

        let new_invoice = tx::NewInvoice {
            id_ref: &query.body.id_ref,
            issuer_account: current_account.id,
            to_account: to.id,
            discount: query.body.discount,
            amount: query.body.amount,
            notes: &query.body.notes,
        };

        schema
            .publish_invoice(new_invoice, items)
            .map_err(From::from)
            .map(SuccessReturn::new)
    }

    /// API endpoint untuk melakukan pembayaran.
    #[authorized_only(user)]
    fn pay(state: &AppState, query: TxQuery<Pay>, req: &ApiHttpRequest) -> ApiResult<SuccessReturn<ID>> {
        let payer = {
            let schema = schema_op::Schema::new(state.db());
            let payer = schema.get_account(query.body.payer)?;
            if payer.id != current_account.id {
                Err(ApiError::NotFound("Invalid account".to_owned()))?
            }
            if query.body.amount > payer.balance {
                Err(ApiError::BadRequest("Insufficient balance".to_owned()))?
            }
            payer
        };
        {
            let schema = tx::Schema::new(state.db());
            schema
                .pay_invoice(query.body.invoice, &payer, query.body.amount, &query.body.via)
                .map_err(From::from)
                .map(SuccessReturn::new)
        }
    }
}

/// Holder untuk implementasi API endpoint privat.
struct PrivateApi;

impl PrivateApi {
    /// Rest API endpoint for topup
    fn credit(state: &AppState, query: TxQuery<Credit>, req: &ApiHttpRequest) -> ApiResult<()> {
        trace!("topup account: {:?}", query);

        let schema = Schema::new(state.db());
        let account = schema.get_account(query.body.account)?;

        {
            let schema = tx::Schema::new(state.db());
            schema.credit(&account, query.body.amount)?;
        }

        Ok(())
    }

    /// Rest API endpoint untuk debit
    fn debit(state: &AppState, query: TxQuery<Debit>, req: &ApiHttpRequest) -> ApiResult<()> {
        trace!("debit: {:?}", query);
        // @TODO(*): Code here
        Ok(())
    }
}
