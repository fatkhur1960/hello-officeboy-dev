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

#[derive(Insertable)]
#[table_name = "account_passhash"]
#[doc(hidden)]
pub struct NewAccountPasshash<'a> {
    pub account_id: i64,
    pub passhash: &'a str,
    pub deprecated: bool,
}

/// Type alias for ID in integer
pub type ID = i64;

/// Untuk mengoperasikan skema data di database
pub struct Schema<'a> {
    db: &'a PgConnection,
}

impl<'a> Schema<'a> {
    /// Create new schema instance.
    pub fn new(db: &'a PgConnection) -> Self {
        Self { db }
    }

    /// Mentransfer sejumlah uang dari satu akun ke akun lainnya.
    pub fn transfer(&self, from: ID, to: ID, amount: f64) -> Result<()> {
        use crate::schema::accounts::{self, dsl};

        self.db.build_transaction().read_write().run(|| {
            let from = self.get_account(from)?;
            let to = self.get_account(to)?;

            if from.balance < amount {
                Err(PaymentError::Insufficient("Insufficient balance"))?
            }

            if !from.active || !to.active {
                Err(PaymentError::BadRequest("Account inactive".to_owned()))?
            }

            debug!("transfer {} -> {} amount of {}", from, to, amount);

            diesel::update(dsl::accounts.filter(dsl::id.eq(from.id)))
                .set(dsl::balance.eq(dsl::balance - amount))
                .execute(self.db)?;

            diesel::update(dsl::accounts.filter(dsl::id.eq(to.id)))
                .set(dsl::balance.eq(dsl::balance + amount))
                .execute(self.db)?;

            Ok(())
        })
    }

    /// Get account by ID.
    pub fn get_account(&self, account_id: ID) -> Result<Account> {
        use crate::schema::accounts::dsl::accounts;
        accounts.find(account_id).first(self.db).map_err(From::from)
    }

    /// Mendaftarkan akun baru.
    /// Mengembalikan ID dari registered account (bukan [Account]: apf::models::Account)
    /// karena user belum aktif, untuk mengaktifkannya perlu memanggil
    /// perintah [Schema::activate_registered_account].
    pub fn register_account(&self, full_name: &str, email: &str, phone_num: &str) -> Result<ID> {
        use crate::schema::{accounts::dsl as dsl_account, register_accounts};

        // tolak akun dengan nama-nama tertentu
        // @TODO(robin): buat konfigurable
        if full_name == "nobody" {
            warn!("Name exception to register: `{}`", full_name);
            Err(PaymentError::Unauthorized)?
        }

        // check apakah akun dengan email/phone sama sudah ada
        let exists = dsl_account::accounts
            .filter(
                dsl_account::email
                    .eq(email)
                    .or(dsl_account::phone_num.eq(phone_num)),
            )
            .select(dsl_account::id)
            .first::<ID>(self.db)
            .is_ok();

        if exists {
            Err(PaymentError::AlreadyExists)?
        }

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
            .map_err(From::from)
    }

    /// Setting account's password
    pub fn set_password(&self, account_id: ID, password: &str) -> Result<()> {
        use crate::schema::account_passhash::{self, dsl};

        let _ = self.get_account(account_id)?;

        self.db.build_transaction().read_write().run(|| {
            let passhash = &crate::crypto::get_passhash(password);

            // dipresiasi password lama
            diesel::update(
                dsl::account_passhash.filter(dsl::account_id.eq(account_id).and(dsl::deprecated.eq(false))),
            )
            .set(dsl::deprecated.eq(true))
            .execute(self.db)?;
            // .map_err(From::from)?;

            // tambahkan password baru
            diesel::insert_into(account_passhash::table)
                .values(&NewAccountPasshash {
                    account_id,
                    passhash,
                    deprecated: false,
                })
                .execute(self.db)?;
            // .map_err(From::from)?;

            Ok(())
        })
    }

    /// Mengaktifkan akun yang telah melakukan registrasi tapi belum aktif
    /// bisa diset juga balance pertamanya (initial balance).
    pub fn activate_registered_account(&self, id: ID, initial_balance: f64) -> Result<Account> {
        use crate::schema::{accounts, register_accounts};

        self.db.build_transaction().read_write().run(|| {
            let reg_acc: RegisterAccount = register_accounts::dsl::register_accounts
                .find(id)
                .first(self.db)?;
            // .map_err(From::from)?;

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
                .get_result(self.db)?;
            // .map_err(From::from)?;

            // delete reference in registered accounts table
            diesel::delete(register_accounts::dsl::register_accounts.find(id)).execute(self.db)?;

            Ok(account)
        })
    }
}
