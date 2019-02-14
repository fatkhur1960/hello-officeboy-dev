//! Koleksi query yang digunakan untuk operasi pada rest API.
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use protobuf;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::api::SuccessReturn;
use crate::crypto::{self, PublicKey, SecretKey, Signature};

use crate::{
    api,
    api::payment::models::*,
    api::{Error as ApiError, HttpRequest as ApiHttpRequest, Result as ApiResult},
    auth,
    error::Error,
    prelude::*,
    schema_op, tx,
};

#[derive(Serialize)]
pub struct EntriesResult<T> {
    pub entries: Vec<T>,
    pub count: i64,
}

#[derive(Deserialize)]
pub struct ListAccount {
    pub query: Option<String>,
    pub page: i64,
    pub limit: i64,
}

#[derive(Deserialize)]
pub struct QueryEntries {
    pub query: Option<String>,
    pub page: i64,
    pub limit: i64,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Credit {
//     pub account: ID,
//     pub amount: f64,
//     pub timestamp: u64,
// }

pub use crate::protos::Credit;

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
    /// Create new tx query instance.
    pub fn new(body: T) -> Self {
        Self {
            body,
            signature: Default::default(),
        }
    }

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

    /// Untuk memverifikasi signature pada body ini.
    pub fn verify(&self, public_key: &PublicKey, secret_key: &SecretKey) -> Result<()> {
        if self.signature.is_empty() {
            Err(Error::BadRequest("message has no signature.".to_string()))?
        }
        let bytes = self.body.write_to_bytes().expect("Cannot write to bytes");
        let signature: Signature = self.signature.parse::<Signature>()?;
        debug!("verify sig `{}` using pubkey: `{}`", &signature, public_key);
        if !crypto::is_verified(bytes.as_slice(), &signature, public_key) {
            // let bytes = self.body.write_to_bytes().expect("Cannot write to bytes");
            let real_signature = crypto::sign(&bytes, &secret_key);
            debug!("  - expected signature: `{}`", real_signature);
            debug!("  - data: `{}`", hex::encode(bytes));
            Err(Error::Unauthorized)?
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceQuery {
    pub account: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountQuery {
    pub id: ID,
}

#[derive(Debug, Serialize)]
pub struct AccountInfo {
    pub id: ID,
    pub balance: f64,
}

impl AccountInfo {
    pub fn new(id: ID, balance: f64) -> Self {
        Self { id, balance }
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

        /// Alamat email kun.
        pub email: String,

        /// Nomor telpon akun.
        pub phone_num: String,

        /// Waktu kapan akun ini didaftarkan.
        pub register_time: NaiveDateTime,
    }

    impl From<models::Account> for Account {
        fn from(a: models::Account) -> Self {
            Account {
                id: a.id,
                full_name: a.full_name,
                email: a.email,
                phone_num: a.phone_num,
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
    #[api_endpoint(path = "/account/register", mutable, auth = "none")]
    pub fn register_account(state: &mut AppState, query: RegisterAccount) -> SuccessReturn<String> {
        let schema = Schema::new(state.db());

        schema
            .register_account(&query.full_name, &query.email, &query.phone_num)
            .map_err(From::from)
            .map(SuccessReturn::new)
    }

    /// Mengaktifkan user yang telah teregister.
    /// Ini nantinya dijadikan link yang akan dikirimkan ke email pendaftar.
    #[api_endpoint(path = "/account/activate", auth = "none", mutable)]
    pub fn activate_account(query: ActivateAccount) -> Account {
        let schema = Schema::new(state.db());
        let account = schema.activate_registered_account(query.token, 0.0f64)?;
        schema.set_password(account.id, &query.password)?;
        Ok(account.into())
    }

    /// Rest API endpoint untuk transfer
    #[api_endpoint(path = "/transfer", auth = "required", mutable)]
    pub fn transfer(query: TxQuery<Transfer>) -> () {
        trace!("transfer: {:?}", query);
        trace!("current_account: {}", current_account);

        if current_account.id != query.body.from {
            Err(ApiError::Unauthorized)?
        }

        if query.body.amount > 1_000_000_000f64 {
            // batas maksimal transfer sementara dibatasi
            // untuk menghindari human error
            // sampai jelas mekanismenya
            Err(ApiError::InvalidParameter("Max limit reached".to_string()))?
        }

        let schema = Schema::new(state.db());

        // verifikasi digital signature
        let acc_key = schema.get_account_key(current_account.id)?;
        let secret_key = acc_key.secret_key.parse::<SecretKey>()?;
        let public_key = acc_key.pub_key.parse::<PublicKey>()?;
        query.verify(&public_key, &secret_key)?;

        schema.transfer(query.body.from, query.body.to, query.body.amount)?;

        Ok(())
    }

    /// Rest API endpoint untuk mendapatkan informasi balance pada akun.
    #[api_endpoint(path = "/balance", auth = "required")]
    pub fn balance(state: &AppState, query: BalanceQuery) -> AccountInfo {
        // @TODO(*): Code here
        // Ok(AccountInfo::new(&query.account, 0.0f64))
        unimplemented!()
    }

    /// Hanya digunakan untuk testing sahaja.
    #[api_endpoint(path = "/info", auth = "optional")]
    pub fn info(query: ()) -> JsonValue {
        Ok(json!({ "version": env!("CARGO_PKG_VERSION") }))
    }

    /// Mendapatkan informasi current account.
    #[api_endpoint(path = "/me/info", auth = "required")]
    pub fn me_info(state: &AppState, query: (), req: &ApiHttpRequest) -> Account {
        Ok(current_account.into())
    }

    /// API endpoint untuk mem-publish invoice (membuat invoice baru).
    #[authorized_only(user)]
    pub fn publish_invoice(
        state: &mut AppState,
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
    #[api_endpoint(path = "/pay", auth = "required", mutable)]
    pub fn pay(query: TxQuery<Pay>) -> SuccessReturn<ID> {
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

use crate::models as db;

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

impl PrivateApi {
    /// Rest API endpoint for topup
    /// Mengembalikan jumlah balance akun setelah dikredit.
    #[api_endpoint(path = "/credit", auth = "required", mutable)]
    pub fn credit(state: &mut AppState, query: TxQuery<Credit>, req: &ApiHttpRequest) -> SuccessReturn<f64> {
        trace!("topup account: {:?}", query);

        let schema = Schema::new(state.db());

        // verifikasi digital signature
        let acc_key = schema.get_account_key(current_account.id)?;
        let secret_key = acc_key.secret_key.parse::<SecretKey>()?;
        let public_key = acc_key.pub_key.parse::<PublicKey>()?;
        query.verify(&public_key, &secret_key)?;

        let account = schema.get_account(query.body.account)?;

        let tx_id = {
            let schema = tx::Schema::new(state.db());
            schema.credit(&account, query.body.amount)?
        };

        debug!("credit transaction processed with id `{}`", tx_id);

        let account = schema.get_account(query.body.account)?;

        Ok(SuccessReturn::new(account.balance))
    }

    /// Rest API endpoint untuk debit
    pub fn debit(state: &mut AppState, query: TxQuery<Debit>, req: &ApiHttpRequest) -> ApiResult<()> {
        trace!("debit: {:?}", query);
        // @TODO(*): Code here
        Ok(())
    }

    /// Listing account
    #[api_endpoint(path = "/accounts", auth = "none")]
    pub fn list_account(query: ListAccount) -> EntriesResult<db::Account> {
        let schema = Schema::new(state.db());

        let offset = query.page * query.limit;

        let entries = schema.get_accounts(offset, query.limit)?;

        let count = schema.get_account_count()?;
        Ok(EntriesResult { count, entries })
    }

    /// Mencari akun berdasarkan kata kunci
    #[api_endpoint(path = "/account/search", auth = "none")]
    pub fn search_accounts(query: ListAccount) -> EntriesResult<db::Account> {
        let schema = Schema::new(state.db());

        let offset = query.page * query.limit;

        if query.query.is_none() {
            return Self::list_account(&state, query, req);
        }

        let keyword = query.query.unwrap();

        let (entries, count) = schema.search_accounts(&keyword, offset, query.limit)?;

        Ok(EntriesResult { count, entries })
    }

    /// Mendapatkan jumlah akun secara keseluruhan
    #[api_endpoint(path = "/account/count")]
    pub fn account_count(state: &AppState, query: ()) -> SuccessReturn<i64> {
        let schema = Schema::new(state.db());

        schema
            .get_account_count()
            .map(SuccessReturn::new)
            .map_err(From::from)
    }

    /// Mendapatkan jumlah akun secara keseluruhan
    #[api_endpoint(path = "/account/info", auth = "required")]
    pub fn account_info(query: AccountQuery) -> SuccessReturn<db::Account> {
        let schema = Schema::new(state.db());

        schema
            .get_account(query.id)
            .map(SuccessReturn::new)
            .map_err(From::from)
    }

    #[api_endpoint(path = "/transactions", auth = "required", immutable)]
    pub fn transactions(query: QueryEntries) -> EntriesResult<db::Transaction> {
        let schema = Schema::new(state.db());

        let entries = schema.get_transactions(query.page, query.limit)?;

        Ok(EntriesResult { count: 0, entries })
    }
}
