//! Koleksi query yang digunakan untuk operasi pada rest API.
 #![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use protobuf;
use serde::Serialize;

use crate::crypto::{self, SecretKey};
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credit {
    pub account: ID,
    pub amount: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Debit {
    pub account: String,
    pub amount: Option<f64>,
    pub timestamp: u64,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Transfer {
//     pub from: ID,
//     pub to: ID,
//     pub amount: f64,
//     pub timestamp: u64,
// }
pub use crate::protos::Transfer;

/// Query transaction untuk melakukan pembayaran
#[derive(Debug, Serialize, Deserialize)]
pub struct Pay {
    /// akun yang membayar
    pub payer: ID,
    /// ID dari invoice.
    pub invoice: ID,
    /// Jumlah yang dibayarkan.
    pub amount: f64,

    pub timestamp: NaiveDateTime,

    pub via: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorize {
    pub account_id: ID,
    pub passhash: String,
}

/// Definisi query untuk mendaftarkan akun baru via rest API.
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAccount {
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
    // comment out: mungkin tidak untuk sekarang
    // pub nik: String,
}

/// Definisi query untuk mengaktifkan akun yang telah didaftarkan.
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateAccount {
    pub reg_id: ID,
    pub initial_balance: f64,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxQuery<T>
where
    T: Serialize,
{
    pub body: T,
    pub signature: String,
}

impl<T> TxQuery<T>
where
    T: protobuf::Message + Serialize + Clone,
{
    pub fn sign(&self, secret_key: &SecretKey) -> Self {
        assert!(self.signature.len() > 0, "already signed.");

        // convert ke bytes format protobuf
        let bytes = self.body.write_to_bytes().expect("Cannot write to bytes");
        let signature = crypto::sign(&bytes, &secret_key);
        Self {
            body: self.body.clone(),
            signature: signature.to_hex(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceQuery {
    pub account: String,
}

#[derive(Debug, Serialize)]
pub struct AccountInfo {
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
pub struct PublishInvoice {
    pub id_ref: String,
    pub issuer: ID,
    pub to: ID,
    pub discount: f64,
    pub amount: f64,
    pub notes: String,
    pub items: Vec<InvoiceItem>,
}

#[derive(Serialize, Deserialize)]
pub struct InvoiceItem {
    pub name: String,
    pub price: f64,
}
