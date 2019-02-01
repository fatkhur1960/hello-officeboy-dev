//! Schema operation a.k.a DAO, digunakan untuk melakukan operasi seperti
//! membuat akun baru, update, dan delete.

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use failure;

use crate::{error::Error as PaymentError, models::*, result::Result, schema::*};

use std::sync::Arc;

#[derive(Insertable)]
#[table_name = "register_accounts"]
#[doc(hidden)]
pub struct NewRegisterAccount<'a> {
    pub full_name: &'a str,
    pub email: &'a str,
    pub phone_num: &'a str,
    pub register_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "accounts"]
#[doc(hidden)]
pub struct NewAccount<'a> {
    pub full_name: &'a str,
    pub balance: f64,
    pub email: &'a str,
    pub phone_num: &'a str,
    pub active: bool,
    pub register_time: NaiveDateTime,
}

/// Type alias for ID in integer
pub type ID = i64;

/// Untuk mengoperasikan skema data di database
pub struct Schema<'a> {
    db: &'a PgConnection,
}

fn error_mapper(e: diesel::result::Error) -> PaymentError {
    e.into()
}

impl<'a> Schema<'a> {
    /// Create new schema instance.
    pub fn new(db: &'a PgConnection) -> Self {
        Self { db }
    }

    /// Mendaftarkan akun baru.
    /// Mengembalikan ID dari registered account (bukan [Account]: payment::models::Account)
    /// karena user belum aktif, untuk mengaktifkannya perlu memanggil
    /// perintah [Schema::activate_registered_account].
    pub fn register_account(&self, full_name: &str, email: &str, phone_num: &str) -> Result<ID> {
        use crate::schema::register_accounts;

        let new_reg_account = NewRegisterAccount {
            full_name,
            email,
            phone_num,
            register_time: Utc::now().naive_utc(),
        };

        diesel::insert_into(register_accounts::table)
            .values(&new_reg_account)
            .get_result::<RegisterAccount>(self.db)
            .map(|d| d.id)
            .map_err(error_mapper)
    }

    /// Mengaktifkan akun yang telah melakukan registrasi tapi belum aktif
    /// bisa diset juga balance pertamanya (initial balance).
    pub fn activate_registered_account(&self, id: ID, initial_balance: f64) -> Result<Account> {
        use crate::schema::accounts;
        use crate::schema::register_accounts;

        self.db.build_transaction().read_write().run(|| {
            let reg_acc: RegisterAccount = register_accounts::dsl::register_accounts
                .find(id)
                .first(self.db)
                .map_err(error_mapper)?;

            let new_account = NewAccount {
                full_name: &reg_acc.full_name,
                balance: initial_balance,
                email: &reg_acc.email,
                phone_num: &reg_acc.phone_num,
                active: true,
                register_time: Utc::now().naive_utc(),
            };

            let account = diesel::insert_into(accounts::table)
                .values(&new_account)
                .get_result(self.db)
                .map_err(error_mapper)?;

            // delete reference in registered accounts table
            diesel::delete(register_accounts::dsl::register_accounts.find(id)).execute(self.db)?;

            Ok(account)
        })

    }
}
