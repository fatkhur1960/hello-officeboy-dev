//! Schema operation a.k.a DAO spesifik untuk transaksi
//! digunakan untuk melakukan operasi seperti
//! transfer, credit, debit, invoicing.

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use failure;

use crate::{
    error::Error as PaymentError,
    error::ErrorCode,
    models::{Account, Invoice, InvoiceItem},
    result::Result,
    schema::{invoice_items, invoices, payment_history, transaction_histories},
    util,
};

use std::sync::Arc;

use crate::schema_op::ID;

/// Debit credit flag
pub enum DbcrFlag {
    /// Pendiebitan
    Debit = 1,
    /// Pengkreditan
    Credit = 2,
}

/// Tipe dari transaksi
pub enum TxType {
    /// Untuk topup
    Topup = 1,
    /// Untuk payment
    Payment = 2,
    /// Untuk recharge
    Recharge = 3,
    /// Untuk transfer
    Transfer = 4,
}

/// Status transaksi
pub enum TxStatus {
    /// Apabila sukses
    Success = 0,
    /// Untuk sedang dalam proses
    InProgress,
    /// Apabila operasi timeout
    Timeout,
    /// Apabila terjadi error yang generic
    GenericError,
    /// Apabila gagal melakukan kontak ke remote server
    CannotContactRemote,
    /// Apabila akun di remote server tidak valid atau tidak exists
    InvalidRemoteAccount,
    /// Apabila akun di lokal kita tidak diijinkan di remote
    InvalidLocalAccount,
    /// Apabila balance tidak mencukupi
    InsufficientBalance,
}

#[derive(Insertable)]
#[table_name = "invoices"]
#[doc(hidden)]
pub struct NewInvoice<'a> {
    pub id_ref: &'a str,
    pub issuer_account: ID,
    pub to_account: ID,
    pub discount: f64,
    pub amount: f64,
    pub notes: &'a str,
}

#[derive(Insertable)]
#[table_name = "invoice_items"]
#[doc(hidden)]
pub struct NewInvoiceItem<'a> {
    pub invoice_id: ID,
    pub name: &'a str,
    pub price: f64,
}

#[derive(Insertable)]
#[table_name = "transaction_histories"]
#[doc(hidden)]
pub struct NewTransactionHistory<'a> {
    pub dbcr_flag: i32,
    pub ttype: i32,
    // pub subttype: i32,
    pub amount: f64,
    pub status: i32,
    pub created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub invoice_id: Option<ID>,
    pub from_account_id: Option<ID>,
    pub to_account_id: Option<ID>,
    pub merchant_id: Option<ID>,
    pub notes: Option<&'a str>,
}

impl<'a> NewInvoiceItem<'a> {
    /// Set invoice ID dimana item ini berada
    pub fn set_invoice_id(&self, invoice_id: ID) -> Self {
        NewInvoiceItem {
            invoice_id,
            name: self.name,
            price: self.price,
        }
    }
}

#[derive(Insertable)]
#[table_name = "payment_history"]
#[doc(hidden)]
pub struct NewPaymentHistory<'a> {
    pub invoice_id: ID,
    pub payer: ID,
    pub via: &'a str,
}

/// Untuk mengoperasikan skema data di database
pub struct Schema<'a> {
    db: &'a PgConnection,
}

impl<'a> Schema<'a> {
    /// Create new schema instance.
    pub fn new(db: &'a PgConnection) -> Self {
        Self { db }
    }

    /// Meng-kredit akun sejumlah uang ke sebuah akun.
    /// Mengembalikan ID dari transaction histories `TransactionHistory`.
    pub fn credit(&self, account: &Account, amount: f64) -> Result<ID> {
        if amount < 0.0f64 || amount > 3_000_000f64 {
            Err(PaymentError::InvalidParameter("Invalid amount".to_string()))?
        }
        if !account.active {
            Err(PaymentError::BadRequest(
                ErrorCode::TxAccountInactive as i32,
                "Account inactive".to_string(),
            ))?
        }

        self.db.build_transaction().read_write().run(|| {
            {
                use crate::schema::accounts::{self, dsl};
                diesel::update(dsl::accounts.filter(dsl::id.eq(account.id)))
                    .set(dsl::balance.eq(dsl::balance + amount))
                    .execute(self.db)?;
            }
            {
                use crate::schema::transaction_histories::{self, dsl};
                diesel::insert_into(transaction_histories::table)
                    .values(&NewTransactionHistory {
                        dbcr_flag: DbcrFlag::Credit as i32,
                        ttype: TxType::Topup as i32,
                        amount,
                        status: TxStatus::Success as i32,
                        created: util::now(),
                        last_updated: util::now(),
                        invoice_id: None,
                        from_account_id: None,
                        to_account_id: Some(account.id),
                        merchant_id: None,
                        notes: None,
                    })
                    .returning(dsl::id)
                    .get_result(self.db)
                    .map_err(From::from)
            }
        })
    }

    /// Publish invoice
    ///
    /// # Arguments
    ///
    /// * `id_ref` merupakan id yang ada dari sisi external misal client/merchant.
    /// * `issuer_account` akun yang menerbitkan invoice.
    /// * `to_account` akun yang dituju untuk melakukan pembayaran
    pub fn publish_invoice(&self, new_invoice: NewInvoice, items: Vec<NewInvoiceItem>) -> Result<ID> {
        use crate::schema::invoice_items;
        use crate::schema::invoice_items::dsl as item_dsl;
        use crate::schema::invoices;
        use crate::schema::invoices::dsl;

        if new_invoice.issuer_account == new_invoice.to_account {
            Err(PaymentError::BadRequest(
                ErrorCode::FromAndToTargetIsSame as i32,
                "Invalid parameter".to_string(),
            ))?;
        }

        if new_invoice.amount <= 0.0 || new_invoice.amount > 3_000_000f64 {
            Err(PaymentError::BadRequest(
                ErrorCode::TxBadAmount as i32,
                "Invalid amount".to_string(),
            ))?;
        }

        if new_invoice.discount <= 0.0 {
            Err(PaymentError::BadRequest(
                ErrorCode::TxBadInvoiceDiscount as i32,
                "Invalid discount".to_string(),
            ))?;
        }

        for item in &items {
            if item.price < 0.0 || item.name.trim().is_empty() {
                Err(PaymentError::BadRequest(
                    ErrorCode::TxBadInvoiceItemData as i32,
                    "Invalid data".to_string(),
                ))?;
            }
        }

        self.db.build_transaction().read_write().run(|| {
            let id: ID = diesel::insert_into(invoices::table)
                .values(&new_invoice)
                .returning(dsl::id)
                .get_result(self.db)?;
            // .map_err(From::from)?;

            let items: Vec<NewInvoiceItem> = items.iter().map(|item| item.set_invoice_id(id)).collect();

            diesel::insert_into(invoice_items::table)
                .values(&items)
                .execute(self.db)?;

            Ok(id)
        })
    }

    /// Bayar invoice. Operasi ini akan melakukan:
    ///
    /// * Pendebetan pada saldo pembayar.
    /// * Menandai sebuah invoice sebagai telah terbayar.
    /// * Penambahan saldo pada penerbit invoice (penjual).
    /// * Mencatat history pembayaran.
    ///
    /// Apabila sukses/berhasil akan mengembalikan ID dari transaksinya.
    pub fn pay_invoice(&self, id: ID, payer: &Account, amount: f64, via: &str) -> Result<ID> {
        use crate::schema::accounts;
        use crate::schema::accounts::dsl as dsl_account;
        use crate::schema::invoices;
        use crate::schema::invoices::dsl;
        use crate::schema::payment_history;
        use crate::schema::payment_history::dsl as dsl_history;

        // check ketersediaan balance
        if payer.balance < amount {
            Err(PaymentError::Insufficient("Insufficient balance"))?
        }

        let invoice = self.get_invoice(id)?;

        // amount harus sama
        // if amount != invoice.amount { // tidak menggunakan strict unequality comparison pada floating point
        // kita gunakan epsilon dengan margin error
        if (amount - invoice.amount).abs() < 0.001 {
            Err(PaymentError::BadRequest(
                ErrorCode::TxAmountMismatch as i32,
                "Mismatch amount".to_owned(),
            ))?
        }

        if invoice.paid {
            // sudah terbayar
            Err(PaymentError::AlreadyExists)?
        }

        self.db.build_transaction().read_write().run(move || {
            // debit saldo pembayar
            diesel::update(dsl_account::accounts.filter(dsl_account::id.eq(payer.id)))
                .set(dsl_account::balance.eq(dsl_account::balance - amount))
                .execute(self.db)?;

            // tandai invoice telah terbayar
            diesel::update(dsl::invoices.filter(dsl::id.eq(id)))
                .set((
                    dsl::paid.eq(true),
                    dsl::paid_by.eq(payer.id),
                    dsl::paid_at.eq(Some(Utc::now().naive_utc())),
                ))
                .execute(self.db)?;

            // credit saldo issuer
            diesel::update(dsl_account::accounts.filter(dsl_account::id.eq(invoice.issuer_account)))
                .set(dsl_account::balance.eq(dsl_account::balance + amount))
                .execute(self.db)?;

            // catat history pembayarannya
            // @TODO(robin): check ini payment_history sepertinya jadi redundant
            // dengan transaction_histories
            diesel::insert_into(payment_history::table)
                .values(NewPaymentHistory {
                    invoice_id: id,
                    payer: payer.id,
                    via,
                })
                // .returning(dsl_history::id)
                .execute(self.db)?;
            // .map_err(From::from)?;

            // Catat history transaksi local
            {
                use crate::schema::transaction_histories::{self, dsl};
                diesel::insert_into(transaction_histories::table)
                    .values(&NewTransactionHistory {
                        dbcr_flag: DbcrFlag::Debit as i32,
                        ttype: TxType::Payment as i32,
                        amount,
                        status: TxStatus::InProgress as i32,
                        created: util::now(),
                        last_updated: util::now(),
                        invoice_id: None,
                        from_account_id: None,
                        to_account_id: Some(invoice.issuer_account),
                        merchant_id: None,
                        notes: None,
                    })
                    .returning(dsl::id)
                    .get_result(self.db)
                    .map_err(From::from)
            }
        })
    }

    /// Mendapatkan invoice berdasarkan ID-nya.
    pub fn get_invoice(&self, id: ID) -> Result<Invoice> {
        assert!(id > 0);
        use crate::schema::invoices::{self, dsl};

        dsl::invoices.find(id).get_result(self.db).map_err(From::from)
    }
}
