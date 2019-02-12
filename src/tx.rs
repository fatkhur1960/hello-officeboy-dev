//! Schema operation a.k.a DAO spesifik untuk transaksi
//! digunakan untuk melakukan operasi seperti
//! transfer, credit, debit, invoicing.

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use failure;

use crate::{
    error::Error as PaymentError,
    models::{Account, Invoice, InvoiceItem},
    result::Result,
    schema::{invoice_items, invoices, payment_history, transactions},
    util,
};

use std::sync::Arc;

use crate::schema_op::ID;

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
#[table_name = "transactions"]
#[doc(hidden)]
pub struct NewTransaction<'a> {
    pub business_cycle: ID,
    pub stan: i64,
    pub dbcr_flag: i32,
    pub ttype: i32,
    pub subttype: i32,
    pub amount: f64,
    pub status: i32,
    pub created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub invoice: Option<&'a str>,
    pub from_wallet: Option<ID>,
    pub to_wallet: Option<ID>,
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

    /// Meng-kredit akun sejumlah uang
    pub fn credit(&self, account: &Account, amount: f64) -> Result<()> {
        // @TODO(hanky): fix this history transaction logic
        {
            use crate::schema::accounts::{self, dsl};
            diesel::update(dsl::accounts.filter(dsl::id.eq(account.id)))
                .set(dsl::balance.eq(dsl::balance + amount))
                .execute(self.db)?;
        }
        {
            use crate::schema::transactions::{self, dsl};
            diesel::insert_into(transactions::table)
                .values(&NewTransaction {
                    business_cycle: 1,
                    stan: 1, // @TODO(*): fix this
                    dbcr_flag: 2,
                    ttype: 1,
                    subttype: 1,
                    amount,
                    status: 0,
                    created: util::now(),
                    last_updated: util::now(),
                    invoice: None,
                    from_wallet: None,
                    to_wallet: Some(account.id),
                    merchant_id: None,
                    notes: None,
                })
                .execute(self.db)?;
        }
        Ok(())
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
    /// Apabila sukses/berhasil akan mengembalikan ID dari history payment-nya.
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
            Err(PaymentError::BadRequest("Mismatch amount".to_owned()))?
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
            diesel::insert_into(payment_history::table)
                .values(NewPaymentHistory {
                    invoice_id: id,
                    payer: payer.id,
                    via,
                })
                .returning(dsl_history::id)
                .get_result(self.db)
                .map_err(From::from)
        })
    }

    /// Mendapatkan invoice berdasarkan ID-nya.
    pub fn get_invoice(&self, id: ID) -> Result<Invoice> {
        assert!(id > 0);
        use crate::schema::invoices::{self, dsl};

        dsl::invoices.find(id).get_result(self.db).map_err(From::from)
    }
}
