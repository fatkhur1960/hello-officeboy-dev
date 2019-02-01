//! Core implementasi untuk Service Payment

use actix_web::HttpResponse;
use serde::Serialize;

use crate::api;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Credit {
    pub account: String,
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
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: u64,
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

/// Core basis service payment
/// Service ini yang men-serve endpoint:
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

    /// Rest API endpoint untuk membuat akun baru.
    fn register_account(state: &AppState, query: TxQuery<CreateAccount>) -> api::Result<()> {
        let schema = Schema::new(state.db());

        schema.register_account(
            &query.body.full_name,
            &query.body.email,
            &query.body.phone_num,
        )?;

        Ok(())
    }
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
            .endpoint_mut("v1/register_account", Self::register_account);
    }
}
