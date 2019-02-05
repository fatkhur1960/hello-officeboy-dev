//! Koleksi query yang digunakan untuk operasi pada rest API.
 #![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use protobuf;
use serde::Serialize;

use crate::api::SuccessReturn;
use crate::crypto::{self, SecretKey};
use crate::{
    api::{Error as ApiError, HttpRequest as ApiHttpRequest, Result as ApiResult},
    auth,
    prelude::*,
    schema_op, tx,
};

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
    pub token: String,
    pub password: String,
}

/// Setiap query transaksi harus menggunakan wrapper ini,
/// masuk pada `body` dan signature dari `body` disimpan pada field `signature`.
///
/// TxQuery implement `sign` method yang bisa digunakan untuk melakukan signing
/// pada data di `body`.
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
    /// Lakukan signing pada data di `body`.
    /// Operasi signing dilakukan dengan cara men-serialize data pada `body` ke dalam
    /// bentuk protobuf bytes lalu di-sign menggunakan `secret_key`.
    pub fn sign(&self, secret_key: &SecretKey) -> Self {
        assert!(self.signature.is_empty(), "already signed.");

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

/// Model untuk keperluan tukar menukar data API
/// bukan yang di database (crate::models).
pub mod models {

    use chrono::NaiveDateTime;

    use crate::models;

    use std::convert::From;

    /// Bentuk model akun di dalam database.
    #[derive(Clone, Serialize, Deserialize, PartialEq)]
    pub struct Account {
        /// ID dari akun.
        pub id: i64,

        /// Nama lengkap akun.
        pub full_name: String,

        /// Waktu kapan akun ini didaftarkan.
        pub register_time: NaiveDateTime,
    }

    impl From<models::Account> for Account {
        fn from(a: models::Account) -> Self {
            Account {
                id: a.id,
                full_name: a.full_name,
                register_time: a.register_time,
            }
        }
    }

}

use crate::models::AccessToken;

/// Holder untuk implementasi API endpoint publik.
pub struct PublicApi;

impl PublicApi {
    /// Rest API endpoint untuk mendaftarkan akun baru.
    pub fn register_account(state: &AppState, query: RegisterAccount) -> ApiResult<SuccessReturn<String>> {
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
            Ok(account.into())
        })
    );

    /// Rest API endpoint untuk transfer
    #[authorized_only(user)]
    pub fn transfer(state: &AppState, query: TxQuery<Transfer>, req: &ApiHttpRequest) -> ApiResult<()> {
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
    pub fn balance(state: &AppState, query: BalanceQuery) -> ApiResult<AccountInfo> {
        // @TODO(*): Code here
        Ok(AccountInfo::new(&query.account, 0.0f64))
    }

    /// Mengaktifkan user yang telah teregister
    pub fn authorize(state: &AppState, query: Authorize) -> ApiResult<AccessToken> {
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
    pub fn publish_invoice(
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
    pub fn pay(state: &AppState, query: TxQuery<Pay>, req: &ApiHttpRequest) -> ApiResult<SuccessReturn<ID>> {
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
pub struct PrivateApi;

impl PrivateApi {
    /// Rest API endpoint for topup
    pub fn credit(state: &AppState, query: TxQuery<Credit>, req: &ApiHttpRequest) -> ApiResult<()> {
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
    pub fn debit(state: &AppState, query: TxQuery<Debit>, req: &ApiHttpRequest) -> ApiResult<()> {
        trace!("debit: {:?}", query);
        // @TODO(*): Code here
        Ok(())
    }
}
