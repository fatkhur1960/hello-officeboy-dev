//! Core implementasi untuk Service Payment

use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;

use crate::models;
use crate::prelude::*;
use crate::{api, auth, schema_op, tx};

#[derive(Debug, Serialize, Deserialize)]
struct Credit {
    pub account: ID,
    pub amount: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debit {
    pub account: String,
    pub amount: Option<f64>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Transfer {
    pub from: ID,
    pub to: ID,
    pub amount: f64,
    pub timestamp: u64,
}

/// Query transaction untuk melakukan pembayaran
#[derive(Debug, Serialize, Deserialize)]
struct Pay {
    /// akun yang membayar
    pub payer: ID,
    /// ID dari invoice.
    pub invoice: ID,
    /// Jumlah yang dibayarkan.
    pub amount: f64,
    pub timestamp: u64,
    pub via: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Authorize {
    pub account_id: ID,
    pub passhash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ActivateAccount {
    pub reg_id: ID,
    pub initial_balance: f64,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateAccount {
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
    // comment out: mungkin tidak untuk sekarang
    // pub nik: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TxQuery<T>
where
    T: Serialize,
{
    body: T,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BalanceQuery {
    pub account: String,
}

#[derive(Debug, Serialize)]
struct AccountInfo {
    pub id: String,
    pub balance: f64,
}

impl AccountInfo {
    pub fn new(id: &str, balance: f64) -> Self {
        Self {
            id: id.to_owned(),
            balance,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PublishInvoice {
    pub id_ref: String,
    pub issuer: ID,
    pub to: ID,
    pub discount: f64,
    pub amount: f64,
    pub notes: String,
    pub items: Vec<InvoiceItem>,
}

#[derive(Serialize, Deserialize)]
struct InvoiceItem {
    pub name: String,
    pub price: f64,
}

#[derive(Debug, Serialize, PartialEq)]
struct SuccessReturn<T> {
    result: T,
}

impl<T: Serialize> SuccessReturn<T> {
    pub fn new(result: T) -> Self {
        Self { result }
    }
}

macro_rules! api_endpoint {
    ($name:ident, $qt:ident, $rv:ty, (|$schema:ident, $query:ident| $( $cs:tt )+ ) ) => {
        fn $name(state: &AppState, $query: TxQuery<$qt>) -> api::Result<$rv> {
            let $schema = Schema::new(state.db());

            {$($cs)+}
        }
    };
}

// use crate::api::Error as ApiError;

/// Core basis service payment.
/// Service ini yang men-serve beberapa endpoint transaksional seperti:
/// /credit, /transfer, /debit, /balance
pub struct PaymentService {}

impl PaymentService {
    #[doc(hidden)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    /// Rest API endpoint for topup
    fn credit(state: &AppState, query: TxQuery<Credit>, req: &api::HttpRequest) -> api::Result<()> {
        trace!("topup account: {:?}", query);

        let schema = Schema::new(state.db());
        let account = schema.get_account(query.body.account)?;

        {
            let schema = tx::Schema::new(state.db());
            schema.credit(&account, query.body.amount)?;
        }

        Ok(())
    }

    /// Rest API endpoint untuk transfer
    #[authorized_only(user)]
    fn transfer(
        state: &AppState,
        query: TxQuery<Transfer>,
        req: &api::HttpRequest,
    ) -> api::Result<()> {
        trace!("transfer: {:?}", query);
        trace!("current_account: {}", current_account);

        if current_account.id != query.body.from {
            Err(api::Error::Unauthorized)?
        }

        let schema = Schema::new(state.db());
        schema.transfer(query.body.from, query.body.to, query.body.amount)?;

        Ok(())
    }

    /// Rest API endpoint untuk debit
    fn debit(state: &AppState, query: TxQuery<Debit>, req: &api::HttpRequest) -> api::Result<()> {
        trace!("debit: {:?}", query);
        // @TODO(*): Code here
        Ok(())
    }

    /// Rest API endpoint untuk mendapatkan informasi balance pada akun.
    fn balance(state: &AppState, query: BalanceQuery) -> api::Result<AccountInfo> {
        // @TODO(*): Code here
        Ok(AccountInfo::new(&query.account, 0.0f64))
    }

    /// Rest API endpoint untuk mendaftarkan akun baru.
    fn register_account(
        state: &AppState,
        query: TxQuery<CreateAccount>,
    ) -> api::Result<SuccessReturn<ID>> {
        let schema = Schema::new(state.db());

        schema
            .register_account(
                &query.body.full_name,
                &query.body.email,
                &query.body.phone_num,
            )
            .map_err(From::from)
            .map(SuccessReturn::new)
    }

    /// Mengaktifkan user yang telah teregister
    fn authorize(state: &AppState, query: Authorize) -> api::Result<models::AccessToken> {
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
                Err(api::Error::Unauthorized)?
            }

            schema
                .generate_access_token(query.account_id)
                .map_err(From::from)
        }
    }

    /// Mengaktifkan user yang telah teregister
    api_endpoint!(
        activate_account,
        ActivateAccount,
        models::Account,
        (|schema, query| {
            let account = schema
                .activate_registered_account(query.body.reg_id, query.body.initial_balance)?;
            schema.set_password(account.id, &query.body.password)?;
            Ok(account)
        })
    );

    /// API endpoint untuk mem-publish invoice (membuat invoice baru).
    #[authorized_only(user)]
    fn publish_invoice(
        state: &AppState,
        query: TxQuery<PublishInvoice>,
        req: &api::HttpRequest,
    ) -> api::Result<SuccessReturn<ID>> {
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

        schema
            .publish_invoice(
                &query.body.id_ref,
                &current_account,
                &to,
                query.body.discount,
                query.body.amount,
                &query.body.notes,
                items,
            )
            .map_err(From::from)
            .map(SuccessReturn::new)
    }

    /// API endpoint untuk melakukan pembayaran.
    #[authorized_only(user)]
    fn pay(state: &AppState, query: TxQuery<Pay>, req: &api::HttpRequest) -> api::Result<SuccessReturn<ID>> {
        let payer = {
            let schema = schema_op::Schema::new(state.db());
            let payer = schema.get_account(query.body.payer)?;
            if payer.id != current_account.id {
                Err(api::Error::NotFound("Invalid account".to_owned()))?
            }
            if query.body.amount > payer.balance {
                Err(api::Error::BadRequest("Insufficient balance".to_owned()))?
            }
            payer
        };
        {
            let schema = tx::Schema::new(state.db());
            schema
                .pay_invoice(
                    query.body.invoice,
                    &payer,
                    query.body.amount,
                    &query.body.via,
                )
                .map_err(From::from)
                .map(SuccessReturn::new)
        }
    }
}

impl Service for PaymentService {
    fn name(&self) -> &'static str {
        "payment"
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint_req_mut("v1/transfer", Self::transfer)
            .endpoint_req_mut("v1/invoice/publish", Self::publish_invoice)
            .endpoint_req_mut("v1/pay", Self::pay)
            .endpoint("v1/balance", Self::balance)
            .endpoint_mut("v1/authorize", Self::authorize);

        builder
            .private_scope()
            .endpoint_mut("v1/account/register", Self::register_account)
            .endpoint_mut("v1/account/activate", Self::activate_account)
            .endpoint_req_mut("v1/credit", Self::credit)
            .endpoint_req_mut("v1/debit", Self::debit);
    }
}
