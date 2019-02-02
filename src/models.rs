//! Definisi struct untuk model-model yang ada di dalam database.

use chrono::NaiveDateTime;
use serde::Serialize;

use std::fmt;

/// Bentuk model akun di dalam database.
#[derive(Queryable, Serialize, PartialEq)]
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
    pub id: i64,
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
    pub register_time: NaiveDateTime,
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

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Account({}, {})", self.id, self.full_name)
    }
}
