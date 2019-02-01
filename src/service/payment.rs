//! Core implementasi untuk Service Payment

use actix_web::HttpResponse;
use serde::Serialize;

use crate::api;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Credit {
    pub account: ID,
    pub amount: Option<f64>,
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

#[derive(Debug, Serialize, Deserialize)]
struct ActivateAccount {
    pub account_id: ID,
    pub initial_balance: f64,
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

macro_rules! api_endpoint {
    ($name:ident, $qt:ident, $rv:ty, (|$schema:ident, $query:ident| $( $cs:tt )+ ) ) => {
        fn $name(state: &AppState, $query: TxQuery<$qt>) -> api::Result<$rv> {
            let $schema = Schema::new(state.db());

            {$($cs)+}
        }
    };
}

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
    fn credit(state: &AppState, query: TxQuery<Credit>) -> api::Result<()> {
        trace!("topup account: {:?}", query);
        // @TODO(*): Code here
        Ok(())
    }

    /// Rest API endpoint untuk transfer
    fn transfer(state: &AppState, query: TxQuery<Transfer>) -> api::Result<()> {
        trace!("transfer: {:?}", query);
        // @TODO(*): code here
        Ok(())
    }

    /// Rest API endpoint untuk debit
    fn debit(state: &AppState, query: TxQuery<Debit>) -> api::Result<()> {
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
    fn register_account(state: &AppState, query: TxQuery<CreateAccount>) -> api::Result<()> {
        let schema = Schema::new(state.db());

        schema.register_account(
            &query.body.full_name,
            &query.body.email,
            &query.body.phone_num,
        )?;

        Ok(())
    }

    /// Mengaktifkan user yang telah teregister
    api_endpoint!(
        activate_account,
        ActivateAccount,
        (),
        (|schema, query| {
            schema
                .activate_registered_account(query.body.account_id, query.body.initial_balance)?;
            Ok(())
        })
    );
}

impl Service for PaymentService {
    fn name(&self) -> &'static str {
        "payment"
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint_mut("v1/credit", Self::credit)
            .endpoint_mut("v1/transfer", Self::transfer)
            .endpoint_mut("v1/debit", Self::debit)
            .endpoint("v1/balance", Self::balance);

        builder
            .private_scope()
            .endpoint_mut("v1/account/register", Self::register_account)
            .endpoint_mut("v1/account/activate", Self::activate_account);
    }
}
