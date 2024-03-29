//! Definisi struct untuk model-model yang ada di dalam database.

use chrono::NaiveDateTime;
use serde::Serialize;

use std::fmt;

use crate::schema_op::ID;

/// Bentuk model akun di dalam database.
#[derive(Queryable, Clone, Serialize, PartialEq)]
pub struct Account {
    /// ID dari akun.
    pub id: i64,

    /// Nama lengkap akun.
    pub full_name: String,

    /// Saldo akun.
    pub balance: f64,

    /// Alamat email dari akun.
    pub email: String,

    /// Nomor telepon.
    pub phone_num: String,

    /// Penanda apakah akun aktif atau tidak,
    /// apabila tidak aktif maka akun tidak diperkenankan untuk beroperasi.
    pub active: bool,

    /// Waktu kapan akun ini didaftarkan.
    pub register_time: NaiveDateTime,
}

/// Bentuk model dari alamat untuk akun.
#[derive(Queryable)]
pub struct Address {
    /// ID dari record ini.
    pub id: i64,

    /// ID dari akun yang memiliki alamat ini.
    pub account_id: i64,

    /// Jenis alamat, 0: Domisili, 1: Kelahiran
    pub kind: i64,

    /// Alamat
    pub address: String,

    /// Kabupaten
    pub regency: String,

    /// Provinsi
    pub province: String,

    /// Negara
    pub country: String,

    /// Nomor telepon yang bisa dihubungi.
    pub phone_num: String,

    /// Penanda apakah alamat ini masih aktif atau tidak.
    pub active: bool,

    /// Catatan tentang alamat ini.
    pub notes: String,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct RegisterAccount {
    // pub id: i64,
    pub token: String,
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
    pub register_time: NaiveDateTime,
    pub code: String,
}

#[doc(hidden)]
#[derive(Queryable, Serialize, PartialEq, Debug)]
pub struct AccessToken {
    pub token: String,
    pub account_id: i64,
    pub created: NaiveDateTime,
    pub valid_thru: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct AccountPashash {
    pub account_id: i64,
    pub passhash: String,
    pub deperecated: bool,
    pub created: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize, Deserialize)]
pub struct Invoice {
    pub id: ID,
    pub id_ref: String,
    pub issuer_account: ID,
    pub to_account: ID,
    pub discount: f64,
    pub amount: f64,
    pub notes: String,
    pub created: NaiveDateTime,
    pub paid: bool,
    pub paid_by: ID,
    pub paid_at: Option<NaiveDateTime>,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct InvoiceItem {
    pub id: ID,
    pub invoice_id: String,
    pub name: String,
    pub price: f64,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct PaymentHistory {
    pub id: ID,
    pub invoice_id: ID,
    pub payer: ID,
    pub via: String,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct AccountKey {
    pub id: ID,
    pub account_id: ID,
    pub pub_key: String,
    pub secret_key: String,
    pub created: NaiveDateTime,
    pub active: bool,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Transaction {
    pub id: ID,
    pub dbcr_flag: i32,
    pub ttype: i32,
    // pub subtype: i32,
    pub amount: f64,
    pub status: i32,
    pub created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub invoice_id: Option<ID>,
    pub from_account: Option<ID>,
    pub to_account: Option<ID>,
    pub merchant_id: Option<ID>,
    pub notes: Option<String>,
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Account({}, {})", self.id, self.full_name)
    }
}

impl fmt::Display for AccountKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key({})", &self.pub_key[..8])
    }
}
