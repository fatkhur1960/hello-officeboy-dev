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
    schema::{invoice_items, invoices, payment_history},
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
        use crate::schema::accounts::{self, dsl};

        diesel::update(dsl::accounts.filter(dsl::id.eq(account.id)))
            .set(dsl::balance.eq(dsl::balance + amount))
            .execute(self.db)?;

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
